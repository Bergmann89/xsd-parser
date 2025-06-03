use xsd_parser::{
    config::{GeneratorFlags, NamespaceIdent, OptimizerFlags},
    schema::Namespace,
    types::IdentType,
    Config,
};

use crate::utils::{generate_test, ConfigEx};

const NS: Namespace = Namespace::new_const(b"https://www.etim-international.com/bmecat/50");

fn config() -> Config {
    let mut config = Config::test_default()
        .with_generator_flags(GeneratorFlags::all())
        .with_optimizer_flags(OptimizerFlags::all() - OptimizerFlags::REMOVE_DUPLICATES)
        .with_generate([(
            IdentType::Element,
            Some(NamespaceIdent::namespace(NS)),
            "BMECAT",
        )]);

    config.generator.type_postfix.type_ = String::new();
    config.generator.type_postfix.element = "Element".into();
    config.generator.type_postfix.element_type = "ElementType".into();

    config
}

#[test]
fn generate_default() {
    generate_test(
        "tests/schema/bmecat_etim_501/schema.xsd",
        "tests/schema/bmecat_etim_501/expected/default.rs",
        config(),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/schema/bmecat_etim_501/schema.xsd",
        "tests/schema/bmecat_etim_501/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

macro_rules! define_test_read_quick_xml {
    ($name:ident, $file:literal) => {
        #[test]
        #[cfg(not(feature = "update-expectations"))]
        fn $name() {
            use quick_xml::BmecatElement;

            crate::utils::quick_xml_read_test::<BmecatElement, _>(
                std::path::Path::new("tests/schema/bmecat_etim_501/example").join($file),
            );
        }
    };
}

define_test_read_quick_xml!(
    read_quick_xml_etim7,
    "SampleFile_bmecat_etim_V5_T_NEW_CATALOG_ETIM7.xml"
);

define_test_read_quick_xml!(
    read_quick_xml_etim8,
    "SampleFile_bmecat_etim_V5_T_NEW_CATALOG_ETIM8.xml"
);

define_test_read_quick_xml!(
    read_quick_xml_etim9,
    "SampleFile_bmecat_etim_V5_T_NEW_CATALOG_ETIM9.xml"
);

define_test_read_quick_xml!(
    read_quick_xml_etim10,
    "SampleFile_bmecat_etim_V5_T_NEW_CATALOG_ETIM10.xml"
);

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
