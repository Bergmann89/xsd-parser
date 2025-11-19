use xsd_parser::{config::SerdeXmlRsVersion, Config};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/empty_string/schema.xsd",
        "tests/feature/empty_string/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::{ComplexContent, SimpleContent};

    let obj = crate::utils::quick_xml_read_test::<SimpleContent, _>(
        "tests/feature/empty_string/example/simple.xml",
    );

    assert_eq!(obj.lang, "");
    assert_eq!(obj.content, "");

    let obj = crate::utils::quick_xml_read_test::<ComplexContent, _>(
        "tests/feature/empty_string/example/complex.xml",
    );

    assert_eq!(obj.lang, "");
    assert_eq!(obj.content, "");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{ComplexContent, SimpleContent};

    let obj = SimpleContent {
        lang: "".into(),
        content: "".into(),
    };
    crate::utils::quick_xml_write_test::<SimpleContent, _>(
        &obj,
        "SimpleContent",
        "tests/feature/empty_string/example/simple.xml",
    );

    let obj = ComplexContent {
        lang: "".into(),
        content: "".into(),
    };
    crate::utils::quick_xml_write_test::<ComplexContent, _>(
        &obj,
        "ComplexContent",
        "tests/feature/empty_string/example/complex.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

/* serde_xml_rs v0.7 */

#[test]
fn generate_serde_xml_rs_v7() {
    generate_test(
        "tests/feature/empty_string/schema.xsd",
        "tests/feature/empty_string/expected/serde_xml_rs_v7.rs",
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version07AndBelow),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs_v7() {
    use serde_xml_rs_v7::{ComplexContent, SimpleContent};

    let obj = crate::utils::serde_xml_rs_v7_read_test::<SimpleContent, _>(
        "tests/feature/empty_string/example/simple.xml",
    );

    assert_eq!(obj.lang, "");
    assert_eq!(obj.content, "");

    let obj = crate::utils::serde_xml_rs_v7_read_test::<ComplexContent, _>(
        "tests/feature/empty_string/example/complex.xml",
    );

    assert_eq!(obj.lang, "");
    assert_eq!(obj.content, "");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs_v7 {
    #![allow(unused_imports)]

    include!("expected/serde_xml_rs_v7.rs");
}

/* serde_xml_rs v0.8 */

#[test]
fn generate_serde_xml_rs_v8() {
    generate_test(
        "tests/feature/empty_string/schema.xsd",
        "tests/feature/empty_string/expected/serde_xml_rs_v8.rs",
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs_v8() {
    use serde_xml_rs_v8::{ComplexContent, SimpleContent};

    let obj = crate::utils::serde_xml_rs_read_test::<SimpleContent, _>(
        "tests/feature/empty_string/example/simple.xml",
    );

    assert_eq!(obj.lang, "");
    assert_eq!(obj.content, "");

    let obj = crate::utils::serde_xml_rs_read_test::<ComplexContent, _>(
        "tests/feature/empty_string/example/complex.xml",
    );

    assert_eq!(obj.lang, "");
    assert_eq!(obj.content, "");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs_v8 {
    #![allow(unused_imports)]

    include!("expected/serde_xml_rs_v8.rs");
}

/* serde_quick_xml */

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/empty_string/schema.xsd",
        "tests/feature/empty_string/expected/serde_quick_xml.rs",
        config().with_serde_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::{ComplexContent, SimpleContent};

    let obj = crate::utils::serde_quick_xml_read_test::<SimpleContent, _>(
        "tests/feature/empty_string/example/simple.xml",
    );

    assert_eq!(obj.lang, "");
    assert_eq!(obj.content, "");

    let obj = crate::utils::serde_quick_xml_read_test::<ComplexContent, _>(
        "tests/feature/empty_string/example/complex.xml",
    );

    assert_eq!(obj.lang, "");
    assert_eq!(obj.content, "");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(unused_imports)]

    include!("expected/serde_quick_xml.rs");
}
