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

fn config_three_levels() -> Config {
    config().with_dynamic_type((IdentType::Type, "tns:dog"))
}

fn config_choice() -> Config {
    config().with_generate([(IdentType::Element, "tns:zoo")])
}

fn config_nillable() -> Config {
    config()
        .with_generator_flags(GeneratorFlags::NILLABLE_TYPE_SUPPORT)
        .with_generate([(IdentType::Element, "tns:bag")])
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/dynamic_type_same_type_elements/schema.xsd",
        "tests/feature/dynamic_type_same_type_elements/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
fn generate_quick_xml_three_levels() {
    generate_test(
        "tests/feature/dynamic_type_same_type_elements/schema_three_levels.xsd",
        "tests/feature/dynamic_type_same_type_elements/expected/quick_xml_three_levels.rs",
        config_three_levels().with_quick_xml(),
    );
}

#[test]
fn generate_quick_xml_choice() {
    generate_test(
        "tests/feature/dynamic_type_same_type_elements/schema_choice.xsd",
        "tests/feature/dynamic_type_same_type_elements/expected/quick_xml_choice.rs",
        config_choice().with_quick_xml(),
    );
}

#[test]
fn generate_quick_xml_nillable() {
    generate_test(
        "tests/feature/dynamic_type_same_type_elements/schema_nillable.xsd",
        "tests/feature/dynamic_type_same_type_elements/expected/quick_xml_nillable.rs",
        config_nillable().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_animal_only() {
    use quick_xml::{DogType, List};

    let obj = crate::utils::quick_xml_read_test::<List, _>(
        "tests/feature/dynamic_type_same_type_elements/example/animal_only.xml",
    );

    assert!(obj.pet.is_none());
    let dog = obj.animal.0.as_any().downcast_ref::<DogType>().unwrap();
    assert_eq!(dog.id, 2);
    assert_eq!(dog.name, "Rex");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_both() {
    use quick_xml::{AnimalDyn, List};

    let obj = crate::utils::quick_xml_read_test::<List, _>(
        "tests/feature/dynamic_type_same_type_elements/example/both.xml",
    );

    let pet = obj.pet.unwrap();
    let pet = pet.0.as_any().downcast_ref::<AnimalDyn>().unwrap();
    assert_eq!(pet.id, 1);
    let animal = obj.animal.0.as_any().downcast_ref::<AnimalDyn>().unwrap();
    assert_eq!(animal.id, 2);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_foreign_tag() {
    use xsd_parser_types::quick_xml::ErrorKind;

    use quick_xml::List;

    let result = crate::utils::quick_xml_read_test_result::<List, _>(
        "tests/feature/dynamic_type_same_type_elements/example/foreign.xml",
    );

    let err = result.unwrap_err();

    assert!(matches!(err.kind, ErrorKind::UnexpectedEvent(_)), "{err:?}");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_three_levels() {
    use quick_xml_three_levels::{List, PuppyType};

    let obj = crate::utils::quick_xml_read_test::<List, _>(
        "tests/feature/dynamic_type_same_type_elements/example/puppy.xml",
    );

    let puppy = obj.pet.0.as_any().downcast_ref::<PuppyType>().unwrap();
    assert_eq!(puppy.id, 3);
    assert_eq!(puppy.name, "Bo");
    assert_eq!(puppy.age, 1);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_choice_note() {
    use quick_xml_choice::{Zoo, ZooType};

    let obj = crate::utils::quick_xml_read_test::<Zoo, _>(
        "tests/feature/dynamic_type_same_type_elements/example/choice_note.xml",
    );

    let ZooType::Note(note) = obj else {
        panic!("unexpected zoo: {obj:?}");
    };
    assert_eq!(note, "hello");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_choice_pet() {
    use quick_xml_choice::{DogType, Zoo, ZooType};

    let obj = crate::utils::quick_xml_read_test::<Zoo, _>(
        "tests/feature/dynamic_type_same_type_elements/example/choice_pet.xml",
    );

    let ZooType::Pet(pet) = obj else {
        panic!("unexpected zoo: {obj:?}");
    };
    let dog = pet.0.as_any().downcast_ref::<DogType>().unwrap();
    assert_eq!(dog.id, 4);
    assert_eq!(dog.name, "Rex");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_nillable_plain() {
    use quick_xml_nillable::{AnimalDyn, Bag};

    let obj = crate::utils::quick_xml_read_test::<Bag, _>(
        "tests/feature/dynamic_type_same_type_elements/example/nillable_plain.xml",
    );

    let pet = obj.pet.0.unwrap();
    let pet = pet.0.as_any().downcast_ref::<AnimalDyn>().unwrap();
    assert_eq!(pet.id, 5);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_nillable_typed() {
    use quick_xml_nillable::{Bag, DogType};

    let obj = crate::utils::quick_xml_read_test::<Bag, _>(
        "tests/feature/dynamic_type_same_type_elements/example/nillable_typed.xml",
    );

    let pet = obj.pet.0.unwrap();
    let dog = pet.0.as_any().downcast_ref::<DogType>().unwrap();
    assert_eq!(dog.id, 6);
    assert_eq!(dog.name, "Bo");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_nillable_nil() {
    use quick_xml_nillable::Bag;

    let obj = crate::utils::quick_xml_read_test::<Bag, _>(
        "tests/feature/dynamic_type_same_type_elements/example/nillable_nil.xml",
    );

    assert!(obj.pet.0.is_none());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_nillable_foreign_nil() {
    use xsd_parser_types::quick_xml::ErrorKind;

    use quick_xml_nillable::Bag;

    let result = crate::utils::quick_xml_read_test_result::<Bag, _>(
        "tests/feature/dynamic_type_same_type_elements/example/nillable_foreign_nil.xml",
    );

    let err = result.unwrap_err();

    assert!(matches!(err.kind, ErrorKind::UnexpectedEvent(_)), "{err:?}");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_choice {
    #![allow(unused_imports)]

    include!("expected/quick_xml_choice.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_nillable {
    #![allow(unused_imports)]

    include!("expected/quick_xml_nillable.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_three_levels {
    #![allow(unused_imports)]

    include!("expected/quick_xml_three_levels.rs");
}

/* quick_xml (optimized) */

#[test]
fn generate_quick_xml_optimized() {
    generate_test(
        "tests/feature/dynamic_type_same_type_elements/schema.xsd",
        "tests/feature/dynamic_type_same_type_elements/expected/quick_xml_optimized.rs",
        config()
            .with_quick_xml()
            .with_optimizer_flags(OptimizerFlags::all()),
    );
}

#[test]
fn generate_quick_xml_three_levels_optimized() {
    generate_test(
        "tests/feature/dynamic_type_same_type_elements/schema_three_levels.xsd",
        "tests/feature/dynamic_type_same_type_elements/expected/quick_xml_three_levels_optimized.rs",
        config_three_levels()
            .with_quick_xml()
            .with_optimizer_flags(OptimizerFlags::all()),
    );
}

#[test]
fn generate_quick_xml_choice_optimized() {
    generate_test(
        "tests/feature/dynamic_type_same_type_elements/schema_choice.xsd",
        "tests/feature/dynamic_type_same_type_elements/expected/quick_xml_choice_optimized.rs",
        config_choice()
            .with_quick_xml()
            .with_optimizer_flags(OptimizerFlags::all()),
    );
}

#[test]
fn generate_quick_xml_nillable_optimized() {
    generate_test(
        "tests/feature/dynamic_type_same_type_elements/schema_nillable.xsd",
        "tests/feature/dynamic_type_same_type_elements/expected/quick_xml_nillable_optimized.rs",
        config_nillable()
            .with_quick_xml()
            .with_optimizer_flags(OptimizerFlags::all()),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_optimized_animal_only() {
    use quick_xml_optimized::{AnimalType, List};

    let obj = crate::utils::quick_xml_read_test::<List, _>(
        "tests/feature/dynamic_type_same_type_elements/example/animal_only.xml",
    );

    assert!(obj.pet.is_none());
    let AnimalType::Dog(dog) = obj.animal else {
        panic!("unexpected animal: {:?}", obj.animal);
    };
    assert_eq!(dog.id, 2);
    assert_eq!(dog.name, "Rex");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_optimized_both() {
    use quick_xml_optimized::{AnimalType, List};

    let obj = crate::utils::quick_xml_read_test::<List, _>(
        "tests/feature/dynamic_type_same_type_elements/example/both.xml",
    );

    let Some(AnimalType::Animal(pet)) = obj.pet else {
        panic!("unexpected pet: {:?}", obj.pet);
    };
    assert_eq!(pet.id, 1);
    let AnimalType::Animal(animal) = obj.animal else {
        panic!("unexpected animal: {:?}", obj.animal);
    };
    assert_eq!(animal.id, 2);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_optimized_foreign_tag() {
    use xsd_parser_types::quick_xml::ErrorKind;

    use quick_xml_optimized::List;

    let result = crate::utils::quick_xml_read_test_result::<List, _>(
        "tests/feature/dynamic_type_same_type_elements/example/foreign.xml",
    );

    let err = result.unwrap_err();

    assert!(matches!(err.kind, ErrorKind::UnexpectedEvent(_)), "{err:?}");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_three_levels_optimized() {
    use quick_xml_three_levels_optimized::{AnimalType, List};

    let obj = crate::utils::quick_xml_read_test::<List, _>(
        "tests/feature/dynamic_type_same_type_elements/example/puppy.xml",
    );

    let AnimalType::Puppy(puppy) = obj.pet else {
        panic!("unexpected pet: {:?}", obj.pet);
    };
    assert_eq!(puppy.id, 3);
    assert_eq!(puppy.name, "Bo");
    assert_eq!(puppy.age, 1);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_optimized_choice_note() {
    use quick_xml_choice_optimized::{Zoo, ZooType};

    let obj = crate::utils::quick_xml_read_test::<Zoo, _>(
        "tests/feature/dynamic_type_same_type_elements/example/choice_note.xml",
    );

    let ZooType::Note(note) = obj else {
        panic!("unexpected zoo: {obj:?}");
    };
    assert_eq!(note, "hello");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_optimized_choice_pet() {
    use quick_xml_choice_optimized::{AnimalType, Zoo, ZooType};

    let obj = crate::utils::quick_xml_read_test::<Zoo, _>(
        "tests/feature/dynamic_type_same_type_elements/example/choice_pet.xml",
    );

    let ZooType::Pet(AnimalType::Dog(dog)) = obj else {
        panic!("unexpected zoo: {obj:?}");
    };
    assert_eq!(dog.id, 4);
    assert_eq!(dog.name, "Rex");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_optimized_nillable_plain() {
    use quick_xml_nillable_optimized::{AnimalType, Bag};

    let obj = crate::utils::quick_xml_read_test::<Bag, _>(
        "tests/feature/dynamic_type_same_type_elements/example/nillable_plain.xml",
    );

    let Some(AnimalType::Animal(pet)) = obj.pet.0 else {
        panic!("unexpected pet: {:?}", obj.pet);
    };
    assert_eq!(pet.id, 5);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_optimized_nillable_typed() {
    use quick_xml_nillable_optimized::{AnimalType, Bag};

    let obj = crate::utils::quick_xml_read_test::<Bag, _>(
        "tests/feature/dynamic_type_same_type_elements/example/nillable_typed.xml",
    );

    let Some(AnimalType::Dog(dog)) = obj.pet.0 else {
        panic!("unexpected pet: {:?}", obj.pet);
    };
    assert_eq!(dog.id, 6);
    assert_eq!(dog.name, "Bo");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_optimized_nillable_nil() {
    use quick_xml_nillable_optimized::Bag;

    let obj = crate::utils::quick_xml_read_test::<Bag, _>(
        "tests/feature/dynamic_type_same_type_elements/example/nillable_nil.xml",
    );

    assert!(obj.pet.0.is_none());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_optimized_nillable_foreign_nil() {
    use xsd_parser_types::quick_xml::ErrorKind;

    use quick_xml_nillable_optimized::Bag;

    let result = crate::utils::quick_xml_read_test_result::<Bag, _>(
        "tests/feature/dynamic_type_same_type_elements/example/nillable_foreign_nil.xml",
    );

    let err = result.unwrap_err();

    assert!(matches!(err.kind, ErrorKind::UnexpectedEvent(_)), "{err:?}");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_optimized {
    #![allow(unused_imports)]

    include!("expected/quick_xml_optimized.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_choice_optimized {
    #![allow(unused_imports)]

    include!("expected/quick_xml_choice_optimized.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_nillable_optimized {
    #![allow(unused_imports)]

    include!("expected/quick_xml_nillable_optimized.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_three_levels_optimized {
    #![allow(unused_imports)]

    include!("expected/quick_xml_three_levels_optimized.rs");
}
