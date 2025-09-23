use xsd_parser::{
    config::{GeneratorFlags, SerdeXmlRsVersion},
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::USE_NAMESPACE_MODULES)
        .with_generate([(IdentType::Element, "tns:Foo")])
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! check_obj {
    ($obj:expr) => {{
        let obj = $obj;

        assert_eq!(obj.a, 150.0);
        assert_eq!(obj.b.b, 3);
        assert_eq!(obj.b.c, "string");
    }};
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! test_obj {
    ($module:ident) => {{
        use $module::{other::BarType, tns::Foo};

        Foo {
            a: 150.0,
            b: BarType {
                b: 3,
                c: "string".into(),
            },
        }
    }};
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/extension_base_two_files/schema.xsd",
        "tests/feature/extension_base_two_files/expected/default.rs",
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
        "tests/feature/extension_base_two_files/schema.xsd",
        "tests/feature/extension_base_two_files/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::tns::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/extension_base_two_files/example/default.xml",
    );

    check_obj!(obj);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    let obj = test_obj!(quick_xml);

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Foo",
        "tests/feature/extension_base_two_files/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

/* serde_xml_rs */

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/feature/extension_base_two_files/schema.xsd",
        "tests/feature/extension_base_two_files/expected/serde_xml_rs.rs",
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::tns::Foo;

    let obj = crate::utils::serde_xml_rs_read_test::<Foo, _>(
        "tests/feature/extension_base_two_files/example/default.xml",
    );

    check_obj!(obj);
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs {
    #![allow(unused_imports)]

    include!("expected/serde_xml_rs.rs");
}

/* serde_quick_xml */

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/extension_base_two_files/schema.xsd",
        "tests/feature/extension_base_two_files/expected/serde_quick_xml.rs",
        config().with_serde_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::tns::Foo;

    let obj = crate::utils::serde_quick_xml_read_test::<Foo, _>(
        "tests/feature/extension_base_two_files/example/default.xml",
    );

    check_obj!(obj);
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(unused_imports)]

    include!("expected/serde_quick_xml.rs");
}
