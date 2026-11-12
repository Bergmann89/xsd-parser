use proc_macro2::{Delimiter, TokenTree};

use xsd_parser::{
    config::{
        BoxFlags, GeneratorFlags, OptimizerFlags, RenderStep, RendererFlags, Schema,
        SerdeXmlRsVersion, TypedefMode,
    },
    generate,
    models::meta::CustomMeta,
    Config, IdentType, PolymorphicFromRenderStep,
};

use crate::utils::{generate_test, ConfigEx};

fn base_config() -> Config {
    Config::test_default()
        .with_optimizer_flags(OptimizerFlags::CONVERT_DYNAMIC_TO_CHOICE)
        .without_optimizer_flags(OptimizerFlags::RESOLVE_TYPEDEFS)
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_typedef_mode(TypedefMode::Typedef)
        .with_generate([
            (IdentType::Type, "t:Base"),
            (IdentType::Type, "t:Middle"),
            (IdentType::Type, "t:Empty"),
            (IdentType::Type, "t:Ordinary"),
            (IdentType::Element, "t:Head"),
            (IdentType::Element, "t:MixedHead"),
            (IdentType::Element, "t:Unique"),
        ])
}

fn config() -> Config {
    base_config().with_render_step(RenderStep::PolymorphicFrom)
}

fn aliases_config() -> Config {
    config()
        .with_generate([
            (IdentType::Element, "t:ScalarHead"),
            (IdentType::Element, "t:TextHead"),
            (IdentType::Element, "t:PathHead"),
            (IdentType::Element, "t:DistinctHead"),
        ])
        .with_type(
            (IdentType::Type, "t:NativeNumber"),
            CustomMeta::new("u32").include_from("u32").with_alias(true),
        )
        .with_type((IdentType::Type, "t:NativeText"), CustomMeta::new("String"))
        .with_type(
            (IdentType::Type, "t:BareText"),
            CustomMeta::new("Text")
                .include_from("native::Text")
                .with_alias(true),
        )
        .with_type(
            (IdentType::Type, "t:LocalText"),
            CustomMeta::new("Text")
                .include_from("self::native::Text")
                .with_alias(true),
        )
        .with_type(
            (IdentType::Type, "t:CrateText"),
            CustomMeta::new("Text")
                .include_from("crate::feature::polymorphic_from::support::Text")
                .with_alias(true),
        )
        .with_type(
            (IdentType::Type, "t:LeftText"),
            CustomMeta::new("Text")
                .include_from("self::left::Text")
                .with_alias(true),
        )
        .with_type(
            (IdentType::Type, "t:RightText"),
            CustomMeta::new("Text")
                .include_from("self::right::Text")
                .with_alias(true),
        )
}

pub mod support {
    pub type Text = String;
}

#[test]
fn aliases() {
    generate_test(
        "tests/feature/polymorphic_from/aliases.xsd",
        "tests/feature/polymorphic_from/expected/aliases.rs",
        aliases_config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod aliases {
    pub mod native {
        pub type Text = super::super::support::Text;
    }
    pub mod left {
        #[derive(Debug)]
        pub struct Text(pub u32);
    }
    pub mod right {
        #[derive(Debug)]
        pub struct Text(pub u32);
    }
    include!("expected/aliases.rs");
}

#[test]
fn nillable() {
    generate_test(
        "tests/feature/polymorphic_from/nillable.xsd",
        "tests/feature/polymorphic_from/expected/nillable.rs",
        Config::test_default()
            .with_optimizer_flags(OptimizerFlags::CONVERT_DYNAMIC_TO_CHOICE)
            .with_generator_flags(
                GeneratorFlags::FLATTEN_CONTENT | GeneratorFlags::NILLABLE_TYPE_SUPPORT,
            )
            .with_render_step(RenderStep::PolymorphicFrom)
            .with_generate([
                (IdentType::Element, "t:Head"),
                (IdentType::Element, "t:Ambiguous"),
            ]),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod nillable {
    include!("expected/nillable.rs");
}

#[test]
fn unconverted_dynamic_types_are_unchanged() {
    let config = base_config()
        .without_optimizer_flags(OptimizerFlags::CONVERT_DYNAMIC_TO_CHOICE)
        .with_schema(Schema::File(
            "tests/feature/polymorphic_from/schema.xsd".into(),
        ));
    let enabled = generate(config.clone().with_render_step(RenderStep::PolymorphicFrom))
        .unwrap()
        .to_string();
    let disabled = generate(config).unwrap().to_string();
    assert_eq!(enabled, disabled);
}

#[test]
fn standalone_step_emits_only_conversions_and_is_deduplicated() {
    let mut config = base_config().with_schema(Schema::File(
        "tests/feature/polymorphic_from/schema.xsd".into(),
    ));
    config.renderer.steps.clear();
    let config = config
        .with_render_step(RenderStep::PolymorphicFrom)
        .with_render_step(PolymorphicFromRenderStep)
        .with_render_step(RenderStep::PolymorphicFrom);
    let mut tokens = generate(config).unwrap().into_iter();
    let mut count = 0;
    while let Some(token) = tokens.next() {
        assert!(matches!(token, TokenTree::Ident(keyword) if keyword == "impl"));
        assert!(tokens.by_ref().any(|token| {
            matches!(token, TokenTree::Group(body) if body.delimiter() == Delimiter::Brace)
        }));
        count += 1;
    }
    assert_eq!(count, 6);
}

macro_rules! fixture {
    ($name:ident, $config:expr) => {
        fixture!($name, "schema.xsd", $config);
    };
    ($name:ident, $schema:literal, $config:expr) => {
        #[test]
        fn $name() {
            generate_test(
                concat!("tests/feature/polymorphic_from/", $schema),
                concat!(
                    "tests/feature/polymorphic_from/expected/",
                    stringify!($name),
                    ".rs"
                ),
                $config,
            );
        }

        #[cfg(not(feature = "update-expectations"))]
        mod $name {
            #![allow(unused_imports)]
            include!(concat!("expected/", stringify!($name), ".rs"));
        }
    };
}

fixture!(
    disabled,
    base_config().with_renderer_flags(RendererFlags::all())
);
fixture!(flat, config());
fixture!(
    non_abstract,
    "non_abstract.xsd",
    config()
        .with_dynamic_type((IdentType::Type, "t:Base"))
        .with_generate([(IdentType::Type, "t:Base")])
);
fixture!(alloc_boxed, {
    let mut config = config()
        .with_box_flags(BoxFlags::ENUM_ELEMENTS)
        .with_generator_flags(GeneratorFlags::BUILD_IN_ABSOLUTE_PATHS);
    config.renderer.alloc = "alloc".into();
    config
});
fixture!(
    unflattened,
    config().without_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
);
fixture!(
    modules,
    config().with_generator_flags(
        GeneratorFlags::USE_NAMESPACE_MODULES | GeneratorFlags::USE_TYPE_MODULES
    )
);
fixture!(serde_quick_xml, config().with_serde_quick_xml());
fixture!(
    serde_xml_rs_v7,
    config().with_serde_xml_rs(SerdeXmlRsVersion::Version07AndBelow)
);
fixture!(
    serde_xml_rs_v8,
    config().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove)
);

#[test]
#[cfg(not(feature = "update-expectations"))]
fn conversions_preserve_payloads() {
    let value: flat::BaseType = flat::LeafType {
        id: 7,
        label: "leaf".into(),
    }
    .into();
    let flat::BaseType::Leaf(leaf) = value else {
        panic!("wrong variant")
    };
    assert_eq!((leaf.id, leaf.label.as_str()), (7, "leaf"));
    let value: flat::MiddleType = flat::LeafType {
        id: 8,
        label: "middle".into(),
    }
    .into();
    let flat::MiddleType::Leaf(leaf) = value;
    assert_eq!(leaf.id, 8);
    let value: flat::Unique = true.into();
    assert!(matches!(value, flat::Unique::Boolean(true)));
    let value: flat::Head = 42.into();
    assert!(matches!(value, flat::Head::Number(42)));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn unflattened_conversions_target_content_enum() {
    let value: unflattened::BaseTypeContent = unflattened::LeafType {
        id: 10,
        label: "content".into(),
    }
    .into();
    let unflattened::BaseTypeContent::Leaf(leaf) = value else {
        panic!("wrong variant")
    };
    assert_eq!(leaf.id, 10);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn conversions_work_across_modules_and_recursive_storage() {
    let leaf: modules::t::BaseType = modules::f::ForeignLeafType { id: 11, value: 12 }.into();
    let value: modules::t::BaseType = modules::t::NodeType {
        id: 13,
        child: Some(Box::new(leaf)),
    }
    .into();
    let modules::t::BaseType::Node(node) = value else {
        panic!("wrong variant")
    };
    let modules::t::BaseType::ForeignLeaf(leaf) = *node.child.unwrap() else {
        panic!("wrong child variant")
    };
    assert_eq!((leaf.id, leaf.value), (11, 12));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn conversions_use_configured_allocator() {
    let value: alloc_boxed::BaseType = alloc_boxed::LeafType {
        id: 9,
        label: "boxed".into(),
    }
    .into();
    let alloc_boxed::BaseType::Leaf(leaf) = value else {
        panic!("wrong variant")
    };
    assert_eq!((leaf.id, leaf.label.as_str()), (9, "boxed"));
    let _: alloc::boxed::Box<alloc_boxed::LeafType> = leaf;
    let value: alloc_boxed::Unique = true.into();
    let alloc_boxed::Unique::Boolean(value) = value;
    let value: alloc::boxed::Box<bool> = value;
    assert!(*value);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn nullable_and_plain_payloads_remain_distinct() {
    let value: nillable::Head = true.into();
    assert!(matches!(value, nillable::Head::Plain(true)));
    let value: nillable::Head = nillable::NilDyn::nil().into();
    let nillable::Head::Nil(value) = value else {
        panic!("wrong variant")
    };
    assert!(value.is_nil());
    let value: nillable::Ambiguous = false.into();
    assert!(matches!(value, nillable::Ambiguous::PlainMember(false)));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn concrete_base_and_derived_values_convert() {
    let base: non_abstract::BaseType = non_abstract::BaseDyn { id: 21 }.into();
    assert!(matches!(base, non_abstract::BaseType::Base(value) if value.id == 21));
    let derived: non_abstract::BaseType = non_abstract::DerivedType {
        id: 22,
        label: "derived".into(),
    }
    .into();
    assert!(matches!(derived, non_abstract::BaseType::Derived(value)
        if value.id == 22 && value.label == "derived"));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn ambiguous_payloads_still_support_explicit_variants() {
    let value: aliases::ScalarHead = true.into();
    assert!(matches!(value, aliases::ScalarHead::Boolean(true)));
    let value = aliases::ScalarHead::AliasedNumber(23);
    assert!(matches!(value, aliases::ScalarHead::AliasedNumber(23)));
    let value = aliases::PathHead::Crate("text".into());
    assert!(matches!(value, aliases::PathHead::Crate(value) if value == "text"));
    let left = aliases::DistinctHead::Left(aliases::left::Text(24));
    let right = aliases::DistinctHead::Right(aliases::right::Text(25));
    assert!(matches!(left, aliases::DistinctHead::Left(value) if value.0 == 24));
    assert!(matches!(right, aliases::DistinctHead::Right(value) if value.0 == 25));
}
