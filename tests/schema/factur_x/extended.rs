use crate::utils::generate_test;

#[test]
fn generate_default_extended() {
    generate_test(
        "tests/schema/factur_x/schema/4 1.07.2 EXTENDED/Factur-X_1.07.2_EXTENDED.xsd",
        "tests/schema/factur_x/expected/extended_default.rs",
        super::config(),
    );
}

#[test]
fn generate_quick_xml_extended() {
    generate_test(
        "tests/schema/factur_x/schema/4 1.07.2 EXTENDED/Factur-X_1.07.2_EXTENDED.xsd",
        "tests/schema/factur_x/expected/extended_quick_xml.rs",
        super::config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use extended_quick_xml::CrossIndustryInvoice;

    crate::utils::quick_xml_read_test::<CrossIndustryInvoice, _>(
        "tests/schema/factur_x/examples/extended.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod extended_default {
    #![allow(unused_imports)]

    include!("expected/extended_default.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod extended_quick_xml {
    #![allow(unused_imports)]

    include!("expected/extended_quick_xml.rs");
}
