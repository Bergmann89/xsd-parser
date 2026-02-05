use std::mem::replace;

use proc_macro2::Literal;
use quote::quote;

use crate::config::GeneratorFlags;
use crate::models::code::IdentPath;

use super::super::{Context, MetaData, Module, RenderStep, RenderStepType};

/// Implements a [`RenderStep`] that renders constants for the different namespaces
/// used in the schema.
#[derive(Default, Debug)]
pub struct NamespaceConstantsRenderStep(State);

#[derive(Default, Debug)]
enum State {
    #[default]
    ResolveNamespace,
    NamespaceResolved(IdentPath),
    Done,
}

impl RenderStep for NamespaceConstantsRenderStep {
    fn render_step_type(&self) -> RenderStepType {
        RenderStepType::ExtraImpls
    }

    fn initialize(&mut self, meta: &mut MetaData<'_>) {
        // Don't skip initialization even if modules are empty,
        // because we might still need to generate XSI constant
        if meta.types.meta.types.modules.is_empty()
            && !meta.check_generator_flags(GeneratorFlags::NILLABLE_TYPE_SUPPORT) {
            self.0 = State::Done;
        }
    }

    fn render_type(&mut self, ctx: &mut Context<'_, '_>) {
        if matches!(&self.0, State::ResolveNamespace) {
            self.0 = State::NamespaceResolved(
                ctx.resolve_root_ident_path("xsd_parser_types::misc::Namespace"),
            );
        }
    }

    fn finish(&mut self, meta: &MetaData<'_>, module: &mut Module) {
        let State::NamespaceResolved(namespace) = replace(&mut self.0, State::Done) else {
            return;
        };

        let namespace_constants = meta.types.meta.types.modules.values().filter_map(|module| {
            let ns = module.namespace.as_ref()?;
            let const_name = module.make_ns_const()?;
            let const_name = const_name.path.ident();
            let ns_literal = Literal::byte_string(&ns.0);

            Some(quote! {
                pub const #const_name: #namespace = #namespace::new_const(#ns_literal);
            })
        });

        let xsi_constant = meta.check_generator_flags(GeneratorFlags::NILLABLE_TYPE_SUPPORT).then(|| {
            let xsi_literal = Literal::byte_string(b"http://www.w3.org/2001/XMLSchema-instance");
            quote! {
                pub const NS_XSI: #namespace = #namespace::new_const(#xsi_literal);
            }
        });

        module.prepend(quote! { #( #namespace_constants )* #xsi_constant });
    }
}
