use xsd_parser::{config::OptimizerFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .without_optimizer_flags(OptimizerFlags::USE_UNRESTRICTED_BASE_TYPE_SIMPLE)
        .with_optimizer_flags(OptimizerFlags::MERGE_ENUM_UNIONS)
        .with_generate([(IdentType::Element, "tns:Foo")])
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/merge_enum_union_multiple_members/schema.xsd",
        "tests/feature/merge_enum_union_multiple_members/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_alpha2() {
    use quick_xml::{CountryCodeType, Foo};

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/merge_enum_union_multiple_members/example/alpha2.xml",
    );

    assert!(matches!(obj.country, CountryCodeType::Alpha2(x) if x.0 == "DE"));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_alpha3() {
    use quick_xml::{CountryCodeType, Foo};

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/merge_enum_union_multiple_members/example/alpha3.xml",
    );

    assert!(matches!(obj.country, CountryCodeType::Alpha3(x) if x.0 == "DEU"));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_named() {
    use quick_xml::{CountryCodeType, Foo};

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/merge_enum_union_multiple_members/example/named.xml",
    );

    assert!(matches!(obj.country, CountryCodeType::Germany));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_invalid() {
    use quick_xml::Foo;

    let result = crate::utils::quick_xml_read_test_result::<Foo, _>(
        "tests/feature/merge_enum_union_multiple_members/example/invalid.xml",
    );

    assert!(result.is_err());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{Alpha2Type, CountryCodeType, Foo};

    let obj = Foo {
        country: CountryCodeType::Alpha2(Alpha2Type("DE".into())),
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Foo",
        "tests/feature/merge_enum_union_multiple_members/example/alpha2.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
