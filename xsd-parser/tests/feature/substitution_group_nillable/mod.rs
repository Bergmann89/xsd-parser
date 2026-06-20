use xsd_parser::{
    config::{GeneratorFlags, OptimizerFlags},
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_derive(["Debug"])
        .with_generate([(IdentType::Element, "tns:list")])
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_nillable_type_support()
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/substitution_group_nillable/schema.xsd",
        "tests/feature/substitution_group_nillable/expected/default.rs",
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
        "tests/feature/substitution_group_nillable/schema.xsd",
        "tests/feature/substitution_group_nillable/expected/default_optimized.rs",
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
        "tests/feature/substitution_group_nillable/schema.xsd",
        "tests/feature/substitution_group_nillable/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::{AnimalDyn, DogDyn, LabradorDyn, List};

    let obj = crate::utils::quick_xml_read_test::<List, _>(
        "tests/feature/substitution_group_nillable/example/default.xml",
    );

    let mut items = obj.animal.into_iter();

    let item = items.next().unwrap();
    let item = item.0.into_any().downcast::<AnimalDyn>().unwrap().unwrap();
    assert_eq!(item.id, 1);

    let item = items.next().unwrap();
    let item = item.0.into_any().downcast::<DogDyn>().unwrap().unwrap();
    assert_eq!(item.id, 2);
    assert_eq!(item.name, "Rex");

    let item = items.next().unwrap();
    let item = item
        .0
        .into_any()
        .downcast::<LabradorDyn>()
        .unwrap()
        .unwrap();
    assert_eq!(item.id, 3);
    assert_eq!(item.name, "Buddy");
    assert_eq!(item.color, "black");

    let item = items.next().unwrap();
    let item = item.0.into_any().downcast::<AnimalDyn>().unwrap();
    assert!(item.is_none());

    let item = items.next().unwrap();
    let item = item.0.into_any().downcast::<DogDyn>().unwrap();
    assert!(item.is_none());

    let item = items.next().unwrap();
    let item = item.0.into_any().downcast::<LabradorDyn>().unwrap();
    assert!(item.is_none());

    assert!(items.next().is_none());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{Animal, AnimalType, DogType, LabradorType, List};
    use xsd_parser_types::xml::Nillable;

    let obj = List {
        animal: vec![
            Animal::new(Nillable::new(AnimalType { id: 1 })),
            Animal::new(Nillable::new(DogType {
                id: 2,
                name: "Rex".into(),
            })),
            Animal::new(Nillable::new(LabradorType {
                id: 3,
                name: "Buddy".into(),
                color: "black".into(),
            })),
            Animal::new(Nillable::<AnimalType>::nil()),
            Animal::new(Nillable::<DogType>::nil()),
            Animal::new(Nillable::<LabradorType>::nil()),
        ],
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "list",
        "tests/feature/substitution_group_nillable/example/default.xml",
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
        "tests/feature/substitution_group_nillable/schema.xsd",
        "tests/feature/substitution_group_nillable/expected/quick_xml_optimized.rs",
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
        "tests/feature/substitution_group_nillable/example/default.xml",
    );

    let mut items = obj.animal.into_iter();

    let item = items.next().unwrap();
    let Animal::Animal(item) = item else {
        panic!("Expected `Animal::Animal`");
    };
    let item = item.unwrap();
    assert_eq!(item.id, 1);

    let item = items.next().unwrap();
    let Animal::Dog(item) = item else {
        panic!("Expected `Animal::Dog`");
    };
    let item = item.unwrap();
    assert_eq!(item.id, 2);
    assert_eq!(item.name, "Rex");

    let item = items.next().unwrap();
    let Animal::Labrador(item) = item else {
        panic!("Expected `Animal::Labrador`");
    };
    let item = item.unwrap();
    assert_eq!(item.id, 3);
    assert_eq!(item.name, "Buddy");
    assert_eq!(item.color, "black");

    let item = items.next().unwrap();
    let Animal::Animal(item) = item else {
        panic!("Expected `Animal::Animal`");
    };
    assert!(item.is_none());

    let item = items.next().unwrap();
    let Animal::Dog(item) = item else {
        panic!("Expected `Animal::Dog`");
    };
    assert!(item.is_none());

    let item = items.next().unwrap();
    let Animal::Labrador(item) = item else {
        panic!("Expected `Animal::Labrador`");
    };
    assert!(item.is_none());

    assert!(items.next().is_none());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_optimized() {
    use quick_xml_optimized::{Animal, AnimalType, DogType, LabradorType, List};
    use xsd_parser_types::xml::Nillable;

    let obj = List {
        animal: vec![
            Animal::Animal(Nillable::new(AnimalType { id: 1 })),
            Animal::Dog(Nillable::new(DogType {
                id: 2,
                name: "Rex".into(),
            })),
            Animal::Labrador(Nillable::new(LabradorType {
                id: 3,
                name: "Buddy".into(),
                color: "black".into(),
            })),
            Animal::Animal(Nillable::<AnimalType>::nil()),
            Animal::Dog(Nillable::<DogType>::nil()),
            Animal::Labrador(Nillable::<LabradorType>::nil()),
        ],
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "list",
        "tests/feature/substitution_group_nillable/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_optimized {
    #![allow(unused_imports)]

    include!("expected/quick_xml_optimized.rs");
}
