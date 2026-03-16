use xsd_parser::{config::OptimizerFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_optimizer_flags(OptimizerFlags::all())
        .with_nillable_type_support()
        .with_generate([
            (IdentType::Element, "tns:Foo"),
            (IdentType::Element, "tns:NillableFoo"),
            (IdentType::Element, "tns:Bar"),
        ])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/nillable/schema.xsd",
        "tests/feature/nillable/expected/default.rs",
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
        "tests/feature/nillable/schema.xsd",
        "tests/feature/nillable/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use crate::utils::quick_xml_read_test;
    use quick_xml::{Bar, Foo, NillableFoo};

    let obj = quick_xml_read_test::<NillableFoo, _>("tests/feature/nillable/example/nillable.xml");
    assert!(obj.is_none());

    let obj = quick_xml_read_test::<Foo, _>("tests/feature/nillable/example/default.xml");
    assert_eq!(obj.a, 1);
    assert_eq!(&*obj.b, &None);
    assert_eq!(obj.c, None);
    assert_eq!(obj.d.as_deref(), Some(&None));

    // Bar has no nillable elements; verify it can be round-tripped without xmlns:xsi
    let obj = quick_xml_read_test::<Bar, _>("tests/feature/nillable/example/bar.xml");
    assert_eq!(obj.x, 1);
    assert_eq!(obj.y, 2);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use crate::utils::quick_xml_write_test;
    use quick_xml::{Bar, Foo, NillableFoo};
    use xsd_parser_types::xml::Nillable;

    let obj = NillableFoo::nil();
    quick_xml_write_test(&obj, "Foo", "tests/feature/nillable/example/nillable.xml");

    let obj = Foo {
        a: 1,
        b: Nillable::nil(),
        c: None,
        d: Some(Nillable::nil()),
    };
    quick_xml_write_test(&obj, "Foo", "tests/feature/nillable/example/serialize.xml");

    // Bar has no nillable elements; verify xmlns:xsi is NOT emitted even though
    // nillable_type_support is enabled globally.
    let obj = Bar { x: 1, y: 2 };
    quick_xml_write_test(&obj, "Bar", "tests/feature/nillable/example/bar.xml");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
