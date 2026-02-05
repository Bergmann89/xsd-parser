use std::mem::replace;

use proc_macro2::Literal;
use quote::quote;

use crate::config::GeneratorFlags;
use crate::models::code::IdentPath;

use super::super::{Context, MetaData, Module, RenderStep, RenderStepType};

/// Implements a [`RenderStep`] that renders constants for the different namespaces
/// used in the schema.
#[derive(Default, Debug)]
pub struct PrefixConstantsRenderStep(State);

#[derive(Default, Debug)]
enum State {
    #[default]
    ResolveNamespace,
    NamespaceResolved(IdentPath),
    Done,
}

impl RenderStep for PrefixConstantsRenderStep {
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
                ctx.resolve_root_ident_path("xsd_parser_types::misc::NamespacePrefix"),
            );
        }
    }

    fn finish(&mut self, meta: &MetaData<'_>, module: &mut Module) {
        let State::NamespaceResolved(prefix) = replace(&mut self.0, State::Done) else {
            return;
        };

        let prefix_constants = meta.types.meta.types.modules.values().filter_map(|module| {
            let pfx = module.prefix()?;
            let const_name = module.make_prefix_const()?;
            let const_name = const_name.path.ident();
            let prefix_literal = Literal::byte_string(pfx.as_str().as_bytes());

            Some(quote! {
                pub const #const_name: #prefix = #prefix::new_const(#prefix_literal);
            })
        });

        let xsi_prefix_constant = meta.check_generator_flags(GeneratorFlags::NILLABLE_TYPE_SUPPORT).then(|| {
            let xsi_prefix_literal = Literal::byte_string(b"xsi");
            quote! {
                pub const PREFIX_XSI: #prefix = #prefix::new_const(#xsi_prefix_literal);
            }
        });

        module.prepend(quote! { #( #prefix_constants )* #xsi_prefix_constant });
    }
}
