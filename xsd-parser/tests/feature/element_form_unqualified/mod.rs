use xsd_parser::{
    config::{NamespaceIdent, OptimizerFlags},
    pipeline::renderer::NamespaceSerialization,
    Config, IdentType,
};
use xsd_parser_types::misc::Namespace;

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([(
        IdentType::Element,
        Some(NamespaceIdent::Namespace(Namespace::new_const(b"T"))),
        "Root",
    )])
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! check_obj {
    ($obj:expr) => {{
        let obj = $obj;

        assert_eq!(obj.lang.as_deref(), Some("de"));
        assert_eq!(obj.attr, Some(1));
        assert_eq!(obj.child.value, "v");
        assert_eq!(obj.item, "i");
        assert_eq!(obj.q, "q");
        assert_eq!(obj.ref_.v, "r");
        assert_eq!(obj.ref_2.v, "r2");
    }};
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! test_obj {
    ($module:ident) => {{
        use $module::{ChildType, RefType, Root, RootRef2Type};

        Root {
            lang: Some("de".into()),
            attr: Some(1),
            child: ChildType { value: "v".into() },
            item: "i".into(),
            q: "q".into(),
            ref_: RefType { v: "r".into() },
            ref_2: RootRef2Type { v: "r2".into() },
        }
    }};
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/element_form_unqualified/schema.xsd",
        "tests/feature/element_form_unqualified/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    let obj = crate::utils::quick_xml_read_test::<quick_xml::Root, _>(
        "tests/feature/element_form_unqualified/example/default.xml",
    );

    check_obj!(obj);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    crate::utils::quick_xml_write_test(
        &test_obj!(quick_xml),
        "tns:Root",
        "tests/feature/element_form_unqualified/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

/* quick_xml_dynamic */

#[test]
fn generate_quick_xml_dynamic() {
    generate_test(
        "tests/feature/element_form_unqualified/schema.xsd",
        "tests/feature/element_form_unqualified/expected/quick_xml_dynamic.rs",
        config().with_quick_xml_config(NamespaceSerialization::Dynamic, None, false, false),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_dynamic() {
    crate::utils::quick_xml_write_test(
        &test_obj!(quick_xml_dynamic),
        "tns:Root",
        "tests/feature/element_form_unqualified/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_dynamic {
    #![allow(unused_imports)]

    include!("expected/quick_xml_dynamic.rs");
}

/* quick_xml_optimized */

#[test]
fn generate_quick_xml_optimized() {
    generate_test(
        "tests/feature/element_form_unqualified/schema.xsd",
        "tests/feature/element_form_unqualified/expected/quick_xml_optimized.rs",
        config()
            .with_quick_xml()
            .with_optimizer_flags(OptimizerFlags::all()),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_optimized() {
    let obj = crate::utils::quick_xml_read_test::<quick_xml_optimized::Root, _>(
        "tests/feature/element_form_unqualified/example/default.xml",
    );

    check_obj!(obj);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_optimized() {
    crate::utils::quick_xml_write_test(
        &test_obj!(quick_xml_optimized),
        "tns:Root",
        "tests/feature/element_form_unqualified/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_optimized {
    #![allow(unused_imports)]

    include!("expected/quick_xml_optimized.rs");
}
