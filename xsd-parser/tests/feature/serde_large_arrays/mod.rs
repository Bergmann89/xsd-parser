use quote::ToTokens;
use xsd_parser::{
    config::{GeneratorFlags, RenderStep, SerdeXmlRsVersion},
    Config, IdentType,
};

use crate::utils::{generate_test, render_with_customizations, with_generated_data, ConfigEx};

mod overrides;
mod wrappers;
mod xml_wrappers;

#[test]
fn plain_and_quick_xml_only_steps_do_not_emit_serde_attributes() {
    for config in [
        config().with_render_step(RenderStep::Types),
        config()
            .with_render_step(RenderStep::Types)
            .with_quick_xml(),
    ] {
        let code = render_with_customizations(
            "tests/feature/serde_large_arrays/schema.xsd",
            config,
            |_, _| {},
            |_| {},
        )
        .to_token_stream()
        .to_string();
        assert!(code.contains("pub struct PacketType"), "{code}");
        assert!(!code.contains("# [serde"), "{code}");
    }
}

fn config() -> Config {
    Config::test_default()
        .with_generate([
            (IdentType::Element, "tns:Packet"),
            (IdentType::Type, "tns:NamedArrayType"),
        ])
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_derive(["Debug", "PartialEq"])
        .with_render_steps([
            RenderStep::TypesSerdeQuickXml {
                large_arrays: true,
                skip_none: false,
                skip_empty_vecs: false,
            },
            RenderStep::Defaults,
        ])
}

fn generate(expected: &str, config: Config) {
    generate_test(
        "tests/feature/serde_large_arrays/schema.xsd",
        expected,
        config,
    );
}

#[test]
fn generate_default() {
    generate(
        "tests/feature/serde_large_arrays/expected/default.rs",
        config().with_render_step(RenderStep::Types),
    );
}

#[test]
fn generate_serde_quick_xml() {
    generate(
        "tests/feature/serde_large_arrays/expected/serde_quick_xml.rs",
        config().with_serde_quick_xml_config(true, false, false),
    );
}

#[test]
fn generate_serde_xml_rs_v7() {
    generate(
        "tests/feature/serde_large_arrays/expected/serde_xml_rs_v7.rs",
        config().with_serde_xml_rs_config(SerdeXmlRsVersion::Version07AndBelow, true, false, false),
    );
}

#[test]
fn generate_serde_xml_rs_v8() {
    generate(
        "tests/feature/serde_large_arrays/expected/serde_xml_rs_v8.rs",
        config().with_serde_xml_rs_config(SerdeXmlRsVersion::Version08AndAbove, true, false, false),
    );
}

#[test]
fn serde_helpers_leave_large_arrays_disabled() {
    use xsd_parser::config::Schema;

    for config in [
        config().with_serde_quick_xml(),
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version07AndBelow),
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    ] {
        let code = xsd_parser::generate(
            config.with_schema(Schema::file("tests/feature/serde_large_arrays/schema.xsd")),
        )
        .unwrap()
        .to_string();
        assert!(!code.contains("serde_with"), "{code}");
    }
}

#[test]
fn configured_and_direct_serde_steps_use_the_same_options() {
    use xsd_parser::{
        config::OptimizerFlags,
        exec_render,
        pipeline::renderer::{
            DefaultsRenderStep, RenderStep as RendererStep, SerdeQuickXmlTypesRenderStep,
            SerdeXmlRsV7TypesRenderStep, SerdeXmlRsV8TypesRenderStep,
        },
        Renderer,
    };

    for (schema, setup) in [
        ("tests/feature/serde_large_arrays/schema.xsd", config()),
        (
            "tests/feature/serde_skip/schema.xsd",
            config().with_generate([(IdentType::Type, "tns:MessageType")]),
        ),
    ] {
        with_generated_data(
            schema,
            setup.clone(),
            |_, _| {},
            |data, _| {
                let steps: [(Config, Box<dyn RendererStep>); 3] = [
                    (
                        setup.clone().with_serde_quick_xml_config(true, true, true),
                        Box::new(SerdeQuickXmlTypesRenderStep {
                            large_arrays: true,
                            skip_none: true,
                            skip_empty_vecs: true,
                        }),
                    ),
                    (
                        setup.clone().with_serde_xml_rs_config(
                            SerdeXmlRsVersion::Version07AndBelow,
                            true,
                            true,
                            true,
                        ),
                        Box::new(SerdeXmlRsV7TypesRenderStep {
                            large_arrays: true,
                            skip_none: true,
                            skip_empty_vecs: true,
                        }),
                    ),
                    (
                        setup.clone().with_serde_xml_rs_config(
                            SerdeXmlRsVersion::Version08AndAbove,
                            true,
                            true,
                            true,
                        ),
                        Box::new(SerdeXmlRsV8TypesRenderStep {
                            large_arrays: true,
                            skip_none: true,
                            skip_empty_vecs: true,
                        }),
                    ),
                ];
                for (config, direct_step) in steps {
                    assert!(config.optimizer.flags.contains(OptimizerFlags::SERDE));
                    let configured = exec_render(config.renderer, data).unwrap();
                    let direct = Renderer::new(data)
                        .derive(["Debug", "PartialEq"])
                        .with_step_boxed(direct_step)
                        .with_step(DefaultsRenderStep)
                        .finish();
                    assert_eq!(
                        configured.to_token_stream().to_string(),
                        direct.to_token_stream().to_string()
                    );
                }
            },
        );
    }
}

#[test]
fn generate_disabled() {
    let mut config = config();
    config = config.with_render_step(RenderStep::Types);
    config.renderer.flags = xsd_parser::config::RendererFlags::all();
    generate(
        "tests/feature/serde_large_arrays/expected/disabled.rs",
        config,
    );
}

#[cfg(not(feature = "update-expectations"))]
mod disabled {
    include!("expected/disabled.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    include!("expected/default.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    include!("expected/serde_quick_xml.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs_v7 {
    include!("expected/serde_xml_rs_v7.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs_v8 {
    include!("expected/serde_xml_rs_v8.rs");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn round_trip_named_fields_and_enum_payloads() {
    let packet = serde_quick_xml::PacketType {
        sample: std::array::from_fn(|i| format!("sample-{i}")),
        small: [7; 32],
        threshold: [8; 33],
        choice: serde_quick_xml::ChoiceType::Number([9; 64]),
    };
    let encoded = serde_json::to_value(&packet).unwrap();
    assert_eq!(encoded["Sample"].as_array().unwrap().len(), 64);
    assert_eq!(encoded["Small"].as_array().unwrap().len(), 32);
    assert_eq!(encoded["Threshold"].as_array().unwrap().len(), 33);
    assert_eq!(encoded["Choice"]["Number"].as_array().unwrap().len(), 64);
    let decoded: serde_quick_xml::PacketType = serde_json::from_value(encoded).unwrap();
    assert_eq!(decoded, packet);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn rejects_incorrect_lengths() {
    for size in [0, 63, 65] {
        let value = serde_json::json!({"Number": vec![7; size]});
        assert!(serde_json::from_value::<serde_quick_xml::ChoiceType>(value).is_err());
    }
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn serde_xml_models_round_trip_as_json() {
    fn check<T: serde::Serialize + serde::de::DeserializeOwned>(prefix: &str) {
        let value = serde_json::json!({
            (format!("{prefix}Sample")): vec!["sample"; 64],
            (format!("{prefix}Small")): vec![7; 32],
            (format!("{prefix}Threshold")): vec![8; 33],
            (format!("{prefix}Choice")): {(format!("{prefix}Number")): vec![9; 64]},
        });
        let decoded: T = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), value);
    }
    check::<serde_quick_xml::PacketType>("");
    check::<serde_xml_rs_v7::PacketType>("");
    check::<serde_xml_rs_v8::PacketType>("tns:");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn reads_and_writes_large_named_arrays_with_serde_quick_xml() {
    let packet = serde_quick_xml::NamedArrayType {
        sample: std::array::from_fn(|index| format!("sample-{index}")),
        threshold: [8; 33],
    };
    let xml = quick_xml::se::to_string(&packet).unwrap();
    let decoded: serde_quick_xml::NamedArrayType = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(decoded, packet);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn reads_large_named_arrays_with_serde_xml_rs_v7() {
    let packet = serde_quick_xml::NamedArrayType {
        sample: std::array::from_fn(|index| format!("sample-{index}")),
        threshold: [8; 33],
    };
    let xml = quick_xml::se::to_string(&packet).unwrap();
    let decoded: serde_xml_rs_v7::NamedArrayType = ::serde_xml_rs_v7::from_str(&xml).unwrap();
    assert_eq!(decoded.sample, packet.sample);
    assert_eq!(decoded.threshold, packet.threshold);
}

#[test]
fn converted_fixed_arrays_receive_vector_omission_rules() {
    for large_arrays in [false, true] {
        let code = render_with_customizations(
            "tests/feature/serde_large_arrays/schema.xsd",
            config().with_serde_xml_rs_config(
                SerdeXmlRsVersion::Version08AndAbove,
                large_arrays,
                true,
                true,
            ),
            |_, _| {},
            |_| {},
        )
        .to_token_stream()
        .to_string();
        // All five named array fields become vectors; the enum payload is preserved.
        assert_eq!(code.matches("skip_serializing_if").count(), 5, "{code}");
        assert_eq!(code.matches("Vec::is_empty").count(), 5, "{code}");
        assert!(!code.contains("serde_with"), "{code}");
    }
}
