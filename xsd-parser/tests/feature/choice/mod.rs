use xsd_parser::{config::SerdeXmlRsVersion, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([(IdentType::Element, "tns:Foo")])
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! check_obj {
    ($module:ident, $obj:expr) => {{
        use $module::FooTypeContent;

        let obj = $obj;

        assert!(matches!(obj.content, FooTypeContent::Baz(3)));
    }};
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! test_obj {
    ($module:ident) => {{
        use $module::{Foo, FooTypeContent};

        Foo {
            content: FooTypeContent::Baz(3),
        }
    }};
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/choice/schema.xsd",
        "tests/feature/choice/expected/default.rs",
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
        "tests/feature/choice/schema.xsd",
        "tests/feature/choice/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Foo;

    let obj =
        crate::utils::quick_xml_read_test::<Foo, _>("tests/feature/choice/example/default.xml");

    check_obj!(quick_xml, obj);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    let obj = test_obj!(quick_xml);

    crate::utils::quick_xml_write_test(&obj, "tns:Foo", "tests/feature/choice/example/default.xml");
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
        "tests/feature/choice/schema.xsd",
        "tests/feature/choice/expected/serde_xml_rs.rs",
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::Foo;

    let obj =
        crate::utils::serde_xml_rs_read_test::<Foo, _>("tests/feature/choice/example/default.xml");

    check_obj!(serde_xml_rs, obj);
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
        "tests/feature/choice/schema.xsd",
        "tests/feature/choice/expected/serde_quick_xml.rs",
        config().with_serde_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::{Foo, FooTypeContent};

    let obj = crate::utils::serde_quick_xml_read_test::<Foo, _>(
        "tests/feature/choice/example/default.xml",
    );

    assert!(matches!(obj.content, FooTypeContent::Baz(3)));
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(unused_imports)]

    include!("expected/serde_quick_xml.rs");
}
