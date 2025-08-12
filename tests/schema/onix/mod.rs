use quote::ToTokens;
use xsd_parser::{
    config::{GeneratorFlags, IdentTriple, OptimizerFlags, RendererFlags, Schema},
    exec_generator, exec_interpreter, exec_optimizer, exec_parser, exec_render,
    models::{
        meta::{ElementMetaVariant, ElementMode, MetaTypeVariant},
        schema::Namespace,
    },
    Config, IdentType, MetaTypes, Schemas,
};

use crate::utils::{generate_test_validate, ConfigEx};

const NS: Namespace = Namespace::new_const(b"http://www.etim-international.com/bmecat/31");

fn config() -> Config {
    let mut config = Config::test_default()
        .with_generator_flags(GeneratorFlags::all())
        .with_optimizer_flags(
            OptimizerFlags::all()
                - OptimizerFlags::FLATTEN_COMPLEX_TYPES
                - OptimizerFlags::USE_UNRESTRICTED_BASE_TYPE,
        )
        .with_renderer_flags(RendererFlags::RENDER_DOCS)
        .with_generate([(IdentType::Element, "onix:ONIXMessage")]);

    config.generator.type_postfix.type_ = String::new();
    config.generator.type_postfix.element = "Element".into();
    config.generator.type_postfix.element_type = "ElementType".into();

    config
}

#[test]
fn generate_default() {
    generate_test(
        "tests/schema/onix/schema/ONIX_BookProduct_3.1_reference.xsd",
        "tests/schema/onix/expected/default.rs",
        config(),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/schema/onix/schema/ONIX_BookProduct_3.1_reference.xsd",
        "tests/schema/onix/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::onix::OnixMessageElement;

    let _obj = crate::utils::quick_xml_read_test::<OnixMessageElement, _>(
        "tests/schema/onix/examples/Onix3sample_refnames.xml",
    );
}

fn generate_test(input_xsd: &str, expected_rs: &str, mut config: Config) {
    config.parser.schemas.push(Schema::File(input_xsd.into()));

    config.parser.debug_output = Some("target/parser.log".into());
    config.interpreter.debug_output = Some("target/interpreter.log".into());
    config.optimizer.debug_output = Some("target/optimizer.log".into());

    let schemas = exec_parser(config.parser).unwrap();
    let meta_types = exec_interpreter(config.interpreter, &schemas).unwrap();
    let meta_types = resolve_naming_conflicts(&schemas, meta_types);
    let meta_types = exec_optimizer(config.optimizer, meta_types).unwrap();
    let data_types = exec_generator(config.generator, &schemas, &meta_types).unwrap();
    let module = exec_render(config.renderer, &data_types).unwrap();
    let code = module.to_token_stream().to_string();

    generate_test_validate(code, expected_rs);
}

fn resolve_naming_conflicts(schemas: &Schemas, mut types: MetaTypes) -> MetaTypes {
    // Fix naming conflicts of the "type" attribute of "ol.attlist"
    let type_ = IdentTriple::from((IdentType::Type, "onix:type"))
        .resolve(schemas)
        .unwrap();
    let type_ = types.items.get_mut(&type_).unwrap();
    let MetaTypeVariant::Enumeration(ei) = &mut type_.variant else {
        unreachable!();
    };
    for variant in &mut *ei.variants {
        match variant.ident.name.as_str() {
            "a" => variant.display_name = Some("ALowerCase".into()),
            "i" => variant.display_name = Some("ILowerCase".into()),
            _ => (),
        }
    }

    // Rename onix:ONIXMessage content field and type
    let message = IdentTriple::from((IdentType::ElementType, "onix:ONIXMessage"))
        .resolve(schemas)
        .unwrap();
    let message = types.items.get(&message).unwrap();
    let MetaTypeVariant::ComplexType(ci) = &message.variant else {
        unreachable!();
    };
    let content = ci.content.clone().unwrap();
    let content = types.items.get_mut(&content).unwrap();
    let MetaTypeVariant::Sequence(si) = &mut content.variant else {
        unreachable!();
    };
    let element = si
        .elements
        .iter_mut()
        .find_map(|element| {
            if let ElementMetaVariant::Type {
                type_,
                mode: ElementMode::Group,
            } = &element.variant
            {
                element.display_name = Some("product_or_no_product_choice".into());

                Some(type_.clone())
            } else {
                None
            }
        })
        .unwrap();
    let element = types.items.get_mut(&element).unwrap();
    element.display_name = Some("ProductOrNoProductChoice".into());

    types
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
