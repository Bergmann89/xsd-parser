use xsd_parser::{
    config::{GeneratorFlags, OptimizerFlags},
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_derive(["Debug"])
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_generate([(IdentType::Element, "tns:list")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/dynamic_type_substitution_group/schema.xsd",
        "tests/feature/dynamic_type_substitution_group/expected/default.rs",
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
        "tests/feature/dynamic_type_substitution_group/schema.xsd",
        "tests/feature/dynamic_type_substitution_group/expected/default_optimized.rs",
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
        "tests/feature/dynamic_type_substitution_group/schema.xsd",
        "tests/feature/dynamic_type_substitution_group/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::{BeagleType, CatType, DogType, LabradorType, List};

    let obj = crate::utils::quick_xml_read_test::<List, _>(
        "tests/feature/dynamic_type_substitution_group/example/default.xml",
    );

    let mut items = obj.animal.iter();

    let item = items.next().unwrap();
    let item = item.0.as_any().downcast_ref::<CatType>().unwrap();
    assert_eq!(item.id, 1);
    assert_eq!(item.breed, "Bengal");

    let item = items.next().unwrap();
    let item = item.0.as_any().downcast_ref::<DogType>().unwrap();
    let item = item.0.as_any().downcast_ref::<LabradorType>().unwrap();
    assert_eq!(item.id, 2);
    assert_eq!(item.name, "Buddy");
    assert_eq!(item.color, "black");

    let item = items.next().unwrap();
    let item = item.0.as_any().downcast_ref::<DogType>().unwrap();
    let item = item.0.as_any().downcast_ref::<BeagleType>().unwrap();
    assert_eq!(item.id, 3);
    assert_eq!(item.name, "Max");
    assert_eq!(item.age, 3);

    assert!(items.next().is_none());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{Animal, BeagleType, CatType, DogType, LabradorType, List};

    let obj = List {
        animal: vec![
            Animal::new(CatType {
                id: 1,
                breed: "Bengal".into(),
            }),
            Animal::new(DogType::new(LabradorType {
                id: 2,
                name: "Buddy".into(),
                color: "black".into(),
            })),
            Animal::new(DogType::new(BeagleType {
                id: 3,
                name: "Max".into(),
                age: 3,
            })),
        ],
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "list",
        "tests/feature/dynamic_type_substitution_group/example/default.xml",
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
        "tests/feature/dynamic_type_substitution_group/schema.xsd",
        "tests/feature/dynamic_type_substitution_group/expected/quick_xml_optimized.rs",
        config()
            .with_quick_xml()
            .with_optimizer_flags(OptimizerFlags::all()),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_optimized() {
    use quick_xml_optimized::{Animal, DogType, List};

    let obj = crate::utils::quick_xml_read_test::<List, _>(
        "tests/feature/dynamic_type_substitution_group/example/default.xml",
    );

    let mut items = obj.animal.into_iter();

    let item = items.next().unwrap();
    let Animal::Cat(item) = item else {
        panic!("Expected `Animal::Cat`");
    };
    assert_eq!(item.id, 1);
    assert_eq!(item.breed, "Bengal");

    let item = items.next().unwrap();
    let Animal::Dog(DogType::Labrador(item)) = item else {
        panic!("Expected `Animal::Dog(DogType::Labrador)`");
    };
    assert_eq!(item.id, 2);
    assert_eq!(item.name, "Buddy");
    assert_eq!(item.color, "black");

    let item = items.next().unwrap();
    let Animal::Dog(DogType::Beagle(item)) = item else {
        panic!("Expected `Animal::Dog(DogType::Beagle)`");
    };
    assert_eq!(item.id, 3);
    assert_eq!(item.name, "Max");
    assert_eq!(item.age, 3);

    assert!(items.next().is_none());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_optimized() {
    use quick_xml_optimized::{Animal, BeagleType, CatType, DogType, LabradorType, List};

    let obj = List {
        animal: vec![
            Animal::Cat(CatType {
                id: 1,
                breed: "Bengal".into(),
            }),
            Animal::Dog(DogType::Labrador(LabradorType {
                id: 2,
                name: "Buddy".into(),
                color: "black".into(),
            })),
            Animal::Dog(DogType::Beagle(BeagleType {
                id: 3,
                name: "Max".into(),
                age: 3,
            })),
        ],
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "list",
        "tests/feature/dynamic_type_substitution_group/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_optimized {
    #![allow(unused_imports)]

    include!("expected/quick_xml_optimized.rs");
}
