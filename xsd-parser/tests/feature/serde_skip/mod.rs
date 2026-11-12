use quote::{quote, ToTokens};
use xsd_parser::{
    config::{GeneratorFlags, RenderStep, SerdeXmlRsVersion},
    models::data::{ComplexData, ComplexDataStruct, DataTypeVariant, StructMode},
    Config, IdentType,
};

use crate::utils::{generate_test, generate_test_validate, render_with_customizations, ConfigEx};

mod aliases;
mod enum_content;
mod schema_defaults;

fn config(skip_none: bool, skip_empty_vecs: bool) -> Config {
    Config::test_default()
        .with_generate([
            (IdentType::Type, "tns:MessageType"),
            (IdentType::Type, "tns:ContentType"),
            (IdentType::Type, "tns:PayloadType"),
        ])
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_derive(["Debug", "PartialEq"])
        .with_render_steps([
            RenderStep::TypesSerdeQuickXml {
                large_arrays: false,
                skip_none,
                skip_empty_vecs,
            },
            RenderStep::Defaults,
        ])
}

fn generate(expected: &str, config: Config) {
    generate_test("tests/feature/serde_skip/schema.xsd", expected, config);
}

#[test]
fn generate_disabled() {
    let render = |config| {
        render_with_customizations(
            "tests/feature/serde_skip/schema.xsd",
            config,
            |_, _| {},
            |_| {},
        )
    };
    let quick_xml = render(config(false, false));
    let xml_rs_v7 = render(
        config(false, false)
            .with_generate([(IdentType::Type, "tns:MessageType")])
            .with_serde_xml_rs_config(SerdeXmlRsVersion::Version07AndBelow, false, false, false),
    );
    let xml_rs_v8 = render(
        config(false, false)
            .with_generate([(IdentType::Type, "tns:MessageType")])
            .with_serde_xml_rs_config(SerdeXmlRsVersion::Version08AndAbove, false, false, false),
    );
    generate_test_validate(
        quote! {
            #quick_xml
            pub mod xml_rs_v7 { #xml_rs_v7 }
            pub mod xml_rs_v8 { #xml_rs_v8 }
        }
        .to_string(),
        "tests/feature/serde_skip/expected/disabled.rs",
    );
}

#[test]
fn omission_options_are_independent() {
    for (none, vecs) in [(true, false), (false, true)] {
        let code = render_with_customizations(
            "tests/feature/serde_skip/schema.xsd",
            config(none, vecs),
            |_, _| {},
            |_| {},
        )
        .to_token_stream()
        .to_string();
        assert_eq!(code.contains("Option::is_none"), none, "{code}");
        assert_eq!(code.contains("Vec::is_empty"), vecs, "{code}");
    }
}

#[test]
fn generate_both() {
    generate(
        "tests/feature/serde_skip/expected/both.rs",
        config(true, true),
    );
}

#[test]
fn generate_serde_xml_rs_v7() {
    generate(
        "tests/feature/serde_skip/expected/serde_xml_rs_v7.rs",
        config(true, true).with_render_step(RenderStep::TypesSerdeXmlRs {
            version: SerdeXmlRsVersion::Version07AndBelow,
            large_arrays: false,
            skip_none: true,
            skip_empty_vecs: true,
        }),
    );
}

#[test]
fn generate_serde_xml_rs_v8() {
    generate(
        "tests/feature/serde_skip/expected/serde_xml_rs_v8.rs",
        config(true, true).with_render_step(RenderStep::TypesSerdeXmlRs {
            version: SerdeXmlRsVersion::Version08AndAbove,
            large_arrays: false,
            skip_none: true,
            skip_empty_vecs: true,
        }),
    );
}

fn generate_custom(expected: &str, customize: impl Fn(&mut ComplexDataStruct<'_>)) {
    let module = render_with_customizations(
        "tests/feature/serde_skip/schema.xsd",
        config(true, true),
        |_, _| {},
        |data| {
            for item in data.items.values_mut() {
                if let DataTypeVariant::Complex(ComplexData::Struct { type_, .. }) =
                    &mut item.variant
                {
                    customize(type_);
                }
            }
        },
    );
    generate_test_validate(module.to_token_stream().to_string(), expected);
}

#[test]
fn generate_custom_skip_and_default() {
    generate_custom("tests/feature/serde_skip/expected/custom.rs", |type_| {
        for attribute in &mut type_.attributes {
            if attribute.s_name == "note" {
                attribute.extra_attributes.push(quote!(serde(
                    default = "crate::feature::serde_skip::default_note"
                )));
            }
        }
        if let StructMode::Sequence { elements, .. } = &mut type_.mode {
            for element in elements {
                match element.s_name.as_str() {
                    "Description" => element
                        .extra_attributes
                        .push(quote!(serde(skip_serializing))),
                    "Item" => element.extra_attributes.push(quote!(serde(
                        skip_serializing_if = "crate::feature::serde_skip::keep_empty",
                        default = "crate::feature::serde_skip::default_items"
                    ))),
                    "Required" => element.extra_attributes.push(quote!(serde(
                        default = "crate::feature::serde_skip::default_items"
                    ))),
                    _ => (),
                }
            }
        }
    });
}

fn generate_deserializer(expected: &str, default: bool) {
    aliases::generate_custom(expected, |type_| {
        if let StructMode::Sequence { elements, .. } = &mut type_.mode {
            for element in elements {
                match element.s_name.as_str() {
                    "Optional" => element.extra_attributes.push(quote!(serde(
                        deserialize_with = "crate::feature::serde_skip::deserialize_option"
                    ))),
                    "Items" => element.extra_attributes.push(quote!(serde(
                        deserialize_with = "crate::feature::serde_skip::deserialize_vec"
                    ))),
                    _ => continue,
                }
                if default {
                    element.extra_attributes.push(quote!(serde(default)));
                }
            }
        }
    });
}

#[test]
fn generate_deserialize_override() {
    generate_deserializer(
        "tests/feature/serde_skip/expected/deserialize_override.rs",
        false,
    );
}

#[test]
fn generate_defaulted_deserialize_override() {
    generate_deserializer(
        "tests/feature/serde_skip/expected/defaulted_deserialize_override.rs",
        true,
    );
}

#[cfg(not(feature = "update-expectations"))]
mod runtime {
    use super::round_trip_json;
    mod disabled {
        include!("expected/disabled.rs");
    }
    mod both {
        include!("expected/both.rs");
    }
    mod custom {
        include!("expected/custom.rs");
    }
    mod deserialize_override {
        include!("expected/deserialize_override.rs");
    }
    mod defaulted_deserialize_override {
        include!("expected/defaulted_deserialize_override.rs");
    }
    mod serde_xml_rs_v7 {
        include!("expected/serde_xml_rs_v7.rs");
    }
    mod serde_xml_rs_v8 {
        include!("expected/serde_xml_rs_v8.rs");
    }

    #[test]
    fn settings_omit_only_empty_values() {
        fn check<T: serde::Serialize + serde::de::DeserializeOwned>(omit: bool) {
            for description in [serde_json::Value::Null, serde_json::json!("present")] {
                for items in [serde_json::json!([]), serde_json::json!(["item"])] {
                    let value = serde_json::json!({
                        "@note": description,
                        "@category": "example",
                        "Description": description,
                        "Item": items,
                        "Required": ["value"],
                    });
                    let decoded: T = serde_json::from_value(value.clone()).unwrap();
                    let mut expected = value;
                    if omit && description.is_null() {
                        expected.as_object_mut().unwrap().remove("@note");
                        expected.as_object_mut().unwrap().remove("Description");
                    }
                    if omit && items.as_array().unwrap().is_empty() {
                        expected.as_object_mut().unwrap().remove("Item");
                    }
                    let encoded = serde_json::to_value(decoded).unwrap();
                    assert_eq!(encoded, expected);
                    let decoded: T = serde_json::from_value(encoded.clone()).unwrap();
                    assert_eq!(serde_json::to_value(decoded).unwrap(), encoded);
                }
            }
        }
        check::<disabled::MessageType>(false);
        check::<both::MessageType>(true);
    }

    #[test]
    fn content_fields_are_omitted_and_enum_payloads_are_preserved() {
        let content: both::ContentType = serde_json::from_str("{}").unwrap();
        assert!(content.content.is_empty());
        assert_eq!(serde_json::to_string(&content).unwrap(), "{}");
        assert_eq!(
            serde_json::to_value(both::PayloadType::Values(vec![])).unwrap(),
            serde_json::json!({"Values": []})
        );
    }

    #[test]
    fn custom_skip_predicates_and_defaults_are_preserved() {
        let value = serde_json::json!({
            "@category": "example",
            "Description": "hidden",
            "Item": [],
            "Required": ["value"],
        });
        let decoded: custom::MessageType = serde_json::from_value(value).unwrap();
        let encoded = serde_json::to_value(decoded).unwrap();
        assert!(encoded.get("Description").is_none());
        assert_eq!(encoded["Item"], serde_json::json!([]));
        let decoded: custom::MessageType = serde_json::from_value(serde_json::json!({
            "@category": "example", "Required": ["value"]
        }))
        .unwrap();
        assert_eq!(decoded.item, ["fallback"]);
        let decoded: custom::MessageType =
            serde_json::from_value(serde_json::json!({"@category": "example"})).unwrap();
        assert_eq!(decoded.required, ["fallback"]);
    }

    #[test]
    fn empty_vector_with_custom_default_round_trips() {
        let (original, decoded, encoded) =
            round_trip_json::<custom::MessageType>(serde_json::json!({
                "@note": "present", "@category": "example", "Description": null,
                "Item": [], "Required": [],
            }));
        assert_eq!(decoded.unwrap(), original);
        assert_eq!(encoded["Required"], serde_json::json!([]));
    }

    #[test]
    fn none_with_custom_default_round_trips() {
        let (original, decoded, encoded) =
            round_trip_json::<custom::MessageType>(serde_json::json!({
                "@note": null, "@category": "example", "Description": null,
                "Item": [], "Required": ["value"],
            }));
        assert_eq!(decoded.unwrap(), original);
        assert!(encoded.as_object().unwrap().contains_key("@note"));
    }

    #[test]
    fn none_with_custom_deserializer_round_trips() {
        let (original, decoded, encoded) = round_trip_json::<deserialize_override::AliasesType>(
            serde_json::json!({
                "Optional": null, "Items": ["value"], "BoxedItems": ["value"], "OptionalArray": null,
            }),
        );
        assert_eq!(decoded.unwrap(), original);
        assert!(encoded.as_object().unwrap().contains_key("Optional"));
    }

    #[test]
    fn empty_vector_with_custom_deserializer_round_trips() {
        let (original, decoded, encoded) =
            round_trip_json::<deserialize_override::AliasesType>(serde_json::json!({
                "Optional": "present", "Items": [], "BoxedItems": ["value"], "OptionalArray": null,
            }));
        assert_eq!(decoded.unwrap(), original);
        assert_eq!(encoded["Items"], serde_json::json!([]));
    }

    #[test]
    fn explicit_default_allows_omission_with_custom_deserializer() {
        let (original, decoded, encoded) =
            round_trip_json::<defaulted_deserialize_override::AliasesType>(serde_json::json!({
                "Optional": null, "Items": [], "BoxedItems": [], "OptionalArray": null,
            }));
        assert_eq!(decoded.unwrap(), original);
        assert_eq!(encoded, serde_json::json!({}));
    }

    #[test]
    fn xml_serialization_matches_backend_baselines() {
        use disabled::{xml_rs_v7 as disabled_v7, xml_rs_v8 as disabled_v8};

        macro_rules! sample {
            ($module:ident) => {
                $module::MessageType {
                    note: None,
                    category: "example".into(),
                    description: None,
                    item: vec![],
                    required: vec!["value".into()],
                }
            };
        }
        macro_rules! check {
            ($module:ident, $serialize:path, $deserialize:path, $category:literal, $baseline:expr, $changed:expr) => {{
                let message = sample!($module);
                let xml = $serialize(&message).unwrap();
                if $changed {
                    assert_ne!(xml, $baseline);
                } else {
                    assert_eq!(xml, $baseline);
                }
                assert!(!xml.contains("note="), "{xml}");
                assert!(!xml.contains("Description"), "{xml}");
                assert!(!xml.contains("Item"), "{xml}");
                assert!(xml.contains("Required"), "{xml}");
                let decoded: $module::MessageType = $deserialize(&xml).unwrap();
                assert_eq!(decoded, message);
                let without_category = xml.replace($category, "");
                assert_ne!(without_category, xml);
                let decoded: $module::MessageType = $deserialize(&without_category).unwrap();
                assert_eq!(decoded.category, "example");
            }};
        }
        let baseline = quick_xml::se::to_string(&sample!(disabled)).unwrap();
        let xml_rs_v7_baseline = ::serde_xml_rs_v7::to_string(&sample!(disabled_v7)).unwrap();
        let xml_rs_v8_baseline = serde_xml_rs::to_string(&sample!(disabled_v8)).unwrap();
        assert!(baseline.contains("note=\"\""), "{baseline}");
        assert!(baseline.contains("<Description/>"), "{baseline}");
        assert!(!baseline.contains("<Item"), "{baseline}");
        check!(
            both,
            quick_xml::se::to_string,
            quick_xml::de::from_str,
            " category=\"example\"",
            baseline,
            true
        );
        check!(
            serde_xml_rs_v7,
            ::serde_xml_rs_v7::to_string,
            ::serde_xml_rs_v7::from_str,
            "<category>example</category>",
            xml_rs_v7_baseline,
            true
        );
        check!(
            serde_xml_rs_v8,
            serde_xml_rs::to_string,
            serde_xml_rs::from_str,
            " category=\"example\"",
            xml_rs_v8_baseline,
            false
        );
    }
}

#[cfg(not(feature = "update-expectations"))]
#[allow(clippy::ptr_arg)] // Matches Serde's field type for this custom predicate.
fn keep_empty(_: &Vec<String>) -> bool {
    false
}

#[cfg(not(feature = "update-expectations"))]
fn default_items() -> Vec<String> {
    vec!["fallback".into()]
}

#[cfg(not(feature = "update-expectations"))]
#[allow(clippy::unnecessary_wraps)] // Exercises a custom default that changes None.
fn default_note() -> Option<String> {
    Some("fallback".into())
}

#[cfg(not(feature = "update-expectations"))]
fn deserialize_option<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    serde::Deserialize::deserialize(deserializer)
}

#[cfg(not(feature = "update-expectations"))]
fn deserialize_vec<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<String>, D::Error> {
    serde::Deserialize::deserialize(deserializer)
}

#[cfg(not(feature = "update-expectations"))]
fn round_trip_json<T>(value: serde_json::Value) -> (T, serde_json::Result<T>, serde_json::Value)
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    let original: T = serde_json::from_value(value).unwrap();
    let encoded = serde_json::to_value(&original).unwrap();
    let decoded = serde_json::from_value(encoded.clone());
    (original, decoded, encoded)
}
