use std::{cell::Cell, collections::HashSet, rc::Rc};

use xsd_parser::{
    config::{
        GeneratorFlags, OptimizerFlags, RendererFlags, Schema, SerdeXmlRsVersion, TypedefMode,
    },
    generate,
    models::meta::{CustomMeta, MetaType},
    pipeline::generator::{Context as GeneratorContext, Error as GeneratorError},
    Config, Error, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

pub mod support {
    pub type Text = String;
}

fn config(alias: bool) -> Config {
    let mut value = MetaType::from(
        CustomMeta::new("String")
            .include_from("::std::string::String")
            .with_alias(alias),
    );
    value.display_name = Some("RenamedValue".into());
    Config::test_default()
        .with_type((IdentType::Type, "ex:Value"), value)
        .with_type(
            (IdentType::Type, "ex:Sibling"),
            CustomMeta::new("Text")
                .include_from("native::Text")
                .with_alias(alias),
        )
        .with_type(
            (IdentType::Type, "ex:Local"),
            CustomMeta::new("Text")
                .include_from("self::native::Text")
                .with_alias(alias),
        )
        .with_type(
            (IdentType::Type, "ex:CrateValue"),
            CustomMeta::new("Text")
                .include_from("crate::feature::native_aliases::support::Text")
                .with_alias(alias),
        )
        .with_generate([(IdentType::Element, "ex:Message")])
        .with_generator_flags(GeneratorFlags::USE_NAMESPACE_MODULES)
        .with_renderer_flags(RendererFlags::RENDER_DOCS)
        .with_optimizer_flags(OptimizerFlags::all())
}

fn schema_config(alias: bool) -> Config {
    config(alias).with_schema(Schema::schema(include_str!("schema.xsd")))
}

macro_rules! fixture {
    ($name:ident, $test:ident, $config:expr) => {
        #[test]
        fn $test() {
            generate_test(
                "tests/feature/native_aliases/schema.xsd",
                concat!(
                    "tests/feature/native_aliases/expected/",
                    stringify!($name),
                    ".rs"
                ),
                $config,
            );
        }

        #[cfg(not(feature = "update-expectations"))]
        mod $name {
            #![allow(unused_imports)]
            pub mod native {
                pub type Text = String;
            }
            include!(concat!("expected/", stringify!($name), ".rs"));
        }
    };
}

fixture!(default, generate_default, config(true));
fixture!(
    disabled,
    generate_disabled,
    config(false).with_generator_flags(GeneratorFlags::ABSOLUTE_PATHS_INSTEAD_USINGS)
);
fixture!(quick_xml, generate_quick_xml, config(true).with_quick_xml());
fixture!(
    serde_quick_xml,
    generate_serde_quick_xml,
    config(true).with_serde_quick_xml()
);
fixture!(
    serde_xml_rs_v7,
    generate_serde_xml_rs_v7,
    config(true).with_serde_xml_rs(SerdeXmlRsVersion::Version07AndBelow)
);
fixture!(
    serde_xml_rs_v8,
    generate_serde_xml_rs_v8,
    config(true).with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove)
);
fixture!(
    newtype_mode,
    generate_newtype_mode,
    config(true)
        .with_typedef_mode(TypedefMode::NewType)
        .with_generate([
            (IdentType::Type, "ex:Value"),
            (IdentType::Type, "ex:Sibling"),
            (IdentType::Type, "ex:Local"),
            (IdentType::Type, "ex:CrateValue"),
        ])
);

#[test]
fn custom_mapping_identity_distinguishes_aliases() {
    let direct = CustomMeta::new("Text");
    let alias = direct.clone().with_alias(true);
    assert_ne!(direct, alias);

    let mappings = HashSet::from([direct.clone(), direct, alias.clone(), alias]);
    assert_eq!(mappings.len(), 2);
}

#[test]
fn custom_generator_runs_once_with_and_without_aliases() {
    for alias in [false, true] {
        let calls = Rc::new(Cell::new(0));
        let counter = Rc::clone(&calls);
        let mapping = CustomMeta::new("String")
            .include_from("::std::string::String")
            .with_alias(alias)
            .with_generator(move |_: &mut GeneratorContext<'_, '_>, _: &CustomMeta| {
                counter.set(counter.get() + 1);
                Ok(())
            });
        generate(
            schema_config(alias)
                .with_type((IdentType::Type, "ex:Value"), mapping)
                .with_generate([(IdentType::Type, "ex:Value")]),
        )
        .unwrap();

        assert_eq!(calls.get(), 1, "alias={alias}");
    }
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn aliases_keep_native_type_identity() {
    let _: default::ex::RenamedValue = String::from("value");
    let _: default::ex::SiblingType = String::from("sibling");
    let _: default::ex::LocalType = String::from("local");
    let _: default::ex::CrateValueType = String::from("crate");
    let _: newtype_mode::ex::RenamedValue = String::from("value");
    let _: newtype_mode::ex::SiblingType = String::from("sibling");
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! check_message {
    ($obj:expr) => {{
        let obj = &$obj;
        assert_eq!(obj.value, "value");
        assert_eq!(obj.sibling, "sibling");
        assert_eq!(obj.local, "local");
        assert_eq!(obj.crate_value, "crate");
    }};
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn round_trip_quick_xml() {
    let obj = crate::utils::quick_xml_read_test::<quick_xml::ex::Message, _>(
        "tests/feature/native_aliases/example/default.xml",
    );
    check_message!(obj);
    crate::utils::quick_xml_write_test(
        &obj,
        "ex:Message",
        "tests/feature/native_aliases/example/default.xml",
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn round_trip_serde_quick_xml() {
    let obj = crate::utils::serde_quick_xml_read_test::<serde_quick_xml::ex::Message, _>(
        "tests/feature/native_aliases/example/default.xml",
    );
    check_message!(obj);
    let xml = ::quick_xml::se::to_string(&obj).unwrap();
    let obj: serde_quick_xml::ex::Message = ::quick_xml::de::from_str(&xml).unwrap();
    check_message!(obj);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn round_trip_serde_xml_rs_v7() {
    let obj = crate::utils::serde_xml_rs_v7_read_test::<serde_xml_rs_v7::ex::Message, _>(
        "tests/feature/native_aliases/example/default.xml",
    );
    check_message!(obj);
    let xml = ::serde_xml_rs_v7::to_string(&obj).unwrap();
    let obj: serde_xml_rs_v7::ex::Message = ::serde_xml_rs_v7::from_str(&xml).unwrap();
    check_message!(obj);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn round_trip_serde_xml_rs_v8() {
    let obj = crate::utils::serde_xml_rs_read_test::<serde_xml_rs_v8::ex::Message, _>(
        "tests/feature/native_aliases/example/default.xml",
    );
    check_message!(obj);
    let xml = ::serde_xml_rs::to_string(&obj).unwrap();
    let obj: serde_xml_rs_v8::ex::Message = ::serde_xml_rs::from_str(&xml).unwrap();
    check_message!(obj);
}

#[test]
fn explicit_documentation_and_distinct_aliases_are_preserved() {
    let mut identical = MetaType::from(
        CustomMeta::new("String")
            .include_from("::std::string::String")
            .with_alias(true),
    );
    identical.documentation = vec!["Explicit documentation.".into()];
    let code = generate(
        schema_config(true)
            .with_type((IdentType::Type, "ex:Value"), identical.clone())
            .with_type((IdentType::Type, "ex:Sibling"), identical),
    )
    .unwrap()
    .to_string();
    assert!(code.contains("pub type ValueType"), "{code}");
    assert!(code.contains("pub type SiblingType"), "{code}");
    assert!(code.contains("Explicit documentation."), "{code}");
    assert!(!code.contains("First paragraph."), "{code}");
}

#[test]
fn rejects_missing_targets() {
    let missing = schema_config(true).with_type(
        (IdentType::Type, "ex:Value"),
        CustomMeta::new("String").with_alias(true),
    );
    assert!(matches!(generate(missing),
        Err(Error::GeneratorError(GeneratorError::MissingCustomAliasTarget(ident)))
            if ident.name.as_str() == "Value"));
}

#[test]
fn rejects_name_collisions() {
    let mut duplicate = MetaType::from(
        CustomMeta::new("String")
            .include_from("::std::string::String")
            .with_alias(true),
    );
    duplicate.display_name = Some("RenamedValue".into());
    let collision = schema_config(true).with_type((IdentType::Type, "ex:Sibling"), duplicate);
    assert!(matches!(generate(collision),
        Err(Error::GeneratorError(GeneratorError::DuplicateTypeName { path, .. }))
            if path.ends_with("RenamedValue")));
}
