use xsd_parser::{
    config::{GeneratorFlags, OptimizerFlags},
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_derive(["Debug"])
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_dynamic_type((IdentType::Type, "tns:animal"))
        .with_generate([(IdentType::Element, "tns:list")])
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/dynamic_type_element_name/schema.xsd",
        "tests/feature/dynamic_type_element_name/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{AnimalDyn, AnimalType, DogType, List};

    let obj = List {
        pet: vec![
            AnimalType::new(AnimalDyn { id: 1 }),
            AnimalType::new(DogType {
                id: 2,
                name: "Rex".into(),
            }),
        ],
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:list",
        "tests/feature/dynamic_type_element_name/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

/* quick_xml (optimized) */

#[test]
fn generate_quick_xml_optimized() {
    generate_test(
        "tests/feature/dynamic_type_element_name/schema.xsd",
        "tests/feature/dynamic_type_element_name/expected/quick_xml_optimized.rs",
        config()
            .with_quick_xml()
            .with_optimizer_flags(OptimizerFlags::all()),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_optimized() {
    use quick_xml_optimized::{AnimalDyn, AnimalType, DogType, List};

    let obj = List {
        pet: vec![
            AnimalType::Animal(AnimalDyn { id: 1 }),
            AnimalType::Dog(DogType {
                id: 2,
                name: "Rex".into(),
            }),
        ],
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:list",
        "tests/feature/dynamic_type_element_name/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_optimized {
    #![allow(unused_imports)]

    include!("expected/quick_xml_optimized.rs");
}
