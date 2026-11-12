use quote::{quote, ToTokens};
use xsd_parser::{
    config::{SerdeXmlRsVersion, TypedefMode},
    models::data::{ComplexData, DataTypeVariant},
    Config, IdentType,
};

use crate::utils::{generate_test_validate, render_with_customizations, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generate([(IdentType::Type, "tns:AttributesType")])
        .with_typedef_mode(TypedefMode::Typedef)
        .with_derive(["Debug", "PartialEq"])
}

fn render(config: Config) -> String {
    render_with_customizations(
        "tests/feature/serde_skip/schema_defaults.xsd",
        config,
        |_, _| {},
        |data| {
            for item in data.items.values_mut() {
                if let DataTypeVariant::Complex(ComplexData::Struct { type_, .. }) =
                    &mut item.variant
                {
                    for attribute in &mut type_.attributes {
                        // Nonempty list defaults need a custom value renderer.
                        attribute.default_value = Some(Box::new(quote!(vec![
                            String::from("fallback"),
                            String::from("other"),
                        ])));
                    }
                }
            }
        },
    )
    .to_token_stream()
    .to_string()
}

#[test]
fn generate_quick_xml() {
    generate_test_validate(
        render(config().with_serde_quick_xml_config(false, false, true)),
        "tests/feature/serde_skip/expected/schema_default_quick_xml.rs",
    );
}

#[test]
fn schema_defaults_prevent_omission_in_every_backend() {
    for config in [
        config().with_serde_quick_xml_config(false, false, true),
        config().with_serde_xml_rs_config(SerdeXmlRsVersion::Version07AndBelow, false, false, true),
        config().with_serde_xml_rs_config(SerdeXmlRsVersion::Version08AndAbove, false, false, true),
    ] {
        let code = render(config);
        assert!(
            code.contains("default = \"AttributesType::default_values\""),
            "{code}"
        );
        assert!(!code.contains("skip_serializing_if"), "{code}");
    }
}

#[cfg(not(feature = "update-expectations"))]
mod runtime {
    mod model {
        include!("expected/schema_default_quick_xml.rs");
    }

    #[test]
    fn empty_list_attribute_with_schema_default_round_trips() {
        let value = serde_json::json!({"@values": []});
        let (original, decoded, encoded) =
            super::super::round_trip_json::<model::AttributesType>(value.clone());
        assert_eq!(decoded.unwrap(), original);
        assert_eq!(encoded, value);
        let original = model::AttributesType { values: vec![] };
        let xml = quick_xml::se::to_string(&original).unwrap();
        assert!(xml.contains("values=\"\""), "{xml}");
        let decoded: model::AttributesType = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(decoded, original);
    }
}
