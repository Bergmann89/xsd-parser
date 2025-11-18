use crate::utils::generate_test;

#[test]
fn generate_default_minimum() {
    generate_test(
        "tests/schema/factur_x/schema/0 1.07.2 MINIMUM/Factur-X_1.07.2_MINIMUM.xsd",
        "tests/schema/factur_x/expected/minimum_default.rs",
        super::config(),
    );
}

#[test]
fn generate_quick_xml_minimum() {
    generate_test(
        "tests/schema/factur_x/schema/0 1.07.2 MINIMUM/Factur-X_1.07.2_MINIMUM.xsd",
        "tests/schema/factur_x/expected/minimum_quick_xml.rs",
        super::config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use minimum_quick_xml::CrossIndustryInvoice;

    crate::utils::quick_xml_read_test::<CrossIndustryInvoice, _>(
        "tests/schema/factur_x/examples/minimum.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod minimum_default {
    #![allow(unused_imports)]

    include!("expected/minimum_default.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod minimum_quick_xml {
    #![allow(unused_imports)]

    include!("expected/minimum_quick_xml.rs");
}
