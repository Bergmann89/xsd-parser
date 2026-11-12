use quote::{quote, ToTokens};
use xsd_parser::models::data::{ComplexData, DataTypeVariant, StructMode};

use crate::utils::{generate_test_validate, render_with_customizations};

fn generate(expected: &str, serialize_only: bool) {
    let module = render_with_customizations(
        "tests/feature/serde_large_arrays/schema.xsd",
        super::config(),
        |_, _| {},
        |data| {
            for item in data.items.values_mut() {
                if let DataTypeVariant::Complex(ComplexData::Struct { type_, .. }) =
                    &mut item.variant
                {
                    if let StructMode::Sequence { elements, .. } = &mut type_.mode {
                        for element in elements {
                            let attribute = match element.s_name.as_str() {
                                "Sample" => Some(quote!(serde(
                                    with = "crate::feature::serde_large_arrays::overrides::codec"
                                ))),
                                "Threshold" if serialize_only => Some(quote!(serde(
                                    serialize_with =
                                        "crate::feature::serde_large_arrays::overrides::codec::serialize"
                                ))),
                                "Threshold" => Some(quote!(serde(
                                    deserialize_with =
                                        "crate::feature::serde_large_arrays::overrides::codec::deserialize"
                                ))),
                                _ => None,
                            };
                            element.extra_attributes.extend(attribute);
                        }
                    }
                }
            }
        },
    );
    generate_test_validate(module.to_token_stream().to_string(), expected);
}

#[test]
fn generate_serialize_override() {
    generate(
        "tests/feature/serde_large_arrays/expected/serialize_override.rs",
        true,
    );
}

#[test]
fn generate_deserialize_override() {
    generate(
        "tests/feature/serde_large_arrays/expected/deserialize_override.rs",
        false,
    );
}

#[cfg(not(feature = "update-expectations"))]
mod serialize_override {
    include!("expected/serialize_override.rs");
}
#[cfg(not(feature = "update-expectations"))]
mod deserialize_override {
    include!("expected/deserialize_override.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod codec {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<T: Serialize, S: Serializer, const N: usize>(
        value: &[T; N],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serde_with::As::<[serde_with::Same; N]>::serialize(value, serializer)
    }

    pub fn deserialize<'de, T: Deserialize<'de>, D: Deserializer<'de>, const N: usize>(
        deserializer: D,
    ) -> Result<[T; N], D::Error> {
        serde_with::As::<[serde_with::Same; N]>::deserialize(deserializer)
    }
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn manual_overrides_coexist_with_automatic_adapters() {
    fn check<T: serde::Serialize + serde::de::DeserializeOwned>() {
        let value = serde_json::json!({
            "Sample": vec!["sample"; 64],
            "Small": vec![7; 32],
            "Threshold": vec![8; 33],
            "Choice": {"Number": vec![9; 64]},
        });
        let decoded: T = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), value);
    }
    check::<serialize_override::PacketType>();
    check::<deserialize_override::PacketType>();
}
