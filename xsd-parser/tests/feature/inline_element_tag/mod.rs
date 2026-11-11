use xsd_parser::{Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_quick_xml()
        .with_generate([(IdentType::Element, "tns:CATALOG")])
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/inline_element_tag/schema.xsd",
        "tests/feature/inline_element_tag/expected/quick_xml.rs",
        config(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_inline_element_with_its_tag() {
    let obj = quick_xml::CatalogProductType { id: "42".into() };

    crate::utils::quick_xml_write_test_default_root(
        &obj,
        "tests/feature/inline_element_tag/example/product.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
