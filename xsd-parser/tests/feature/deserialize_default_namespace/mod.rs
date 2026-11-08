use xsd_parser::{pipeline::renderer::NamespaceSerialization, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generate([(IdentType::Element, "tns:Foo")])
        .with_quick_xml_config(NamespaceSerialization::Global, None, false, false)
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/deserialize_default_namespace/schema.xsd",
        "tests/feature/deserialize_default_namespace/expected/quick_xml.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod read {
    use super::quick_xml::{Foo, FooTypeContent};
    use xsd_parser_types::quick_xml::ErrorKind;

    use crate::utils::quick_xml_read_test_result;

    macro_rules! path {
        ($name:literal) => {
            concat!(
                "tests/feature/deserialize_default_namespace/example/",
                $name
            )
        };
    }

    #[test]
    fn default_namespace_of_the_schema_selects_local_element() {
        let obj = quick_xml_read_test_result::<Foo, _>(path!("default_t.xml")).unwrap();

        assert!(matches!(obj.content, FooTypeContent::TnsA(x) if x == "t"));
    }

    #[test]
    fn foreign_default_namespace_selects_referenced_element() {
        let obj = quick_xml_read_test_result::<Foo, _>(path!("default_o.xml")).unwrap();

        assert!(matches!(obj.content, FooTypeContent::OA(x) if x == "o"));
    }

    #[test]
    fn missing_default_namespace_is_lenient_and_takes_first_matcher() {
        let obj = quick_xml_read_test_result::<Foo, _>(path!("no_default.xml")).unwrap();

        assert!(matches!(obj.content, FooTypeContent::TnsA(x) if x == "t"));
    }

    #[test]
    fn unknown_default_namespace_is_rejected() {
        let result = quick_xml_read_test_result::<Foo, _>(path!("foreign_default.xml"));

        let err = result.unwrap_err();

        assert!(matches!(err.kind, ErrorKind::UnexpectedEvent(_)), "{err:?}");
    }
}
