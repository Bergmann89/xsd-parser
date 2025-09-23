use xsd_parser::{
    config::{GeneratorFlags, SerdeXmlRsVersion},
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_generate([(IdentType::Element, "tns:Foo")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/complex_type_with_group/schema.xsd",
        "tests/feature/complex_type_with_group/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/complex_type_with_group/schema.xsd",
        "tests/feature/complex_type_with_group/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::{Foo, FooOuterType};

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/complex_type_with_group/example/default.xml",
    );

    assert!(matches!(obj.outer_1, FooOuterType::Bar(x) if x == "Hello World"));
    assert!(matches!(obj.outer_2, FooOuterType::Baz(3)));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{Foo, FooOuterType};

    let obj = Foo {
        outer_1: FooOuterType::Bar("Hello World".into()),
        outer_2: FooOuterType::Baz(3),
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Foo",
        "tests/feature/complex_type_with_group/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

/* serde_xml_rs */

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/feature/complex_type_with_group/schema.xsd",
        "tests/feature/complex_type_with_group/expected/serde_xml_rs.rs",
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::{Foo, FooTypeContent};

    let obj = crate::utils::serde_xml_rs_read_test::<Foo, _>(
        "tests/feature/complex_type_with_group/example/default.xml",
    );

    let mut it = obj.content.into_iter();

    assert!(matches!(
        it.next(),
        Some(FooTypeContent::Bar(x)) if x == "Hello World"));
    assert!(matches!(it.next(), Some(FooTypeContent::Baz(3))));
    assert!(it.next().is_none());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_serde_xml_rs() {
    use serde_xml_rs::{Foo, FooTypeContent};

    let obj = Foo {
        content: vec![
            FooTypeContent::Bar("Hello World".into()),
            FooTypeContent::Baz(3),
        ],
    };

    crate::utils::serde_xml_rs_write_test::<Foo, _>(
        &obj,
        "tests/feature/complex_type_with_group/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs {
    #![allow(unused_imports)]

    include!("expected/serde_xml_rs.rs");
}

/* serde_quick_xml */

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/complex_type_with_group/schema.xsd",
        "tests/feature/complex_type_with_group/expected/serde_quick_xml.rs",
        config().with_serde_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::{Foo, FooTypeContent};

    let obj = crate::utils::serde_quick_xml_read_test::<Foo, _>(
        "tests/feature/complex_type_with_group/example/default.xml",
    );

    let mut it = obj.content.into_iter();

    assert!(matches!(
        it.next(),
        Some(FooTypeContent::Bar(x)) if x == "Hello World"));
    assert!(matches!(it.next(), Some(FooTypeContent::Baz(3))));
    assert!(it.next().is_none());
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(unused_imports)]

    include!("expected/serde_quick_xml.rs");
}
