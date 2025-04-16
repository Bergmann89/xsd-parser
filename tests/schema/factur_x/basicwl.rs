use crate::utils::generate_test;

#[test]
fn generate_default_basicwl() {
    generate_test(
        "tests/schema/factur_x/schema/1 1.07.2 BASICWL/Factur-X_1.07.2_BASICWL.xsd",
        "tests/schema/factur_x/expected/basicwl_default.rs",
        super::config(),
    );
}

#[test]
fn generate_quick_xml_basicwl() {
    generate_test(
        "tests/schema/factur_x/schema/1 1.07.2 BASICWL/Factur-X_1.07.2_BASICWL.xsd",
        "tests/schema/factur_x/expected/basicwl_quick_xml.rs",
        super::config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use basicwl_quick_xml::CrossIndustryInvoice;

    crate::utils::quick_xml_read_test::<CrossIndustryInvoice, _>(
        "tests/schema/factur_x/examples/basicwl.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod basicwl_default {
    #![allow(unused_imports)]

    include!("expected/basicwl_default.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod basicwl_quick_xml {
    #![allow(unused_imports)]

    include!("expected/basicwl_quick_xml.rs");
}
