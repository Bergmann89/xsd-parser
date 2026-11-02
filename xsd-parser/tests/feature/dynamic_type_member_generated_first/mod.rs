use std::path::Path;

use xsd_parser::{
    config::{IdentQuadruple, Schema},
    generate, Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([
        (IdentType::Element, "tns:Foo"),
        (IdentType::Element, "tns:Holder"),
    ])
}

/* user defined member type */

#[test]
fn user_defined_member_type_is_not_generated() {
    let mut config = config().with_quick_xml();
    config
        .generator
        .types
        .push(IdentQuadruple::from((IdentType::Type, "tns:FooType")));
    let xsd = Path::new("tests/feature/dynamic_type_member_generated_first/schema.xsd")
        .canonicalize()
        .unwrap();
    config.parser.schemas.push(Schema::File(xsd));

    let code = generate(config).unwrap().to_string();

    assert!(!code.contains("pub struct FooType"), "{code}");
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/dynamic_type_member_generated_first/schema.xsd",
        "tests/feature/dynamic_type_member_generated_first/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_holder() {
    use quick_xml::{FooType, Holder};

    let obj = crate::utils::quick_xml_read_test::<Holder, _>(
        "tests/feature/dynamic_type_member_generated_first/example/holder.xml",
    );

    let member = obj.head.0.as_any().downcast_ref::<FooType>().unwrap();
    assert_eq!(member.name, "abc");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
