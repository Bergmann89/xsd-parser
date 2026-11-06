use xsd_parser::{Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/eof_in_wildcard_content/schema.xsd",
        "tests/feature/eof_in_wildcard_content/expected/quick_xml.rs",
        Config::test_default()
            .with_quick_xml_deserialize()
            .with_generate([(IdentType::Element, "tns:Root")]),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    include!("expected/quick_xml.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod read {
    use std::mem::replace;

    use xsd_parser_types::quick_xml::{
        DeserializeSync, Error, ErrorKind, Event, SliceReader, XmlReader, XmlReaderSync,
    };

    use super::quick_xml::Root;

    /// Panics on a second `Eof`, so a hang fails the test instead.
    struct EofOnceReader<'a> {
        inner: SliceReader<'a>,
        eof_seen: bool,
    }

    impl XmlReader for EofOnceReader<'_> {
        fn extend_error(&self, error: Error) -> Error {
            self.inner.extend_error(error)
        }
    }

    impl<'a> XmlReaderSync<'a> for EofOnceReader<'a> {
        fn read_event(&mut self) -> Result<Event<'a>, Error> {
            let event = self.inner.read_event()?;
            if matches!(event, Event::Eof) {
                assert!(!replace(&mut self.eof_seen, true), "read past the end");
            }

            Ok(event)
        }
    }

    #[test]
    fn eof_in_wildcard_content() {
        let error = Root::deserialize(&mut EofOnceReader {
            inner: SliceReader::new("<tns:Root xmlns:tns=\"http://example.com\"><tns:Info>"),
            eof_seen: false,
        })
        .unwrap_err();

        assert!(
            matches!(error.kind, ErrorKind::UnexpectedEvent(Event::Eof)),
            "unexpected error: {error:?}"
        );
    }
}
