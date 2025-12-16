use xsd_parser::{config::InterpreterFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_interpreter_flags(InterpreterFlags::NONZERO_TYPEDEFS)
        .with_generate([(IdentType::Element, "tns:Foo")])
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! check_obj {
    ($obj:expr) => {{
        let obj = $obj;

        assert_eq!(obj.a_int.get(), 123);
        assert_eq!(obj.b_int.get(), -123);
    }};
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! test_obj {
    ($module:ident) => {{
        use core::num::{NonZeroIsize, NonZeroUsize};
        use $module::Foo;

        Foo {
            a_int: NonZeroUsize::new(123).unwrap(),
            b_int: NonZeroIsize::new(-123).unwrap(),
        }
    }};
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/nonzero_int/schema.xsd",
        "tests/feature/nonzero_int/expected/default.rs",
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
        "tests/feature/nonzero_int/schema.xsd",
        "tests/feature/nonzero_int/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/nonzero_int/example/default.xml",
    );

    check_obj!(obj);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    let obj = test_obj!(quick_xml);

    crate::utils::quick_xml_write_test(
        &obj,
        "Foo",
        "tests/feature/nonzero_int/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
