//! The default namespace of the serializer is set at runtime with
//! `SerializeHelper::with_default_namespace`.

use xsd_parser::{
    config::{GeneratorFlags, NamespaceIdent},
    pipeline::renderer::NamespaceSerialization,
    Config, IdentType,
};
use xsd_parser_types::misc::Namespace;

use crate::utils::{generate_test, ConfigEx};

const NS_T: Namespace = Namespace::new_const(b"T");
const NS_O: Namespace = Namespace::new_const(b"O");

fn config(mode: NamespaceSerialization) -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT | GeneratorFlags::USE_MODULES)
        .with_dynamic_type((IdentType::Type, "tns:Animal"))
        .with_generate([
            (
                IdentType::Element,
                Some(NamespaceIdent::Namespace(NS_T)),
                "Foo",
            ),
            (
                IdentType::Element,
                Some(NamespaceIdent::Namespace(NS_O)),
                "Wrap",
            ),
            (
                IdentType::Element,
                Some(NamespaceIdent::Namespace(NS_T)),
                "Zoo",
            ),
        ])
        .with_quick_xml_config(mode, None, false, false)
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! check_foo {
    ($foo:expr) => {{
        let foo = $foo;

        assert_eq!(foo.bar.content, "b");
        assert_eq!(foo.bar.a, 1);
        assert_eq!(foo.baz, "z");
        assert_eq!(foo.q.inner, "i");
        // Dynamic elements with a name other than their type can not be read yet.
        assert!(foo.animal.is_none());
    }};
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! foo_obj {
    () => {{
        foo_obj!(true)
    }};
    ($with_animal:expr) => {{
        use generated::tns::{AnimalType, BarType, DogType, FooType, QType};

        FooType {
            bar: BarType {
                content: "b".into(),
                a: 1,
            },
            baz: "z".into(),
            q: QType { inner: "i".into() },
            animal: $with_animal.then(|| AnimalType::new(DogType { name: "d".into() })),
        }
    }};
}

/// One test case per namespace mode: generate the snapshot, read every file
/// of `$read` back, and write the objects with the runtime default namespace
/// of the given case and compare them to the XML files.
macro_rules! case {
    ($module:ident, $expected:literal, $mode:expr, [$($read:literal),+], [$($name:ident: $root:literal, $default:expr, $xml:literal, $animal:literal;)+]) => {
        mod $module {
            use super::*;

            #[test]
            fn generate() {
                generate_test(
                    "tests/feature/default_namespace_runtime/wrap.xsd",
                    concat!("tests/feature/default_namespace_runtime/expected/", $expected),
                    config($mode),
                );
            }

            #[test]
            #[cfg(not(feature = "update-expectations"))]
            fn read() {
                $(
                    let obj = crate::utils::quick_xml_read_test::<generated::tns::FooType, _>(
                        concat!("tests/feature/default_namespace_runtime/example/", $read),
                    );

                    check_foo!(obj);
                )+
            }

            $(
                #[test]
                #[cfg(not(feature = "update-expectations"))]
                fn $name() {
                    use xsd_parser_types::quick_xml::SerializeHelper;

                    let default: Option<Namespace> = $default;
                    let mut helper = SerializeHelper::default();
                    if let Some(default) = default {
                        helper = helper.with_default_namespace(default);
                    }

                    crate::utils::quick_xml_write_test_with_helper(
                        &foo_obj!($animal),
                        $root,
                        helper,
                        concat!("tests/feature/default_namespace_runtime/example/", $xml),
                    );
                }
            )+

            #[cfg(not(feature = "update-expectations"))]
            pub(super) mod generated {
                #![allow(unused_imports)]

                include!(concat!("expected/", $expected));
            }
        }
    };
}

// root with and without prefix, with and without runtime default.
case!(
    dynamic,
    "quick_xml_dynamic.rs",
    NamespaceSerialization::Dynamic,
    ["local.xml"],
    [
        write_prefixed_root: "tns:Foo", Some(NS_T), "runtime_dynamic.xml", true;
        write_unprefixed_root: "Foo", Some(NS_T), "runtime_dynamic.xml", true;
        write_no_default: "tns:Foo", None, "none_dynamic.xml", true;
    ]
);
case!(
    global,
    "quick_xml_global.rs",
    NamespaceSerialization::Global,
    ["local.xml"],
    [
        write_prefixed_root: "tns:Foo", Some(NS_T), "runtime.xml", true;
        write_unprefixed_root: "Foo", Some(NS_T), "runtime.xml", true;
        write_no_default: "tns:Foo", None, "none.xml", true;
    ]
);
case!(
    local,
    "quick_xml_local.rs",
    NamespaceSerialization::Local,
    ["local.xml"],
    [
        write_prefixed_root: "tns:Foo", Some(NS_T), "local.xml", false;
    ]
);

// root in another namespace than the runtime default (dynamic mode).
#[cfg(not(feature = "update-expectations"))]
mod wrap {
    use xsd_parser_types::quick_xml::SerializeHelper;

    use super::dynamic::generated;
    use super::*;

    #[test]
    fn write() {
        use generated::o::WrapType;

        let obj = WrapType { foo: foo_obj!() };

        crate::utils::quick_xml_write_test_with_helper(
            &obj,
            "o:Wrap",
            SerializeHelper::default().with_default_namespace(NS_T),
            "tests/feature/default_namespace_runtime/example/wrap.xml",
        );
    }
}

// the prefix of T is only used by the `xsi:type` value, so it is kept.
#[cfg(not(feature = "update-expectations"))]
mod zoo {
    use xsd_parser_types::quick_xml::SerializeHelper;

    use super::*;

    macro_rules! zoo_test {
        ($name:ident, $module:ident, $xml:literal) => {
            #[test]
            fn $name() {
                use super::$module::generated::tns::{AnimalType, DogType, ZooType};

                let obj = ZooType {
                    animal: AnimalType::new(DogType { name: "d".into() }),
                };

                crate::utils::quick_xml_write_test_with_helper(
                    &obj,
                    "tns:Zoo",
                    SerializeHelper::default().with_default_namespace(NS_T),
                    concat!("tests/feature/default_namespace_runtime/example/", $xml),
                );
            }
        };
    }

    zoo_test!(write_global, global, "zoo.xml");
    zoo_test!(write_dynamic, dynamic, "zoo_dynamic.xml");
}
