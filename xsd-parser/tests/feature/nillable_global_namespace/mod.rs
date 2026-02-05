use xsd_parser::pipeline::renderer::NamespaceSerialization;
use xsd_parser::{config::OptimizerFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_optimizer_flags(OptimizerFlags::all())
        .with_nillable_type_support()
        .with_quick_xml_deserialize()
        .with_generate([(IdentType::Element, "tns:TestElement")])
}

#[test]
fn generate_quick_xml_global() {
    generate_test(
        "tests/feature/nillable_global_namespace/schema.xsd",
        "tests/feature/nillable_global_namespace/expected/quick_xml_global.rs",
        config().with_quick_xml_serialize_config(NamespaceSerialization::Global, None),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_global {
    #![allow(unused_imports)]

    include!("expected/quick_xml_global.rs");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_write_quick_xml_global() {
    use crate::utils::{quick_xml_read_test, quick_xml_write_test};
    use quick_xml_global::TestElement;

    let obj = quick_xml_read_test::<TestElement, _>(
        "tests/feature/nillable_global_namespace/example/test.xml",
    );
    assert!(obj.value.is_nil());
    assert_eq!(obj.count, None);

    quick_xml_write_test(
        &obj,
        "TestElement",
        "tests/feature/nillable_global_namespace/example/test.xml",
    );
}
