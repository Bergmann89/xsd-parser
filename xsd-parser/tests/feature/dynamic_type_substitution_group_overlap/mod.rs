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
        "tests/feature/dynamic_type_substitution_group_overlap/schema.xsd",
        "tests/feature/dynamic_type_substitution_group_overlap/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}

/* default (flatten) */

#[test]
fn generate_default_flatten() {
    generate_test(
        "tests/feature/dynamic_type_substitution_group_overlap/schema.xsd",
        "tests/feature/dynamic_type_substitution_group_overlap/expected/default_flatten.rs",
        config().with_optimizer_flags(OptimizerFlags::MERGE_DYNAMIC_TYPES),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default_flatten {
    #![allow(unused_imports)]

    include!("expected/default_flatten.rs");
}

/* default (optimized) */

#[test]
fn generate_default_optimized() {
    generate_test(
        "tests/feature/dynamic_type_substitution_group_overlap/schema.xsd",
        "tests/feature/dynamic_type_substitution_group_overlap/expected/default_optimized.rs",
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
        "tests/feature/dynamic_type_substitution_group_overlap/schema.xsd",
        "tests/feature/dynamic_type_substitution_group_overlap/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::{AnimalDyn, AnimalType, DogType, LabradorType, List};

    let obj = crate::utils::quick_xml_read_test::<List, _>(
        "tests/feature/dynamic_type_substitution_group_overlap/example/default.xml",
    );

    let mut items = obj.animal.iter();

    let item = items.next().unwrap();
    let item = item.0.as_any().downcast_ref::<AnimalType>().unwrap();
    let item = item.0.as_any().downcast_ref::<AnimalDyn>().unwrap();
    assert_eq!(item.id, 1);

    let item = items.next().unwrap();
    let item = item.0.as_any().downcast_ref::<AnimalType>().unwrap();
    let item = item.0.as_any().downcast_ref::<DogType>().unwrap();
    assert_eq!(item.id, 2);
    assert_eq!(item.name, "Max");

    let item = items.next().unwrap();
    let item = item.0.as_any().downcast_ref::<AnimalType>().unwrap();
    let item = item.0.as_any().downcast_ref::<LabradorType>().unwrap();
    assert_eq!(item.id, 3);
    assert_eq!(item.name, "Charlie");
    assert_eq!(item.color, "yellow");

    let item = items.next().unwrap();
    let item = item.0.as_any().downcast_ref::<DogType>().unwrap();
    assert_eq!(item.id, 4);
    assert_eq!(item.name, "Rex");

    let item = items.next().unwrap();
    let item = item.0.as_any().downcast_ref::<LabradorType>().unwrap();
    assert_eq!(item.id, 5);
    assert_eq!(item.name, "Buddy");
    assert_eq!(item.color, "black");

    assert!(items.next().is_none());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{Animal, AnimalDyn, AnimalType, DogType, LabradorType, List};

    let obj = List {
        animal: vec![
            Animal::new(AnimalType::new(AnimalDyn { id: 1 })),
            Animal::new(AnimalType::new(DogType {
                id: 2,
                name: "Max".into(),
            })),
            Animal::new(AnimalType::new(LabradorType {
                id: 3,
                name: "Charlie".into(),
                color: "yellow".into(),
            })),
            Animal::new(DogType {
                id: 4,
                name: "Rex".into(),
            }),
            Animal::new(LabradorType {
                id: 5,
                name: "Buddy".into(),
                color: "black".into(),
            }),
        ],
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "list",
        "tests/feature/dynamic_type_substitution_group_overlap/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

/* quick_xml (flatten) */

#[test]
fn generate_quick_xml_flatten() {
    generate_test(
        "tests/feature/dynamic_type_substitution_group_overlap/schema.xsd",
        "tests/feature/dynamic_type_substitution_group_overlap/expected/quick_xml_flatten.rs",
        config()
            .with_quick_xml()
            .with_optimizer_flags(OptimizerFlags::MERGE_DYNAMIC_TYPES),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_flatten() {
    use quick_xml_flatten::{AnimalDyn, DogType, LabradorType, List};

    let obj = crate::utils::quick_xml_read_test::<List, _>(
        "tests/feature/dynamic_type_substitution_group_overlap/example/default.xml",
    );

    let mut items = obj.animal.into_iter();

    let item = items.next().unwrap();
    let item = item.0.as_any().downcast_ref::<AnimalDyn>().unwrap();
    assert_eq!(item.id, 1);

    let item = items.next().unwrap();
    let item = item.0.as_any().downcast_ref::<DogType>().unwrap();
    assert_eq!(item.id, 2);
    assert_eq!(item.name, "Max");

    let item = items.next().unwrap();
    let item = item.0.as_any().downcast_ref::<LabradorType>().unwrap();
    assert_eq!(item.id, 3);
    assert_eq!(item.name, "Charlie");
    assert_eq!(item.color, "yellow");

    let item = items.next().unwrap();
    let item = item.0.as_any().downcast_ref::<DogType>().unwrap();
    assert_eq!(item.id, 4);
    assert_eq!(item.name, "Rex");

    let item = items.next().unwrap();
    let item = item.0.as_any().downcast_ref::<LabradorType>().unwrap();
    assert_eq!(item.id, 5);
    assert_eq!(item.name, "Buddy");
    assert_eq!(item.color, "black");

    assert!(items.next().is_none());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_flatten() {
    use quick_xml_flatten::{Animal, AnimalDyn, DogType, LabradorType, List};

    let obj = List {
        animal: vec![
            Animal::new(AnimalDyn { id: 1 }),
            Animal::new(DogType {
                id: 2,
                name: "Max".into(),
            }),
            Animal::new(LabradorType {
                id: 3,
                name: "Charlie".into(),
                color: "yellow".into(),
            }),
            Animal::new(DogType {
                id: 4,
                name: "Rex".into(),
            }),
            Animal::new(LabradorType {
                id: 5,
                name: "Buddy".into(),
                color: "black".into(),
            }),
        ],
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "list",
        "tests/feature/dynamic_type_substitution_group_overlap/example/default_flatten.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_flatten {
    #![allow(unused_imports)]

    include!("expected/quick_xml_flatten.rs");
}

/* quick_xml (optimized) */

#[test]
fn generate_quick_xml_optimized() {
    generate_test(
        "tests/feature/dynamic_type_substitution_group_overlap/schema.xsd",
        "tests/feature/dynamic_type_substitution_group_overlap/expected/quick_xml_optimized.rs",
        config()
            .with_quick_xml()
            .with_optimizer_flags(OptimizerFlags::all()),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_optimized() {
    use quick_xml_optimized::{Animal, List};

    let obj = crate::utils::quick_xml_read_test::<List, _>(
        "tests/feature/dynamic_type_substitution_group_overlap/example/default.xml",
    );

    let mut items = obj.animal.into_iter();

    let item = items.next().unwrap();
    let Animal::Animal(item) = item else {
        panic!("Expected `Animal::Animal`");
    };
    assert_eq!(item.id, 1);

    let item = items.next().unwrap();
    let Animal::Dog(item) = item else {
        panic!("Expected `Animal::Dog`");
    };
    assert_eq!(item.id, 2);
    assert_eq!(item.name, "Max");

    let item = items.next().unwrap();
    let Animal::Labrador(item) = item else {
        panic!("Expected `Animal::Labrador`");
    };
    assert_eq!(item.id, 3);
    assert_eq!(item.name, "Charlie");
    assert_eq!(item.color, "yellow");

    let item = items.next().unwrap();
    let Animal::Dog(item) = item else {
        panic!("Expected `Animal::Dog`");
    };
    assert_eq!(item.id, 4);
    assert_eq!(item.name, "Rex");

    let item = items.next().unwrap();
    let Animal::Labrador(item) = item else {
        panic!("Expected `Animal::Labrador`");
    };
    assert_eq!(item.id, 5);
    assert_eq!(item.name, "Buddy");
    assert_eq!(item.color, "black");

    assert!(items.next().is_none());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_optimized() {
    use quick_xml_optimized::{Animal, AnimalDyn, DogType, LabradorType, List};

    let obj = List {
        animal: vec![
            Animal::Animal(AnimalDyn { id: 1 }),
            Animal::Dog(DogType {
                id: 2,
                name: "Max".into(),
            }),
            Animal::Labrador(LabradorType {
                id: 3,
                name: "Charlie".into(),
                color: "yellow".into(),
            }),
            Animal::Dog(DogType {
                id: 4,
                name: "Rex".into(),
            }),
            Animal::Labrador(LabradorType {
                id: 5,
                name: "Buddy".into(),
                color: "black".into(),
            }),
        ],
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "list",
        "tests/feature/dynamic_type_substitution_group_overlap/example/default_optimized.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_optimized {
    #![allow(unused_imports)]

    include!("expected/quick_xml_optimized.rs");
}
