use xsd_parser::{config::TypedefMode, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_typedef_mode(TypedefMode::NewType)
        .with_generate([
            (IdentType::Element, "tns:Foo"),
            (IdentType::Element, "tns:Holder"),
        ])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/tuple_with_complex/schema.xsd",
        "tests/feature/tuple_with_complex/expected/default.rs",
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
        "tests/feature/tuple_with_complex/schema.xsd",
        "tests/feature/tuple_with_complex/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/tuple_with_complex/example/default.xml",
    );

    assert_eq!(obj.0.name, "abc");
    assert_eq!(obj.0.value, 42);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_holder() {
    use quick_xml::{Holder, MemberType};

    let obj = crate::utils::quick_xml_read_test::<Holder, _>(
        "tests/feature/tuple_with_complex/example/holder.xml",
    );

    let member = obj.0.head.0.as_any().downcast_ref::<MemberType>().unwrap();
    assert_eq!(member.info, "xyz");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{Foo, FooType};

    let obj = Foo(FooType {
        name: "abc".into(),
        value: 42,
    });

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Foo",
        "tests/feature/tuple_with_complex/example/default.xml",
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_holder() {
    use quick_xml::{Head, Holder, HolderType, MemberType};

    let obj = Holder(HolderType {
        head: Head::new(MemberType { info: "xyz".into() }),
    });

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Holder",
        "tests/feature/tuple_with_complex/example/holder.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

/* quick_xml boxed */

#[test]
fn generate_quick_xml_boxed() {
    generate_test(
        "tests/feature/tuple_with_complex/schema.xsd",
        "tests/feature/tuple_with_complex/expected/quick_xml_boxed.rs",
        config().with_quick_xml_deserialize_config(true, false),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_boxed() {
    use quick_xml_boxed::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/tuple_with_complex/example/default.xml",
    );

    assert_eq!(obj.0.name, "abc");
    assert_eq!(obj.0.value, 42);
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_boxed {
    #![allow(unused_imports)]

    include!("expected/quick_xml_boxed.rs");
}

/* quick_xml skip unknown elements */

#[test]
fn generate_quick_xml_skip_unknown_elements() {
    generate_test(
        "tests/feature/tuple_with_complex/schema.xsd",
        "tests/feature/tuple_with_complex/expected/quick_xml_skip_unknown_elements.rs",
        config().with_quick_xml_deserialize_config(false, true),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_skip_unknown_elements() {
    use xsd_parser_types::quick_xml::{DeserializeSync, SliceReader};

    use quick_xml_skip_unknown_elements::Foo;

    let obj = Foo::deserialize(&mut SliceReader::new(
        "<tns:Foo xmlns:tns=\"http://example.com\"><tns:Z/><tns:Name>abc</tns:Name><tns:Z><tns:Name/></tns:Z><tns:Value>42</tns:Value></tns:Foo>",
    ))
    .unwrap();

    assert_eq!(obj.0.name, "abc");
    assert_eq!(obj.0.value, 42);
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_skip_unknown_elements {
    #![allow(unused_imports)]

    include!("expected/quick_xml_skip_unknown_elements.rs");
}
