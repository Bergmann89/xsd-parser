use std::path::Path;

use quote::{quote, ToTokens};
use xsd_parser::{
    config::{IdentTriple, Schema, SerdeXmlRsVersion},
    exec_generator, exec_interpreter, exec_optimizer, exec_parser, exec_render,
    models::data::DataTypeVariant,
    Config, DataTypes, IdentType, Schemas,
};

use crate::utils::{generate_test_validate, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generate([(IdentType::Element, "tns:Foo")])
        .with_derive(["Default", "Debug", "Clone", "Eq", "PartialEq"])
}

fn generate_test(input_xsd: &str, expected_rs: &str, mut config: Config) {
    let input_xsd = Path::new(input_xsd).canonicalize().unwrap();
    config.parser.schemas.push(Schema::File(input_xsd));

    let schemas = exec_parser(config.parser).unwrap();
    let meta_types = exec_interpreter(config.interpreter, &schemas).unwrap();
    let meta_types = exec_optimizer(config.optimizer, meta_types).unwrap();
    let data_types = exec_generator(config.generator, &schemas, &meta_types).unwrap();
    let data_types = extra_attributes(&schemas, data_types);
    let module = exec_render(config.renderer, &data_types).unwrap();
    let code = module.to_token_stream().to_string();

    generate_test_validate(code, expected_rs);
}

fn extra_attributes<'types>(
    schemas: &Schemas,
    mut data_types: DataTypes<'types>,
) -> DataTypes<'types> {
    let ident = IdentTriple::from((IdentType::Type, "tns:EnumType"))
        .resolve(schemas)
        .unwrap();

    let data_type = data_types.get_mut(&ident).unwrap();
    let DataTypeVariant::Enumeration(enum_data) = &mut data_type.variant else {
        panic!();
    };
    let variant = enum_data
        .variants
        .iter_mut()
        .find(|x| x.meta.ident.name.as_str() == "AUTO")
        .unwrap();
    variant.extra_attributes.push(quote!(default));

    data_types
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/derive/schema.xsd",
        "tests/feature/derive/expected/default.rs",
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
        "tests/feature/derive/schema.xsd",
        "tests/feature/derive/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::{EnumType, Foo};

    let obj =
        crate::utils::quick_xml_read_test::<Foo, _>("tests/feature/derive/example/default.xml");

    assert_eq!(obj.min, 1);
    assert_eq!(obj.max, 2);
    assert_eq!(obj.value, EnumType::Auto);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{EnumType, Foo};

    let obj = Foo {
        min: 1,
        max: 2,
        value: EnumType::Auto,
    };

    crate::utils::quick_xml_write_test(&obj, "tns:Foo", "tests/feature/derive/example/default.xml");
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
        "tests/feature/derive/schema.xsd",
        "tests/feature/derive/expected/serde_xml_rs.rs",
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::{EnumTypeValue, Foo};

    let obj =
        crate::utils::serde_xml_rs_read_test::<Foo, _>("tests/feature/derive/example/default.xml");

    assert_eq!(obj.min, 1);
    assert_eq!(obj.max, 2);
    assert_eq!(obj.value.value, EnumTypeValue::Auto);
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
        "tests/feature/derive/schema.xsd",
        "tests/feature/derive/expected/serde_quick_xml.rs",
        config().with_serde_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::{EnumType, Foo};

    let obj = crate::utils::serde_quick_xml_read_test::<Foo, _>(
        "tests/feature/derive/example/default.xml",
    );

    assert_eq!(obj.min, 1);
    assert_eq!(obj.max, 2);
    assert_eq!(obj.value, EnumType::Auto);
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(unused_imports)]

    include!("expected/serde_quick_xml.rs");
}
