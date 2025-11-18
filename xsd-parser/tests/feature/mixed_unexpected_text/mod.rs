use xsd_parser::{config::GeneratorFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_generate([
            (IdentType::Element, "tns:Mixed"),
            (IdentType::Element, "tns:Normal"),
        ])
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! check_obj {
    ($obj:expr) => {{
        use quick_xml::MixedGroupType;

        let obj = $obj;

        let mut group = obj.group.into_iter();

        let MixedGroupType::Fuu(fuu) = group.next().unwrap() else {
            panic!();
        };
        let mut fuu = fuu.into_iter();
        assert!(matches!(fuu.next(), Some(111)));
        assert!(matches!(fuu.next(), Some(222)));
        assert!(matches!(fuu.next(), None));

        let MixedGroupType::Bar(bar) = group.next().unwrap() else {
            panic!();
        };
        let mut bar = bar.into_iter();
        assert!(matches!(bar.next(), Some(333)));
        assert!(matches!(bar.next(), Some(444)));
        assert!(matches!(bar.next(), None));

        let mut baz = obj.baz.into_iter();
        assert!(matches!(baz.next(), Some(555)));
        assert!(matches!(baz.next(), Some(666)));
        assert!(matches!(baz.next(), None));
    }};
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! check_err {
    ($err:expr) => {{
        use xsd_parser_types::quick_xml::ErrorKind;

        let ret = $err;
        let error = ret.unwrap_err();

        assert!(matches!(error.kind, ErrorKind::UnexpectedEvent(_)));
    }};
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/mixed_unexpected_text/schema.xsd",
        "tests/feature/mixed_unexpected_text/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_mixed() {
    use quick_xml::Mixed;

    check_obj!(crate::utils::quick_xml_read_test::<Mixed, _>(
        "tests/feature/mixed_unexpected_text/example/text0.xml"
    ));
    check_obj!(crate::utils::quick_xml_read_test::<Mixed, _>(
        "tests/feature/mixed_unexpected_text/example/text1.xml"
    ));
    check_obj!(crate::utils::quick_xml_read_test::<Mixed, _>(
        "tests/feature/mixed_unexpected_text/example/text2.xml"
    ));
    check_obj!(crate::utils::quick_xml_read_test::<Mixed, _>(
        "tests/feature/mixed_unexpected_text/example/text3.xml"
    ));
    check_obj!(crate::utils::quick_xml_read_test::<Mixed, _>(
        "tests/feature/mixed_unexpected_text/example/text4.xml"
    ));
    check_obj!(crate::utils::quick_xml_read_test::<Mixed, _>(
        "tests/feature/mixed_unexpected_text/example/text5.xml"
    ));
    check_obj!(crate::utils::quick_xml_read_test::<Mixed, _>(
        "tests/feature/mixed_unexpected_text/example/text6.xml"
    ));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_not_mixed() {
    use quick_xml::Normal;

    check_err!(crate::utils::quick_xml_read_test_result::<Normal, _>(
        "tests/feature/mixed_unexpected_text/example/text0.xml"
    ));
    check_err!(crate::utils::quick_xml_read_test_result::<Normal, _>(
        "tests/feature/mixed_unexpected_text/example/text1.xml"
    ));
    check_err!(crate::utils::quick_xml_read_test_result::<Normal, _>(
        "tests/feature/mixed_unexpected_text/example/text2.xml"
    ));
    check_err!(crate::utils::quick_xml_read_test_result::<Normal, _>(
        "tests/feature/mixed_unexpected_text/example/text3.xml"
    ));
    check_err!(crate::utils::quick_xml_read_test_result::<Normal, _>(
        "tests/feature/mixed_unexpected_text/example/text4.xml"
    ));
    check_err!(crate::utils::quick_xml_read_test_result::<Normal, _>(
        "tests/feature/mixed_unexpected_text/example/text5.xml"
    ));
    check_err!(crate::utils::quick_xml_read_test_result::<Normal, _>(
        "tests/feature/mixed_unexpected_text/example/text6.xml"
    ));
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
