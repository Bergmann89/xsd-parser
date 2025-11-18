use xsd_parser::{config::InterpreterFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_interpreter_flags(InterpreterFlags::all())
        .with_generate([(IdentType::Element, "xml:my_name")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/element_without_type/schema.xsd",
        "tests/feature/element_without_type/expected/default.rs",
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
        "tests/feature/element_without_type/schema.xsd",
        "tests/feature/element_without_type/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::MyName;

    let obj = crate::utils::quick_xml_read_test::<MyName, _>(
        "tests/feature/element_without_type/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
