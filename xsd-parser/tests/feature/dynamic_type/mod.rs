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

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/dynamic_type/schema.xsd",
        "tests/feature/dynamic_type/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}

/* default (optimized) */

#[test]
fn generate_default_optimized() {
    generate_test(
        "tests/feature/dynamic_type/schema.xsd",
        "tests/feature/dynamic_type/expected/default_optimized.rs",
        config().with_optimizer_flags(OptimizerFlags::all()),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default_optmized {
    #![allow(unused_imports)]

    include!("expected/default_optimized.rs");
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/dynamic_type/schema.xsd",
        "tests/feature/dynamic_type/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::{AnimalDyn, DogType, LabradorType, List};

    let obj = crate::utils::quick_xml_read_test::<List, _>(
        "tests/feature/dynamic_type/example/default.xml",
    );

    let mut items = obj.animal.iter();

    let item = items.next().unwrap();
    let item = item.0.as_any().downcast_ref::<AnimalDyn>().unwrap();
    assert_eq!(item.id, 1);

    let item = items.next().unwrap();
    let item = item.0.as_any().downcast_ref::<DogType>().unwrap();
    assert_eq!(item.id, 2);
    assert_eq!(item.name, "Rex");

    let item = items.next().unwrap();
    let item = item.0.as_any().downcast_ref::<LabradorType>().unwrap();
    assert_eq!(item.id, 3);
    assert_eq!(item.name, "Buddy");
    assert_eq!(item.color, "black");

    assert!(items.next().is_none());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{AnimalDyn, AnimalType, DogType, LabradorType, List};

    let obj = List {
        animal: vec![
            AnimalType::new(AnimalDyn { id: 1 }),
            AnimalType::new(DogType {
                id: 2,
                name: "Rex".into(),
            }),
            AnimalType::new(LabradorType {
                id: 3,
                name: "Buddy".into(),
                color: "black".into(),
            }),
        ],
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "list",
        "tests/feature/dynamic_type/example/default.xml",
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
        "tests/feature/dynamic_type/schema.xsd",
        "tests/feature/dynamic_type/expected/quick_xml_optimized.rs",
        config()
            .with_quick_xml()
            .with_optimizer_flags(OptimizerFlags::all()),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_optimized() {
    use quick_xml_optimized::{AnimalType, List};

    let obj = crate::utils::quick_xml_read_test::<List, _>(
        "tests/feature/dynamic_type/example/default.xml",
    );

    let mut items = obj.animal.into_iter();

    let item = items.next().unwrap();
    let AnimalType::Animal(item) = item else {
        panic!("Expected `AnimalType::Animal`");
    };
    assert_eq!(item.id, 1);

    let item = items.next().unwrap();
    let AnimalType::Dog(item) = item else {
        panic!("Expected `AnimalType::Dog`");
    };
    assert_eq!(item.id, 2);
    assert_eq!(item.name, "Rex");

    let item = items.next().unwrap();
    let AnimalType::Labrador(item) = item else {
        panic!("Expected `AnimalType::Labrador`");
    };
    assert_eq!(item.id, 3);
    assert_eq!(item.name, "Buddy");
    assert_eq!(item.color, "black");

    assert!(items.next().is_none());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_optimized() {
    use quick_xml_optimized::{AnimalDyn, AnimalType, DogType, LabradorType, List};

    let obj = List {
        animal: vec![
            AnimalType::Animal(AnimalDyn { id: 1 }),
            AnimalType::Dog(DogType {
                id: 2,
                name: "Rex".into(),
            }),
            AnimalType::Labrador(LabradorType {
                id: 3,
                name: "Buddy".into(),
                color: "black".into(),
            }),
        ],
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "list",
        "tests/feature/dynamic_type/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_optimized {
    #![allow(unused_imports)]

    include!("expected/quick_xml_optimized.rs");
}
