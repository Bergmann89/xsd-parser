use xsd_parser::{config::GeneratorFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_generate([(IdentType::Element, "Container")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/sequence_with_choice_nested/schema.xsd",
        "tests/feature/sequence_with_choice_nested/expected/default.rs",
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
        "tests/feature/sequence_with_choice_nested/schema.xsd",
        "tests/feature/sequence_with_choice_nested/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::{Container, ContainerContent2Type, ContainerContent4Type};

    let obj = crate::utils::quick_xml_read_test::<Container, _>(
        "tests/feature/sequence_with_choice_nested/example/default.xml",
    );

    let mut it = obj.content_2.into_iter();
    let Some(ContainerContent2Type::Content3(cntr)) = it.next() else {
        panic!();
    };
    let mut items = cntr.content_4.into_iter();
    assert!(matches!(items.next(), Some(ContainerContent4Type::Bar(s)) if s == "bar"));

    assert!(matches!(it.next(), Some(ContainerContent2Type::Foo(s)) if s == "foo"));
    assert!(it.next().is_none());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{
        Container, ContainerContent2Type, ContainerContent3Type, ContainerContent4Type,
    };

    let obj = Container {
        content_2: vec![
            ContainerContent2Type::Content3(ContainerContent3Type {
                content_4: vec![ContainerContent4Type::Bar("bar".into())],
            }),
            ContainerContent2Type::Foo("foo".into()),
        ],
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "Container",
        "tests/feature/sequence_with_choice_nested/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
