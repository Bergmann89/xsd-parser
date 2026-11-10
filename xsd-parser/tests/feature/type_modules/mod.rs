use xsd_parser::{
    config::{
        GeneratorFlags, IdentQuadruple, NamespaceIdent, OptimizerFlags, Schema, SerdeXmlRsVersion,
    },
    generate_modules,
    models::ExplicitNaming,
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

fn config(namespace: bool) -> Config {
    let mut flags = GeneratorFlags::USE_TYPE_MODULES | GeneratorFlags::FLATTEN_CONTENT;
    if namespace {
        flags |= GeneratorFlags::USE_NAMESPACE_MODULES;
    }
    Config::test_default()
        .with_generator_flags(flags)
        .with_dynamic_type((IdentType::Type, "tns:animal"))
        .with_generate([(IdentType::Element, "tns:list")])
}

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/dynamic_type/schema.xsd",
        "tests/feature/type_modules/expected/default.rs",
        config(false),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]
    include!("expected/default.rs");
}

#[test]
fn generate_namespace_modules() {
    generate_test(
        "tests/feature/dynamic_type/schema.xsd",
        "tests/feature/type_modules/expected/namespace_modules.rs",
        config(true),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod namespace_modules {
    #![allow(unused_imports)]
    include!("expected/namespace_modules.rs");
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/dynamic_type/schema.xsd",
        "tests/feature/type_modules/expected/quick_xml.rs",
        config(true).with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::tns::{AnimalDyn, DogType, LabradorType, List};
    let obj = crate::utils::quick_xml_read_test::<List, _>(
        "tests/feature/dynamic_type/example/default.xml",
    );
    let mut animals = obj.animal.iter();
    assert_eq!(
        animals
            .next()
            .unwrap()
            .0
            .as_any()
            .downcast_ref::<AnimalDyn>()
            .unwrap()
            .id,
        1
    );
    let dog = animals
        .next()
        .unwrap()
        .0
        .as_any()
        .downcast_ref::<DogType>()
        .unwrap();
    assert_eq!(dog.name, "Rex");
    let labrador = animals
        .next()
        .unwrap()
        .0
        .as_any()
        .downcast_ref::<LabradorType>()
        .unwrap();
    assert_eq!(labrador.name, "Buddy");
    assert_eq!(labrador.color, "black");
    assert!(animals.next().is_none());
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]
    include!("expected/quick_xml.rs");
}

#[test]
fn generate_substitution_group_quick_xml() {
    generate_test(
        "tests/feature/substitution_group/schema.xsd",
        "tests/feature/type_modules/expected/substitution_group_quick_xml.rs",
        Config::test_default()
            .with_generator_flags(
                GeneratorFlags::USE_TYPE_MODULES
                    | GeneratorFlags::USE_NAMESPACE_MODULES
                    | GeneratorFlags::FLATTEN_CONTENT,
            )
            .with_generate([(IdentType::Element, "tns:list")])
            .with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_substitution_group_quick_xml() {
    use substitution_group_quick_xml::tns::{AnimalType, DogType, LabradorType, List};
    let obj = crate::utils::quick_xml_read_test::<List, _>(
        "tests/feature/substitution_group/example/default.xml",
    );
    let mut animals = obj.animal.iter();
    assert_eq!(
        animals
            .next()
            .unwrap()
            .0
            .as_any()
            .downcast_ref::<AnimalType>()
            .unwrap()
            .id,
        1
    );
    let dog = animals
        .next()
        .unwrap()
        .0
        .as_any()
        .downcast_ref::<DogType>()
        .unwrap();
    assert_eq!(dog.name, "Rex");
    let labrador = animals
        .next()
        .unwrap()
        .0
        .as_any()
        .downcast_ref::<LabradorType>()
        .unwrap();
    assert_eq!(labrador.name, "Buddy");
    assert_eq!(labrador.color, "black");
    assert!(animals.next().is_none());
}

#[cfg(not(feature = "update-expectations"))]
mod substitution_group_quick_xml {
    #![allow(unused_imports)]
    include!("expected/substitution_group_quick_xml.rs");
}

#[test]
fn splitting_is_opt_in() {
    let config = Config::test_default()
        .with_schema(Schema::schema(
            r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
                xmlns:tns="urn:example" targetNamespace="urn:example">
                <xs:element name="Root" type="tns:Node"/>
                <xs:complexType name="Node"><xs:sequence>
                    <xs:element name="Next" type="tns:Node" minOccurs="0"/>
                </xs:sequence></xs:complexType>
            </xs:schema>"#,
        ))
        .with_generate([(IdentType::Element, "tns:Root")])
        .with_generator_flags(
            GeneratorFlags::USE_NAMESPACE_MODULES | GeneratorFlags::FLATTEN_CONTENT,
        );
    let inline = generate_modules(config.clone()).unwrap();
    assert!(inline.modules["tns"].modules.is_empty());
    let split = generate_modules(
        config
            .with_generator_flags(GeneratorFlags::USE_TYPE_MODULES)
            .with_quick_xml(),
    )
    .unwrap();
    assert!(split.modules["tns"].modules.contains_key("type_root"));
    assert!(split.modules["tns"].modules.contains_key("type_node_type"));
}

#[test]
fn disambiguates_module_names() {
    let config = Config::test_default()
        .with_naming(ExplicitNaming::default().unify_names(false))
        .with_generator_flags(GeneratorFlags::USE_TYPE_MODULES)
        .with_schema(Schema::schema(
            r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
                <xs:complexType name="Entry"/>
                <xs:complexType name="ENTRY"/>
            </xs:schema>"#,
        ))
        .with_generate([(IdentType::Type, "Entry"), (IdentType::Type, "ENTRY")]);
    let module = generate_modules(config).unwrap();
    assert!(module.modules.contains_key("type_entry_type"));
    assert!(module.modules.contains_key("type_entry_type_2"));
}

#[test]
fn unselected_types_do_not_change_module_names() {
    let generate = |extra: &str| {
        generate_modules(
            Config::test_default()
                .with_naming(ExplicitNaming::default().unify_names(false))
                .with_generator_flags(GeneratorFlags::USE_TYPE_MODULES)
                .with_schema(Schema::schema(format!(
                    r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
                        <xs:complexType name="Entry"/>{extra}
                    </xs:schema>"#,
                )))
                .with_generate([(IdentType::Type, "Entry")]),
        )
        .unwrap()
    };
    let initial = generate("");
    let extended = generate(r#"<xs:complexType name="ENTRY"/>"#);
    assert_eq!(
        initial.modules.keys().collect::<Vec<_>>(),
        extended.modules.keys().collect::<Vec<_>>()
    );
    assert!(extended.modules.contains_key("type_entry_type"));
}

#[test]
fn rejects_dynamic_trait_name_collisions() {
    let result = generate_modules(
        Config::test_default()
            .with_type_postfix("")
            .with_generator_flags(GeneratorFlags::USE_TYPE_MODULES)
            .with_schema(Schema::schema(
                r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
                    <xs:complexType name="Base"/>
                    <xs:complexType name="BaseTrait"/>
                </xs:schema>"#,
            ))
            .with_dynamic_type((IdentType::Type, "Base"))
            .with_generate([(IdentType::Type, "Base"), (IdentType::Type, "BaseTrait")]),
    );
    assert!(matches!(
        result,
        Err(xsd_parser::Error::GeneratorError(
            xsd_parser::pipeline::generator::Error::DuplicateTypeName { ref path, .. }
        )) if path == "BaseTrait"
    ));
}

#[test]
fn generate_schema_modules() {
    generate_test(
        "tests/feature/duplicate_idents/schema.xsd",
        "tests/feature/type_modules/expected/schema_modules.rs",
        Config::test_default()
            .with_generate([
                IdentQuadruple::new("Outer", IdentType::Element)
                    .with_ns(NamespaceIdent::namespace(b"Test".as_slice()))
                    .with_schema("foo".to_owned()),
                IdentQuadruple::new("Outer", IdentType::Element)
                    .with_ns(NamespaceIdent::namespace(b"Test".as_slice()))
                    .with_schema("bar".to_owned()),
            ])
            .with_generator_flags(
                GeneratorFlags::USE_TYPE_MODULES
                    | GeneratorFlags::USE_NAMESPACE_MODULES
                    | GeneratorFlags::USE_SCHEMA_MODULES
                    | GeneratorFlags::FLATTEN_CONTENT,
            ),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod schema_modules {
    #![allow(unused_imports)]
    include!("expected/schema_modules.rs");
}

fn simple_config() -> Config {
    Config::test_default()
        .with_generate([(IdentType::Element, "tns:Foo")])
        .with_generator_flags(GeneratorFlags::USE_TYPE_MODULES | GeneratorFlags::FLATTEN_CONTENT)
}

fn imports_config() -> Config {
    Config::test_default()
        .with_generate([(IdentType::Element, "tns:Root")])
        .with_optimizer_flags(OptimizerFlags::MERGE_ENUM_UNIONS)
        .without_optimizer_flags(OptimizerFlags::USE_UNRESTRICTED_BASE_TYPE_SIMPLE)
        .with_generator_flags(
            GeneratorFlags::USE_TYPE_MODULES
                | GeneratorFlags::USE_NAMESPACE_MODULES
                | GeneratorFlags::FLATTEN_CONTENT,
        )
        .with_quick_xml()
}

#[test]
fn fallback_enumerations_do_not_import_unused_errors() {
    let module =
        generate_modules(imports_config().with_schema(Schema::schema(include_str!("imports.xsd"))))
            .unwrap();
    let modules = &module.modules["tns"].modules;
    for name in ["ErrorKind", "RawByteStr"] {
        assert!(
            !modules["type_limit_type"]
                .usings
                .keys()
                .any(|key| key.ends_with(name)),
            "the native fallback handles errors without {name}",
        );
        assert!(modules["type_mode_type"]
            .usings
            .keys()
            .any(|key| key.ends_with(name)));
    }
}

#[test]
fn generate_imports_quick_xml() {
    generate_test(
        "tests/feature/type_modules/imports.xsd",
        "tests/feature/type_modules/expected/imports_quick_xml.rs",
        imports_config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod imports_quick_xml {
    #![deny(unused_imports)]
    include!("expected/imports_quick_xml.rs");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_imports_quick_xml() {
    use imports_quick_xml::tns::{
        ChoiceType, LabelType, LimitType, ModeType, Root, RootType, WordsType,
    };

    let RootType {
        limit,
        choice,
        label: LabelType(label),
        words: WordsType(words),
        mode,
    } = crate::utils::quick_xml_read_test::<Root, _>(
        "tests/feature/type_modules/example/imports.xml",
    );
    let words: Vec<imports_quick_xml::xs::StringType> = words;
    assert!(matches!(limit, LimitType::Unbounded));
    assert!(matches!(choice, ChoiceType::I32(42)));
    assert_eq!(label, "example");
    assert_eq!(words, ["one", "two"]);
    assert!(matches!(mode, ModeType::On));
}

#[test]
fn generate_simple() {
    generate_test(
        "tests/feature/enumeration/schema.xsd",
        "tests/feature/type_modules/expected/simple.rs",
        simple_config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod simple {
    #![allow(unused_imports)]
    include!("expected/simple.rs");
}

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/enumeration/schema.xsd",
        "tests/feature/type_modules/expected/serde_quick_xml.rs",
        simple_config().with_serde_quick_xml(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(unused_imports)]
    include!("expected/serde_quick_xml.rs");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    let obj = crate::utils::serde_quick_xml_read_test::<serde_quick_xml::Foo, _>(
        "tests/feature/enumeration/example/default.xml",
    );
    assert!(matches!(obj.enum_, serde_quick_xml::EnumType::Auto));
}

#[test]
fn generate_serde_xml_rs_v7() {
    generate_test(
        "tests/feature/enumeration/schema.xsd",
        "tests/feature/type_modules/expected/serde_xml_rs_v7.rs",
        simple_config().with_serde_xml_rs(SerdeXmlRsVersion::Version07AndBelow),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs_v7 {
    #![allow(unused_imports)]
    include!("expected/serde_xml_rs_v7.rs");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs_v7() {
    let obj = crate::utils::serde_xml_rs_v7_read_test::<serde_xml_rs_v7::Foo, _>(
        "tests/feature/enumeration/example/default.xml",
    );
    assert!(matches!(obj.enum_, serde_xml_rs_v7::EnumType::Auto));
}

#[test]
fn generate_serde_xml_rs_v8() {
    generate_test(
        "tests/feature/enumeration/schema.xsd",
        "tests/feature/type_modules/expected/serde_xml_rs_v8.rs",
        simple_config().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs_v8 {
    #![allow(unused_imports)]
    include!("expected/serde_xml_rs_v8.rs");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs_v8() {
    let obj = crate::utils::serde_xml_rs_read_test::<serde_xml_rs_v8::Foo, _>(
        "tests/feature/enumeration/example/default.xml",
    );
    assert!(matches!(
        obj.enum_.value,
        serde_xml_rs_v8::EnumTypeValue::Auto
    ));
}
