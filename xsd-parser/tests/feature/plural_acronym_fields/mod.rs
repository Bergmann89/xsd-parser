use xsd_parser::{
    config::{Schema, SerdeXmlRsVersion},
    models::{normalize_plural_acronyms, ExplicitNaming, Naming},
    traits::Naming as _,
    Config, IdentType,
};

use crate::utils::ConfigEx;

#[test]
fn public_normalization_preserves_word_boundaries_and_handles_digits() {
    for (input, normalized) in [
        ("IDs", "Ids"),
        ("MP3s", "Mp3s"),
        ("IPV4s", "Ipv4s"),
        ("IPv4s", "IPv4s"),
        ("GPS", "GPS"),
        ("HTTPS", "HTTPS"),
        ("STATUS", "STATUS"),
        ("Address", "Address"),
        ("HTTPServer", "HTTPServer"),
        ("IDStatus", "IDStatus"),
        ("123s", "123s"),
        ("MP3status", "MP3status"),
    ] {
        assert_eq!(normalize_plural_acronyms(input), normalized, "{input}");
    }
}

#[test]
fn plural_acronyms_stay_together_in_field_names() {
    for naming in [
        Box::new(Naming::default()) as Box<dyn xsd_parser::traits::Naming>,
        Box::new(ExplicitNaming::default()),
        Box::new(ExplicitNaming::default().unify_names(false)),
    ] {
        for (schema_name, field_name) in [
            ("IDs", "ids"),
            ("UserIDs", "user_ids"),
            ("ProductIDs", "product_ids"),
            ("IDsForUser", "ids_for_user"),
            ("Related_IDs", "related_ids"),
            ("NumberOfAPIs", "number_of_apis"),
            ("UUIDs", "uuids"),
            ("URIs", "uris"),
            ("URLsForIDs", "urls_for_ids"),
            ("IDs_mut", "ids_mut"),
            ("UserID", "user_id"),
            ("IDStatus", "id_status"),
            ("URLSettings", "url_settings"),
            ("XMLSchema", "xml_schema"),
            // Preserve the case converter's existing handling of mixed-case IPv4.
            ("IPv4s", "i_pv_4s"),
            ("Type", "type_"),
        ] {
            assert_eq!(
                naming.format_field_name(schema_name),
                field_name,
                "{schema_name} with {naming:?}"
            );
        }
    }
}

#[test]
fn explicit_field_postfixes_preserve_plural_acronyms() {
    let naming = ExplicitNaming::default()
        .with_element_field_postfix("Element")
        .with_attribute_field_postfix("Attribute");

    assert_eq!(naming.format_element_field_name("IDs"), "ids_element");
    assert_eq!(naming.format_attribute_field_name("URIs"), "uris_attribute");
}

#[test]
fn generated_elements_and_attributes_use_plural_acronym_field_names() {
    for config in [
        Config::test_default(),
        Config::test_default().with_naming(ExplicitNaming::default()),
        Config::test_default().with_quick_xml(),
        Config::test_default().with_serde_quick_xml(),
        Config::test_default().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    ] {
        let code = xsd_parser::generate(
            config
                .with_schema(Schema::schema(include_str!("schema.xsd")))
                .with_generate([(IdentType::Element, "tns:Response")]),
        )
        .unwrap()
        .to_string();

        for field_name in [
            "user_ids",
            "number_of_apis",
            "id_status",
            "resource_urls",
            "i_pv_4s",
        ] {
            assert!(
                code.contains(&format!("pub {field_name} :")),
                "missing field {field_name} in generated code: {code}"
            );
        }
    }
}
