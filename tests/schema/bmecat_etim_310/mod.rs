use xsd_parser::{
    config::{GeneratorFlags, NamespaceIdent, OptimizerFlags},
    models::schema::Namespace,
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

const NS: Namespace = Namespace::new_const(b"http://www.etim-international.com/bmecat/31");

fn config() -> Config {
    let mut config = Config::test_default()
        .with_generator_flags(
            GeneratorFlags::all()
                - GeneratorFlags::BUILD_IN_ABSOLUTE_PATHS
                - GeneratorFlags::NILLABLE_TYPE_SUPPORT
                - GeneratorFlags::ABSOLUTE_PATHS_INSTEAD_USINGS,
        )
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
        "tests/schema/bmecat_etim_310/schema.xsd",
        "tests/schema/bmecat_etim_310/expected/default.rs",
        config(),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/schema/bmecat_etim_310/schema.xsd",
        "tests/schema/bmecat_etim_310/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

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
