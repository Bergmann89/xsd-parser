use xsd_parser::{config::SerdeXmlRsVersion, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([(IdentType::Element, "tns:Foo")])
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! check_obj {
    ($module:ident, $obj:expr) => {{
        use $module::EnumType;

        let obj = $obj;

        assert_eq!(obj.value.as_deref(), Some("an attribute value"));
        assert_eq!(obj.another_value.as_deref(), Some("more attribute data"));
        assert!(matches!(obj.content, EnumType::Auto));
    }};
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! test_obj {
    ($module:ident) => {{
        use $module::{EnumType, FooType};

        FooType {
            value: Some("an attribute value".into()),
            another_value: Some("more attribute data".into()),
            content: EnumType::Auto,
        }
    }};
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/extension_simple_content/schema.xsd",
        "tests/feature/extension_simple_content/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/extension_simple_content/schema.xsd",
        "tests/feature/extension_simple_content/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/extension_simple_content/example/default.xml",
    );

    check_obj!(quick_xml, obj);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    let obj = test_obj!(quick_xml);

    crate::utils::quick_xml_write_test(
        &obj,
        "Foo",
        "tests/feature/extension_simple_content/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

/* serde_xml_rs v0.8 */

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/feature/extension_simple_content/schema.xsd",
        "tests/feature/extension_simple_content/expected/serde_xml_rs.rs",
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::{EnumTypeValue, Foo};

    let obj = crate::utils::serde_xml_rs_read_test::<Foo, _>(
        "tests/feature/extension_simple_content/example/default.xml",
    );

    assert_eq!(obj.value.as_deref(), Some("an attribute value"));
    assert_eq!(obj.another_value.as_deref(), Some("more attribute data"));
    assert!(matches!(obj.content, EnumTypeValue::Auto));
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs {
    #![allow(unused_imports)]

    include!("expected/serde_xml_rs.rs");
}

/* serde_xml_rs v0.7 */

#[test]
fn generate_serde_xml_rs_v7() {
    generate_test(
        "tests/feature/extension_simple_content/schema.xsd",
        "tests/feature/extension_simple_content/expected/serde_xml_rs_v7.rs",
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version07AndBelow),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs_v7() {
    use serde_xml_rs_v7::Foo;

    let obj = crate::utils::serde_xml_rs_v7_read_test::<Foo, _>(
        "tests/feature/extension_simple_content/example/default.xml",
    );

    check_obj!(serde_xml_rs_v7, obj);
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs_v7 {
    #![allow(unused_imports)]

    include!("expected/serde_xml_rs_v7.rs");
}

/* serde_quick_xml */

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/extension_simple_content/schema.xsd",
        "tests/feature/extension_simple_content/expected/serde_quick_xml.rs",
        config().with_serde_quick_xml(),
    );
}

#[test]
// quick_xml is not able to handle white spaces around the value, so it raises an error:
//    "unknown variant `\n    AUTO\n`, expected one of `OFF`, `ON`, `AUTO`"
#[ignore]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::Foo;

    let obj = crate::utils::serde_quick_xml_read_test::<Foo, _>(
        "tests/feature/extension_simple_content/example/default.xml",
    );

    check_obj!(serde_quick_xml, obj);
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(unused_imports)]

    include!("expected/serde_quick_xml.rs");
}
