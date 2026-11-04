use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::io::{Error, ErrorKind, Result, Write};
use std::mem::take;
use std::str::from_utf8;

use encoding_rs::{Encoder, EncoderResult, Encoding};
use quick_xml::events::{BytesDecl, Event};

use super::{Error as XmlError, SerializeSync, Writer};

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
/// Characters that can not be represented in the target encoding result in an
/// error, they are not replaced. Encodings that `encoding_rs` can not encode
/// (`UTF-16` and `replacement`) are rejected by [`EncodingWriter::new`].
pub struct EncodingWriter<W> {
    inner: W,
    encoder: Encoder,
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
        let encoding = self.encoder.encoding().name();

        let mut writer = Writer::new(self);
        writer
            .write_event(Event::Decl(BytesDecl::new("1.0", Some(encoding), None)))
            .map_err(XmlError::from)?;
        value.serialize(root, &mut writer)?;

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

    fn encode(&mut self, mut input: &str, last: bool) -> Result<()> {
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

fn invalid_utf8() -> Error {
    Error::new(ErrorKind::InvalidData, "Written data is not valid UTF-8")
}

#[cfg(test)]
mod tests {
    use std::io::{ErrorKind, Write};

    use encoding_rs::{UTF_16LE, UTF_8, WINDOWS_1251};

    use super::super::ErrorKind as ErrorKind2;
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

    #[test]
    fn rejects_unmappable_characters() {
        let mut writer = EncodingWriter::new(Vec::new(), WINDOWS_1251).unwrap();
        let error = writer.write_all("<a>😀</a>".as_bytes()).unwrap_err();

        assert_eq!(error.kind(), ErrorKind::InvalidData);
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
    fn write_document_fails_for_unmappable_characters() {
        let error = EncodingWriter::new(Vec::new(), WINDOWS_1251)
            .unwrap()
            .write_document(&String::from("😀"), "a")
            .unwrap_err();

        assert!(
            matches!(&error.kind, ErrorKind2::XmlError(quick_xml::Error::Io(x)) if x.kind() == ErrorKind::InvalidData),
            "{error:?}"
        );
    }

    #[test]
    fn rejects_utf16() {
        let error = EncodingWriter::new(Vec::new(), UTF_16LE).unwrap_err();

        assert_eq!(error.kind(), ErrorKind::InvalidInput);
    }
}
