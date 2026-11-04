use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::io::BufRead;

use encoding_rs::Encoding;
use quick_xml::{encoding::DecodingReader, events::Event, NsReader, Reader};

use super::super::{Error, XmlReader, XmlReaderSync};

/// Implements an [`XmlReader`] for any kind of [`BufRead`] that decodes the
/// input to UTF-8.
///
/// The encoding is detected from the byte order mark or from the `encoding`
/// attribute of the XML declaration, so any encoding supported by
/// [`encoding_rs`] can be read (e.g. `UTF-16` or `Windows-1251`). Without
/// both the input is read as UTF-8. The XML declaration has to fit into the
/// buffer of the passed reader (8 KiB for a default [`BufReader`](std::io::BufReader)).
///
/// Use the [`IoReader`](super::IoReader) instead if the input is always UTF-8,
/// it does not need to transcode the input. There is no async version of this
/// reader. To read other encodings asynchronously, decode the document to
/// UTF-8 first and use the [`SliceReader`](super::SliceReader).
pub struct DecodingIoReader<R> {
    inner: NsReader<DecodingReader<R>>,
    buffer: Vec<u8>,
}

impl<R: BufRead> DecodingIoReader<R> {
    /// Create a new [`DecodingIoReader`] instance from the passed `reader`.
    pub fn new(mut reader: R) -> Self {
        // The `DecodingReader` decodes the first bytes before the parser has
        // seen the XML declaration, so the encoding is taken from the buffered
        // input up front. The BOM and UTF-16 are detected by the `DecodingReader`.
        let encoding = reader.fill_buf().ok().and_then(declared_encoding);

        let mut reader = DecodingReader::new(reader);
        if let Some(encoding) = encoding {
            reader.set_encoding(encoding);
        }

        let inner = NsReader::from_reader(reader);
        let buffer = Vec::new();

        Self { inner, buffer }
    }
}

impl<R> XmlReader for DecodingIoReader<R> {
    fn extend_error(&self, error: Error) -> Error {
        error.with_pos(self.inner.buffer_position())
    }
}

impl<R> XmlReaderSync<'static> for DecodingIoReader<R>
where
    R: BufRead,
{
    fn read_event(&mut self) -> Result<Event<'static>, Error> {
        self.buffer.clear();

        self.inner
            .read_event_into(&mut self.buffer)
            .map(Event::into_owned)
            .map_err(Error::from)
    }
}

/// Returns the encoding of the XML declaration at the start of `bytes`.
fn declared_encoding(bytes: &[u8]) -> Option<&'static Encoding> {
    let mut reader = Reader::from_reader(bytes);

    match reader.read_event_into(&mut Vec::new()) {
        Ok(Event::Decl(decl)) => decl.encoder(),
        _ => None,
    }
}

impl<R> Debug for DecodingIoReader<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "DecodingIoReader")
    }
}

#[cfg(test)]
mod tests {
    use quick_xml::events::Event;

    use super::{DecodingIoReader, XmlReaderSync};

    /// "Привет" encoded as `Windows-1251`.
    const PRIVET_WINDOWS_1251: &[u8] = &[0xCF, 0xF0, 0xE8, 0xE2, 0xE5, 0xF2];

    fn read_text(input: &[u8]) -> String {
        let mut reader = DecodingIoReader::new(input);

        loop {
            match reader.read_event().unwrap() {
                Event::Text(text) => return text.as_ref().to_owned(),
                Event::Eof => panic!("No text event found"),
                _ => (),
            }
        }
    }

    #[test]
    fn reads_encoding_from_declaration() {
        let mut input = b"<?xml version=\"1.0\" encoding=\"Windows-1251\"?><a>".to_vec();
        input.extend_from_slice(PRIVET_WINDOWS_1251);
        input.extend_from_slice(b"</a>");

        assert_eq!(read_text(&input), "Привет");
    }

    #[test]
    fn reads_encoding_from_long_declaration() {
        let mut input =
            b"<?xml version=\"1.0\" encoding=\"Windows-1251\" standalone=\"yes\"   ?><a>".to_vec();
        input.extend_from_slice(PRIVET_WINDOWS_1251);
        input.extend_from_slice(b"</a>");

        assert_eq!(read_text(&input), "Привет");
    }

    #[test]
    fn reads_utf16_with_bom() {
        let mut input = vec![0xFF, 0xFE];
        for unit in "<?xml version=\"1.0\" encoding=\"UTF-16\"?><a>Привет</a>".encode_utf16()
        {
            input.extend_from_slice(&unit.to_le_bytes());
        }

        assert_eq!(read_text(&input), "Привет");
    }

    #[test]
    fn reads_utf8_without_declaration() {
        assert_eq!(read_text("<a>Привет</a>".as_bytes()), "Привет");
    }
}
