use xsd_parser::{
    config::{GeneratorFlags, InterpreterFlags, SerdeXmlRsVersion},
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_interpreter_flags(InterpreterFlags::WITH_NUM_BIG_INT)
        .with_generator_flags(GeneratorFlags::USE_MODULES)
        .with_generate([(IdentType::Element, "tns:Foo")])
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! check_obj {
    ($obj:expr) => {{
        use num::BigInt;
        use std::str::FromStr;

        let obj = $obj;

        assert_eq!(obj.a_int, BigInt::from(123));
        assert_eq!(
            obj.b_int,
            BigInt::from_str("1000000000000000000000000").unwrap()
        );
    }};
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! test_obj {
    ($module:ident) => {{
        use num::BigInt;
        use std::str::FromStr;
        use $module::tns::Foo;

        Foo {
            a_int: BigInt::from(123),
            b_int: BigInt::from_str("1000000000000000000000000").unwrap(),
        }
    }};
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/num_big_int/schema.xsd",
        "tests/feature/num_big_int/expected/default.rs",
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
        "tests/feature/num_big_int/schema.xsd",
        "tests/feature/num_big_int/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::tns::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/num_big_int/example/default.xml",
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
        "tests/feature/num_big_int/example/serialize.xml",
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
        "tests/feature/num_big_int/schema.xsd",
        "tests/feature/num_big_int/expected/serde_xml_rs.rs",
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::tns::Foo;

    let obj = crate::utils::serde_xml_rs_read_test::<Foo, _>(
        "tests/feature/num_big_int/example/default.xml",
    );

    check_obj!(obj);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_serde_xml_rs() {
    let obj = test_obj!(serde_xml_rs);

    crate::utils::serde_xml_rs_write_test(&obj, "tests/feature/num_big_int/example/serialize.xml");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs {
    #![allow(unused_imports)]

    include!("expected/serde_xml_rs.rs");
}
