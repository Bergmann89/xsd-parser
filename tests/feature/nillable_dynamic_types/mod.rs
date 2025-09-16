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
        "tests/feature/nillable_dynamic_types/schema.xsd",
        "tests/feature/nillable_dynamic_types/expected/default.rs",
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
        "tests/feature/nillable_dynamic_types/schema.xsd",
        "tests/feature/nillable_dynamic_types/expected/default_optimized.rs",
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
        "tests/feature/nillable_dynamic_types/schema.xsd",
        "tests/feature/nillable_dynamic_types/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::{FinalDyn, IntermediateDyn, List};

    let obj = crate::utils::quick_xml_read_test::<List, _>(
        "tests/feature/nillable_dynamic_types/example/default.xml",
    );

    let mut items = obj.base.into_iter();
    let item = items.next().unwrap();
    let item = item
        .0
        .into_any()
        .downcast::<IntermediateDyn>()
        .unwrap()
        .unwrap();
    assert_eq!(item.base_value, Some(10));
    assert_eq!(item.intermediate_value, Some(11));

    let item = items.next().unwrap();
    let item = item.0.into_any().downcast::<FinalDyn>().unwrap().unwrap();
    assert_eq!(item.base_value, Some(20));
    assert_eq!(item.intermediate_value, Some(21));
    assert_eq!(item.final_value, Some(22));

    assert!(items.next().is_none());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{Base, FinalType, IntermediateType, List};
    use xsd_parser::xml::Nillable;

    let obj = List {
        base: vec![
            Base(Box::new(Nillable::new(IntermediateType {
                base_value: Some(10),
                intermediate_value: Some(11),
            }))),
            Base(Box::new(Nillable::new(FinalType {
                base_value: Some(20),
                intermediate_value: Some(21),
                final_value: Some(22),
            }))),
        ],
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "List",
        "tests/feature/nillable_dynamic_types/example/default.xml",
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
        "tests/feature/nillable_dynamic_types/schema.xsd",
        "tests/feature/nillable_dynamic_types/expected/quick_xml_optimized.rs",
        config()
            .with_quick_xml()
            .with_optimizer_flags(OptimizerFlags::all()),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_optimized() {
    use quick_xml_optimized::{Base, List};

    let obj = crate::utils::quick_xml_read_test::<List, _>(
        "tests/feature/nillable_dynamic_types/example/default.xml",
    );

    let mut items = obj.base.into_iter();
    let item = items.next().unwrap();
    let Base::Intermediate(item) = item else {
        panic!("Expected `Base::Intermediate`");
    };
    let item = item.unwrap();
    assert_eq!(item.base_value, Some(10));
    assert_eq!(item.intermediate_value, Some(11));

    let item = items.next().unwrap();
    let Base::Final(item) = item else {
        panic!("Expected `Base::Final`");
    };
    let item = item.unwrap();
    assert_eq!(item.base_value, Some(20));
    assert_eq!(item.intermediate_value, Some(21));
    assert_eq!(item.final_value, Some(22));

    assert!(items.next().is_none());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_optimized() {
    use quick_xml_optimized::{Base, FinalType, IntermediateType, List};
    use xsd_parser::xml::Nillable;

    let obj = List {
        base: vec![
            Base::Intermediate(Nillable::new(IntermediateType {
                base_value: Some(10),
                intermediate_value: Some(11),
            })),
            Base::Final(Nillable::new(FinalType {
                base_value: Some(20),
                intermediate_value: Some(21),
                final_value: Some(22),
            })),
        ],
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "List",
        "tests/feature/nillable_dynamic_types/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_optimized {
    #![allow(unused_imports)]

    include!("expected/quick_xml_optimized.rs");
}
