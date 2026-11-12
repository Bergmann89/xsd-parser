use quote::ToTokens;
use xsd_parser::{
    config::SerdeXmlRsVersion,
    models::data::{ComplexData, DataTypeVariant, Occurs, StructMode},
    Config, IdentType,
};

use crate::utils::{generate_test_validate, render_with_customizations, ConfigEx};

#[test]
fn v8_enum_content_uses_scalar_omission_shape() {
    for occurs in [Occurs::Optional, Occurs::DynamicList] {
        let config = Config::test_default()
            .with_generate([(IdentType::Type, "tns:MessageType")])
            .with_derive(["Debug", "PartialEq"])
            .with_serde_xml_rs_config(SerdeXmlRsVersion::Version08AndAbove, false, true, true);
        let code = render_with_customizations(
            "tests/feature/serde_skip/enum_content.xsd",
            config,
            |_, _| {},
            |data| {
                let mut changed = false;
                for item in data.items.values_mut() {
                    if let DataTypeVariant::Complex(ComplexData::Struct { type_, .. }) =
                        &mut item.variant
                    {
                        if let StructMode::Content { content } = &mut type_.mode {
                            content.occurs = occurs;
                            changed = true;
                        }
                    }
                }
                assert!(changed);
            },
        )
        .to_token_stream()
        .to_string();
        assert!(code.contains("pub content : StateTypeValue"), "{code}");
        assert!(!code.contains("skip_serializing_if"), "{code}");
        generate_test_validate(code, "tests/feature/serde_skip/expected/enum_content.rs");
    }
}

#[cfg(not(feature = "update-expectations"))]
mod model {
    include!("expected/enum_content.rs");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn scalar_enum_content_round_trips() {
    let original = model::MessageType {
        id: "example".into(),
        content: model::StateTypeValue::Ready,
    };
    let encoded = serde_json::to_value(&original).unwrap();
    assert_eq!(
        encoded,
        serde_json::json!({"@id": "example", "#text": "READY"})
    );
    let decoded: model::MessageType = serde_json::from_value(encoded).unwrap();
    assert_eq!(decoded, original);
}
