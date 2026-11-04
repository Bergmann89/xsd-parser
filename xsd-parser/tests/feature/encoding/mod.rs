use xsd_parser::{Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generate([(IdentType::Element, "tns:Greeting")])
        .with_quick_xml()
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/encoding/schema.xsd",
        "tests/feature/encoding/expected/quick_xml.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod read_write {
    use xsd_parser_types::quick_xml::{
        DecodingIoReader, DeserializeSync, Encoding, EncodingWriter,
    };

    use super::quick_xml::Greeting;

    const XML: &str = "<tns:Greeting xmlns:tns=\"http://example.com\" lang=\"ру\"><tns:Text>Привет</tns:Text></tns:Greeting>";

    fn windows_1251() -> &'static Encoding {
        Encoding::for_label(b"windows-1251").unwrap()
    }

    fn document() -> Vec<u8> {
        let xml = format!("<?xml version=\"1.0\" encoding=\"windows-1251\"?>{XML}");
        let (bytes, _, unmappable) = windows_1251().encode(&xml);
        assert!(!unmappable);

        bytes.into_owned()
    }

    #[test]
    fn read_windows_1251() {
        let document = document();
        let value = Greeting::deserialize(&mut DecodingIoReader::new(&document[..])).unwrap();

        assert_eq!(value.lang.as_deref(), Some("ру"));
        assert_eq!(value.text, "Привет");
    }

    #[test]
    fn write_windows_1251() {
        let value = Greeting {
            lang: Some("ру".into()),
            text: "Привет".into(),
        };

        let actual = EncodingWriter::new(Vec::new(), windows_1251())
            .unwrap()
            .write_document(&value, "tns:Greeting")
            .unwrap();

        assert_eq!(actual, document());
    }
}
