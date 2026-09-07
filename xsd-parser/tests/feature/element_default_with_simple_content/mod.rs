use xsd_parser::{Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([(IdentType::Element, "Foo")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/element_default_with_simple_content/schema.xsd",
        "tests/feature/element_default_with_simple_content/expected/default.rs",
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
        "tests/feature/element_default_with_simple_content/schema.xsd",
        "tests/feature/element_default_with_simple_content/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_empty_element_uses_default() {
    use quick_xml::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/element_default_with_simple_content/example/empty.xml",
    );

    let bar = obj.bar.expect("bar should be present");
    assert!(!bar.content, "expected default value false");
    assert_eq!(bar.baz, "xxxx");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_empty_explicit_element_uses_default() {
    use quick_xml::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/element_default_with_simple_content/example/empty_explicit.xml",
    );

    let bar = obj.bar.expect("bar should be present");
    assert!(
        !bar.content,
        "expected default value false for explicit empty tags"
    );
    assert_eq!(bar.baz, "xxxx");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_with_content() {
    use quick_xml::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/element_default_with_simple_content/example/with_content.xml",
    );

    let bar = obj.bar.expect("bar should be present");
    assert!(bar.content, "expected explicit value true");
    assert_eq!(bar.baz, "xxxx");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
