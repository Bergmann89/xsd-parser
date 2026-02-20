use xsd_parser::{Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_advanced_enums()
        .with_qname_type()
        .with_generate([(IdentType::Element, "tns:Foo")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/advanced_enums/schema.xsd",
        "tests/feature/advanced_enums/expected/default.rs",
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
        "tests/feature/advanced_enums/schema.xsd",
        "tests/feature/advanced_enums/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::{Foo, QNameEnumType, StringEnumType};

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/advanced_enums/example/default.xml",
    );

    assert!(matches!(obj.string_enum, StringEnumType::On));
    assert!(matches!(obj.q_name_enum, QNameEnumType::TnsBar));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{Foo, QNameEnumType, StringEnumType};

    let obj = Foo {
        string_enum: StringEnumType::On,
        q_name_enum: QNameEnumType::TnsBar,
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Foo",
        "tests/feature/advanced_enums/example/serialize.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
