use crate::utils::generate_test;

#[test]
fn generate_default_en16931() {
    generate_test(
        "tests/schema/factur_x/schema/3 1.07.2 EN16931/Factur-X_1.07.2_EN16931.xsd",
        "tests/schema/factur_x/expected/en16931_default.rs",
        super::config(),
    );
}

#[test]
fn generate_quick_xml_en16931() {
    generate_test(
        "tests/schema/factur_x/schema/3 1.07.2 EN16931/Factur-X_1.07.2_EN16931.xsd",
        "tests/schema/factur_x/expected/en16931_quick_xml.rs",
        super::config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use en16931_quick_xml::CrossIndustryInvoice;

    crate::utils::quick_xml_read_test::<CrossIndustryInvoice, _>(
        "tests/schema/factur_x/examples/en16931.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod en16931_default {
    #![allow(unused_imports)]

    include!("expected/en16931_default.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod en16931_quick_xml {
    #![allow(unused_imports)]

    include!("expected/en16931_quick_xml.rs");
}
