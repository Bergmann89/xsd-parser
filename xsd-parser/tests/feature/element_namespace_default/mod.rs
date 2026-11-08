use xsd_parser::{
    config::{GeneratorFlags, InterpreterFlags, NamespaceIdent},
    pipeline::renderer::NamespaceSerialization,
    Config, IdentType,
};
use xsd_parser_types::misc::Namespace;

use crate::utils::{generate_test, ConfigEx};

const NS_T: Namespace = Namespace::new_const(b"T");

fn config(mode: NamespaceSerialization, default_namespace: Option<Namespace>) -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_interpreter_flags(InterpreterFlags::WITH_XS_ANY_SIMPLE_TYPE)
        .with_nillable_type_support()
        .with_any_type_support()
        .with_generate([(
            IdentType::Element,
            Some(NamespaceIdent::Namespace(NS_T)),
            "Foo",
        )])
        .with_quick_xml_config(mode, default_namespace, false, false)
        .with_xs_any_simple_type("xsd_parser_types::xml::AnySimpleType")
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! check_obj {
    ($module:ident, $obj:expr) => {{
        use xsd_parser_types::xml::AnySimpleType;
        use $module::CircleType;

        let obj = $obj;

        assert_eq!(obj.child.as_ref().unwrap().value, "v");
        assert_eq!(obj.item, ["a", "b"]);
        assert_eq!(obj.ref_, "r");
        let circle = obj.shape.0.as_any().downcast_ref::<CircleType>().unwrap();
        assert_eq!(circle.radius, 1);
        assert!(obj.n.is_nil());
        assert!(matches!(&obj.any_value, AnySimpleType::String(x) if x == "s"));
        assert_eq!(obj.node.name, "a");
        let inner = obj.node.node.as_ref().unwrap();
        assert_eq!(inner.name, "b");
        assert!(inner.node.is_none());
        assert_eq!(obj.any.name.as_ref(), b"other");
        assert_eq!(obj.any.values.len(), 1);
    }};
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! test_obj {
    ($module:ident) => {{
        use xsd_parser_types::{
            quick_xml::BytesText,
            xml::{AnySimpleType, Element, Nillable, Value},
        };
        use $module::{ChildType, CircleType, FooType, NodeType, Shape};

        FooType {
            child: Some(ChildType { value: "v".into() }),
            item: vec!["a".into(), "b".into()],
            ref_: "r".into(),
            shape: Shape::new(CircleType { radius: 1 }),
            n: Nillable::nil(),
            any_value: AnySimpleType::String("s".into()),
            node: NodeType {
                name: "a".into(),
                node: Some(Box::new(NodeType {
                    name: "b".into(),
                    node: None,
                })),
            },
            any: Element::default()
                .name(b"other")
                .child(Value::Text(BytesText::new("o"))),
        }
    }};
}

/// One test case: generate the snapshot, write the object with `root` and
/// compare it to `xml`, and read every file in `read` back into the object.
macro_rules! case {
    ($module:ident, $xsd:literal, $expected:literal, $config:expr, $root:literal, $xml:literal, [$($read:literal),+]) => {
        mod $module {
            use super::*;

            #[test]
            fn generate() {
                generate_test(
                    concat!("tests/feature/element_namespace_default/", $xsd),
                    concat!("tests/feature/element_namespace_default/expected/", $expected),
                    $config,
                );
            }

            #[test]
            #[cfg(not(feature = "update-expectations"))]
            fn read() {
                $(
                    let obj = crate::utils::quick_xml_read_test::<generated::FooType, _>(concat!(
                        "tests/feature/element_namespace_default/example/",
                        $read
                    ));

                    check_obj!(generated, obj);
                )+
            }

            #[test]
            #[cfg(not(feature = "update-expectations"))]
            fn write() {
                crate::utils::quick_xml_write_test(
                    &test_obj!(generated),
                    $root,
                    concat!("tests/feature/element_namespace_default/example/", $xml),
                );
            }

            #[cfg(not(feature = "update-expectations"))]
            mod generated {
                #![allow(unused_imports)]

                include!(concat!("expected/", $expected));
            }
        }
    };
}

// unqualified schema with an unprefixed target namespace.
case!(
    local,
    "schema.xsd",
    "quick_xml_local.rs",
    config(NamespaceSerialization::Local, None),
    "Foo",
    "local.xml",
    ["local.xml", "global.xml"]
);
case!(
    global,
    "schema.xsd",
    "quick_xml_global.rs",
    config(NamespaceSerialization::Global, None),
    "Foo",
    "global.xml",
    ["global.xml", "local.xml"]
);
case!(
    dynamic,
    "schema.xsd",
    "quick_xml_dynamic.rs",
    config(NamespaceSerialization::Dynamic, None),
    "Foo",
    "dynamic.xml",
    ["dynamic.xml", "global.xml"]
);

// target namespace with prefix, written as default namespace.
case!(
    prefixed_default_namespace,
    "schema_prefixed.xsd",
    "quick_xml_prefixed_default_namespace.rs",
    config(NamespaceSerialization::Global, Some(NS_T)),
    "Foo",
    "global.xml",
    ["global.xml", "local.xml"]
);

// target namespace with prefix, root name with prefix.
case!(
    prefixed_global,
    "schema_prefixed.xsd",
    "quick_xml_prefixed_global.rs",
    config(NamespaceSerialization::Global, None),
    "tns:Foo",
    "prefixed.xml",
    ["prefixed.xml", "prefixed_local.xml"]
);

// root name with prefix in mode `Local`.
case!(
    prefixed_local,
    "schema_prefixed.xsd",
    "quick_xml_prefixed_local.rs",
    config(NamespaceSerialization::Local, None),
    "tns:Foo",
    "prefixed_local.xml",
    ["prefixed_local.xml", "prefixed.xml"]
);
