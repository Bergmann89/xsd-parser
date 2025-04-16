use crate::utils::generate_test;

#[test]
fn generate_default_cii_d22b() {
    generate_test(
        "tests/schema/factur_x/schema/5 CII D22B XSD/CrossIndustryInvoice_100pD22B.xsd",
        "tests/schema/factur_x/expected/cii_d22b_default.rs",
        super::config(),
    );
}

#[test]
fn generate_quick_xml_cii_d22b() {
    generate_test(
        "tests/schema/factur_x/schema/5 CII D22B XSD/CrossIndustryInvoice_100pD22B.xsd",
        "tests/schema/factur_x/expected/cii_d22b_quick_xml.rs",
        super::config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use cii_d22b_quick_xml::CrossIndustryInvoice;

    crate::utils::quick_xml_read_test::<CrossIndustryInvoice, _>(
        "tests/schema/factur_x/examples/xrechnung.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod cii_d22b_default {
    #![allow(unused_imports)]

    include!("expected/cii_d22b_default.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod cii_d22b_quick_xml {
    #![allow(unused_imports)]

    include!("expected/cii_d22b_quick_xml.rs");
}
