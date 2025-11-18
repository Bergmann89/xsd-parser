use crate::utils::generate_test;

#[test]
fn generate_default_basic() {
    generate_test(
        "tests/schema/factur_x/schema/2 1.07.2 BASIC/Factur-X_1.07.2_BASIC.xsd",
        "tests/schema/factur_x/expected/basic_default.rs",
        super::config(),
    );
}

#[test]
fn generate_quick_xml_basic() {
    generate_test(
        "tests/schema/factur_x/schema/2 1.07.2 BASIC/Factur-X_1.07.2_BASIC.xsd",
        "tests/schema/factur_x/expected/basic_quick_xml.rs",
        super::config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use basic_quick_xml::CrossIndustryInvoice;

    crate::utils::quick_xml_read_test::<CrossIndustryInvoice, _>(
        "tests/schema/factur_x/examples/basic.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod basic_default {
    #![allow(unused_imports)]

    include!("expected/basic_default.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod basic_quick_xml {
    #![allow(unused_imports)]

    include!("expected/basic_quick_xml.rs");
}
