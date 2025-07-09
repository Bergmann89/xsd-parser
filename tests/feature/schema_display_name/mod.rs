use quote::ToTokens;
use xsd_parser::{
    config::{GeneratorFlags, NamespaceIdent, Schema, SerdeXmlRsVersion},
    exec_generator, exec_interpreter, exec_optimizer, exec_parser, exec_render,
    models::schema::Namespace,
    Config, IdentType,
};

use crate::utils::{generate_test_validate, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::USE_MODULES)
        .with_generate([(
            IdentType::Element,
            Some(NamespaceIdent::namespace(b"http://example.com")),
            "Foo",
        )])
}

#[test]
fn generate_default() {
    exec_generate_test(
        "tests/feature/schema_display_name/schema.xsd",
        "tests/feature/schema_display_name/expected/default.rs",
        config(),
    );
}

#[test]
fn generate_quick_xml() {
    exec_generate_test(
        "tests/feature/schema_display_name/schema.xsd",
        "tests/feature/schema_display_name/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
fn generate_serde_xml_rs() {
    exec_generate_test(
        "tests/feature/schema_display_name/schema.xsd",
        "tests/feature/schema_display_name/expected/serde_xml_rs.rs",
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    );
}

#[test]
fn generate_serde_quick_xml() {
    exec_generate_test(
        "tests/feature/schema_display_name/schema.xsd",
        "tests/feature/schema_display_name/expected/serde_quick_xml.rs",
        config().with_serde_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::example::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/schema_display_name/example/default.xml",
    );

    assert_eq!(obj.once, 222);
    assert_eq!(obj.optional, None);
    assert_eq!(obj.once_specify, 444);
    assert_eq!(obj.twice_or_more, &[111, 333, 555]);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::example::Foo;

    let obj = Foo {
        once: 222,
        optional: None,
        once_specify: 444,
        twice_or_more: vec![111, 333, 555],
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "Foo",
        "tests/feature/schema_display_name/example/default.xml",
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::example::Foo;

    let obj = crate::utils::serde_xml_rs_read_test::<Foo, _>(
        "tests/feature/schema_display_name/example/serde.xml",
    );

    assert_eq!(obj.once, 111);
    assert_eq!(obj.optional, Some(222));
    assert_eq!(obj.once_specify, 333);
    assert_eq!(obj.twice_or_more, &[444, 555, 666]);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::example::Foo;

    let obj = crate::utils::serde_quick_xml_read_test::<Foo, _>(
        "tests/feature/schema_display_name/example/serde.xml",
    );

    assert_eq!(obj.once, 111);
    assert_eq!(obj.optional, Some(222));
    assert_eq!(obj.once_specify, 333);
    assert_eq!(obj.twice_or_more, &[444, 555, 666]);
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs {
    #![allow(unused_imports)]

    include!("expected/serde_xml_rs.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(unused_imports)]

    include!("expected/serde_quick_xml.rs");
}

fn exec_generate_test(input_xsd: &str, expected_rs: &str, mut config: Config) {
    config.parser.schemas.push(Schema::File(input_xsd.into()));

    let mut schemas = exec_parser(config.parser).unwrap();
    let ns_id = schemas
        .resolve_namespace(&Some(Namespace::new_const(b"http://example.com")))
        .unwrap();
    let ns_info = schemas.get_namespace_info_mut(&ns_id).unwrap();
    ns_info.module_name = Some("example".into());

    let meta_types = exec_interpreter(config.interpreter, &schemas).unwrap();
    let meta_types = exec_optimizer(config.optimizer, meta_types).unwrap();
    let data_types = exec_generator(config.generator, &schemas, &meta_types).unwrap();
    let module = exec_render(config.renderer, &data_types).unwrap();
    let code = module.to_token_stream().to_string();

    generate_test_validate(code, expected_rs);
}
