use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::io::{Error, ErrorKind, Result, Write};
use std::mem::take;
use std::str::from_utf8;

use encoding_rs::{CoderResult, Encoder, EncoderResult, Encoding};
use quick_xml::events::{BytesDecl, Event};

use super::{Error as XmlError, SerializeHelper, SerializeSync, Writer};

/// Implements [`Write`] for any other [`Write`] and encodes the written UTF-8
/// data using the passed [`Encoding`].
///
/// The serializers of this crate always produce UTF-8. Use this writer to
/// write other encodings (e.g. `Windows-1251`). [`write_document`](Self::write_document)
/// writes a complete document including the matching XML declaration:
///
/// ```rust
/// use xsd_parser_types::quick_xml::{Encoding, EncodingWriter};
///
/// let encoding = Encoding::for_label(b"windows-1251").unwrap();
/// let value = String::from("Привет");
///
/// let bytes = EncodingWriter::new(Vec::new(), encoding)
///     .unwrap()
///     .write_document(&value, "Greeting")
///     .unwrap();
///
/// let (text, _, _) = encoding.decode(&bytes);
/// assert_eq!(text, "<?xml version=\"1.0\" encoding=\"windows-1251\"?><Greeting>Привет</Greeting>");
/// ```
///
/// To write other content, use it as target of a [`Writer`] and call
/// [`finish`](Self::finish) after the last write.
///
/// Characters that can not be represented in the target encoding are written
/// as numeric character references (e.g. `&#128512;`) in text and attribute
/// values. In names, comments, CDATA sections and processing instructions a
/// reference would change the content, so they result in an error there.
/// Encodings that `encoding_rs` can not encode (`UTF-16` and `replacement`)
/// are rejected by [`EncodingWriter::new`].
pub struct EncodingWriter<W> {
    inner: W,
    encoder: Encoder,
    state: State,
    pending: Vec<u8>,
    buffer: Vec<u8>,
}

impl<W: Write> EncodingWriter<W> {
    /// Create a new [`EncodingWriter`] instance that writes to `inner` using
    /// the passed `encoding`.
    ///
    /// # Errors
    ///
    /// Returns an error of kind [`ErrorKind::InvalidInput`] if the passed
    /// `encoding` can not be used for encoding.
    pub fn new(inner: W, encoding: &'static Encoding) -> Result<Self> {
        if encoding.output_encoding() != encoding {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Encoding {} is not supported for writing", encoding.name()),
            ));
        }

        Ok(Self {
            inner,
            encoder: encoding.new_encoder(),
            state: State::Text,
            pending: Vec::new(),
            buffer: vec![0; 4096],
        })
    }

    /// Write `value` as complete XML document, starting with an XML declaration
    /// that names the encoding of this writer, and return the underlying writer.
    ///
    /// # Errors
    ///
    /// Returns an error if the value could not be serialized, contains
    /// characters that can not be encoded, or if the underlying writer fails.
    pub fn write_document<T>(self, value: &T, root: &str) -> std::result::Result<W, T::Error>
    where
        T: SerializeSync,
        T::Error: From<XmlError>,
    {
        self.write_document_with(value, root, SerializeHelper::default())
    }

    /// Like [`write_document`](Self::write_document), but uses the passed
    /// `helper`, e.g. to set a default namespace.
    ///
    /// # Errors
    ///
    /// Returns an error if the value could not be serialized, contains
    /// characters that can not be encoded, or if the underlying writer fails.
    pub fn write_document_with<T>(
        self,
        value: &T,
        root: &str,
        helper: SerializeHelper,
    ) -> std::result::Result<W, T::Error>
    where
        T: SerializeSync,
        T::Error: From<XmlError>,
    {
        let encoding = self.encoder.encoding().name();

        let mut writer = Writer::new(self);
        writer
            .write_event(Event::Decl(BytesDecl::new("1.0", Some(encoding), None)))
            .map_err(XmlError::from)?;
        value.serialize_with(root, &mut writer, helper)?;

        let inner = writer.into_inner().finish().map_err(XmlError::from)?;

        Ok(inner)
    }

    /// Get a reference to the underlying writer.
    pub fn get_ref(&self) -> &W {
        &self.inner
    }

    /// Finish the encoding and return the underlying writer.
    ///
    /// This must be called after the last write, otherwise stateful encodings
    /// (like `ISO-2022-JP`) may miss their final bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the written data ends with an incomplete UTF-8
    /// sequence or if the underlying writer fails.
    pub fn finish(mut self) -> Result<W> {
        if !self.pending.is_empty() {
            return Err(invalid_utf8());
        }

        self.encode("", true)?;
        self.inner.flush()?;

        Ok(self.inner)
    }

    fn encode(&mut self, input: &str, last: bool) -> Result<()> {
        // Splits the input into runs that either allow character references
        // or not. The state only changes on ASCII bytes, so every split is at
        // a character boundary.
        let mut start = 0;
        let mut escape = self.state.allows_char_refs();

        for (pos, byte) in input.bytes().enumerate() {
            self.state = self.state.next(byte);

            let next = self.state.allows_char_refs();
            if next != escape {
                self.encode_run(&input[start..=pos], escape, false)?;
                start = pos + 1;
                escape = next;
            }
        }

        self.encode_run(&input[start..], escape, last)
    }

    fn encode_run(&mut self, mut input: &str, escape: bool, last: bool) -> Result<()> {
        if escape {
            loop {
                let (result, read, written, _) =
                    self.encoder.encode_from_utf8(input, &mut self.buffer, last);
                self.inner.write_all(&self.buffer[..written])?;
                input = &input[read..];

                if result == CoderResult::InputEmpty {
                    return Ok(());
                }
            }
        }

        loop {
            let (result, read, written) =
                self.encoder
                    .encode_from_utf8_without_replacement(input, &mut self.buffer, last);
            self.inner.write_all(&self.buffer[..written])?;
            input = &input[read..];

            match result {
                EncoderResult::InputEmpty => return Ok(()),
                EncoderResult::OutputFull => (),
                EncoderResult::Unmappable(c) => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "Character {c:?} (U+{:04X}) can not be encoded using {}",
                            u32::from(c),
                            self.encoder.encoding().name()
                        ),
                    ));
                }
            }
        }
    }
}

impl<W: Write> Write for EncodingWriter<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if self.pending.is_empty() {
            if let Ok(s) = from_utf8(buf) {
                self.encode(s, false)?;

                return Ok(buf.len());
            }
        }

        // A multi-byte UTF-8 sequence may be split across two calls, the
        // incomplete end is kept until the next call.
        let mut data = take(&mut self.pending);
        data.extend_from_slice(buf);

        let valid = match from_utf8(&data) {
            Ok(s) => s.len(),
            Err(error) if error.error_len().is_none() => error.valid_up_to(),
            Err(_) => return Err(invalid_utf8()),
        };

        let (complete, rest) = data.split_at(valid);
        self.encode(from_utf8(complete).map_err(|_| invalid_utf8())?, false)?;
        self.pending = rest.to_vec();

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}

impl<W> Debug for EncodingWriter<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "EncodingWriter({})", self.encoder.encoding().name())
    }
}

/// Tracks where in the XML output the writer currently is, to decide where
/// characters may be written as character references.
///
/// The output of the [`Writer`] is always well-formed, so this only needs to
/// find the start and the end of the different kinds of markup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    /// Text content between tags.
    Text,

    /// Directly after a `<`.
    Open,

    /// Inside a start or end tag, optionally inside a quoted attribute value.
    Tag { quote: Option<u8> },

    /// Directly after a `<!`.
    Bang,

    /// Inside a comment, counting the trailing `-`.
    Comment { dashes: u8 },

    /// Inside a CDATA section, counting the trailing `]`.
    CData { brackets: u8 },

    /// Inside a processing instruction or the XML declaration.
    Pi { question: bool },

    /// Inside a document type declaration, tracking the internal subset.
    DocType { depth: usize },
}

impl State {
    fn allows_char_refs(self) -> bool {
        matches!(self, Self::Text | Self::Tag { quote: Some(_) })
    }

    fn next(self, byte: u8) -> Self {
        match (self, byte) {
            (Self::Text, b'<') => Self::Open,
            (Self::Text, _) => Self::Text,

            (Self::Open, b'!') => Self::Bang,
            (Self::Open, b'?') => Self::Pi { question: false },
            (Self::Open, _) => Self::Tag { quote: None },

            (Self::Tag { quote: None }, b'>') => Self::Text,
            (Self::Tag { quote: None }, b'"' | b'\'') => Self::Tag { quote: Some(byte) },
            (Self::Tag { quote: Some(quote) }, _) if quote == byte => Self::Tag { quote: None },
            (Self::Tag { .. }, _) => self,

            (Self::Bang, b'-') => Self::Comment { dashes: 0 },
            (Self::Bang, b'[') => Self::CData { brackets: 0 },
            (Self::Bang, _) => Self::DocType { depth: 0 },

            (Self::Comment { dashes }, b'>') if dashes >= 2 => Self::Text,
            (Self::Comment { dashes }, b'-') => Self::Comment {
                dashes: dashes.saturating_add(1),
            },
            (Self::Comment { .. }, _) => Self::Comment { dashes: 0 },

            (Self::CData { brackets }, b'>') if brackets >= 2 => Self::Text,
            (Self::CData { brackets }, b']') => Self::CData {
                brackets: brackets.saturating_add(1),
            },
            (Self::CData { .. }, _) => Self::CData { brackets: 0 },

            (Self::Pi { question: true }, b'>') => Self::Text,
            (Self::Pi { .. }, _) => Self::Pi {
                question: byte == b'?',
            },

            (Self::DocType { depth: 0 }, b'>') => Self::Text,
            (Self::DocType { depth }, b'[') => Self::DocType { depth: depth + 1 },
            (Self::DocType { depth }, b']') => Self::DocType {
                depth: depth.saturating_sub(1),
            },
            (Self::DocType { .. }, _) => self,
        }
    }
}

fn invalid_utf8() -> Error {
    Error::new(ErrorKind::InvalidData, "Written data is not valid UTF-8")
}

#[cfg(test)]
mod tests {
    use std::io::{ErrorKind, Write};

    use encoding_rs::{UTF_16LE, UTF_8, WINDOWS_1251};

    use super::EncodingWriter;

    #[test]
    fn encodes_windows_1251() {
        let mut writer = EncodingWriter::new(Vec::new(), WINDOWS_1251).unwrap();
        writer.write_all("<a>Привет</a>".as_bytes()).unwrap();

        let mut expected = b"<a>".to_vec();
        expected.extend_from_slice(&[0xCF, 0xF0, 0xE8, 0xE2, 0xE5, 0xF2]);
        expected.extend_from_slice(b"</a>");

        assert_eq!(writer.finish().unwrap(), expected);
    }

    #[test]
    fn keeps_utf8_sequences_split_across_writes() {
        let bytes = "Привет".as_bytes();
        let mut writer = EncodingWriter::new(Vec::new(), WINDOWS_1251).unwrap();
        for byte in bytes {
            writer.write_all(&[*byte]).unwrap();
        }

        assert_eq!(
            writer.finish().unwrap(),
            [0xCF, 0xF0, 0xE8, 0xE2, 0xE5, 0xF2]
        );
    }

    #[test]
    fn writes_utf8_unchanged() {
        let mut writer = EncodingWriter::new(Vec::new(), UTF_8).unwrap();
        writer.write_all("<a>Привет 😀</a>".as_bytes()).unwrap();

        assert_eq!(writer.finish().unwrap(), "<a>Привет 😀</a>".as_bytes());
    }

    fn encode(input: &str) -> std::io::Result<Vec<u8>> {
        let mut writer = EncodingWriter::new(Vec::new(), WINDOWS_1251).unwrap();
        writer.write_all(input.as_bytes())?;
        writer.finish()
    }

    fn encode_bytewise(input: &str) -> std::io::Result<Vec<u8>> {
        let mut writer = EncodingWriter::new(Vec::new(), WINDOWS_1251).unwrap();
        for byte in input.as_bytes() {
            writer.write_all(&[*byte])?;
        }
        writer.finish()
    }

    #[test]
    fn writes_char_refs_in_text_and_attributes() {
        let input = "<?xml version=\"1.0\"?><!DOCTYPE a [<!ENTITY e \"x\">]><a b=\"😀\" c='😀'><!-- - --><![CDATA[]]]>😀<?pi ?>😀</a>";
        let expected = "<?xml version=\"1.0\"?><!DOCTYPE a [<!ENTITY e \"x\">]><a b=\"&#128512;\" c='&#128512;'><!-- - --><![CDATA[]]]>&#128512;<?pi ?>&#128512;</a>";

        assert_eq!(encode(input).unwrap(), expected.as_bytes());
        assert_eq!(encode_bytewise(input).unwrap(), expected.as_bytes());
    }

    #[test]
    fn rejects_unmappable_characters_outside_of_text_and_attributes() {
        for input in [
            "<😀/>",
            "<a 😀=\"x\"/>",
            "<a></😀>",
            "<!--😀-->",
            "<!-- -->x<!--😀-->",
            "<![CDATA[😀]]>",
            "<![CDATA[]]]>x<![CDATA[]>😀]]>",
            "<?pi 😀?>",
            "<!DOCTYPE a [<!ENTITY e \"😀\">]>",
        ] {
            for result in [encode(input), encode_bytewise(input)] {
                assert_eq!(
                    result.unwrap_err().kind(),
                    ErrorKind::InvalidData,
                    "{input}"
                );
            }
        }
    }

    #[test]
    fn rejects_incomplete_utf8_at_the_end() {
        let mut writer = EncodingWriter::new(Vec::new(), WINDOWS_1251).unwrap();
        writer.write_all(&"П".as_bytes()[..1]).unwrap();

        assert_eq!(writer.finish().unwrap_err().kind(), ErrorKind::InvalidData);
    }

    #[test]
    fn write_document_writes_declaration_and_value() {
        let bytes = EncodingWriter::new(Vec::new(), WINDOWS_1251)
            .unwrap()
            .write_document(&String::from("Привет"), "a")
            .unwrap();

        let mut expected = b"<?xml version=\"1.0\" encoding=\"windows-1251\"?><a>".to_vec();
        expected.extend_from_slice(&[0xCF, 0xF0, 0xE8, 0xE2, 0xE5, 0xF2]);
        expected.extend_from_slice(b"</a>");

        assert_eq!(bytes, expected);
    }

    #[test]
    fn write_document_writes_char_refs_for_unmappable_characters() {
        let bytes = EncodingWriter::new(Vec::new(), WINDOWS_1251)
            .unwrap()
            .write_document(&String::from("Привет 😀"), "a")
            .unwrap();

        let mut expected = b"<?xml version=\"1.0\" encoding=\"windows-1251\"?><a>".to_vec();
        expected.extend_from_slice(&[0xCF, 0xF0, 0xE8, 0xE2, 0xE5, 0xF2]);
        expected.extend_from_slice(b" &#128512;</a>");

        assert_eq!(bytes, expected);
    }

    #[test]
    fn rejects_utf16() {
        let error = EncodingWriter::new(Vec::new(), UTF_16LE).unwrap_err();

        assert_eq!(error.kind(), ErrorKind::InvalidInput);
    }
}
