use proc_macro2::Literal;
use quote::quote;

use crate::pipeline::renderer::Context;

use super::super::{MetaData, Module, RenderStep};

/// Implements a [`RenderStep`] that renders constants for the different namespaces
/// used in the schema.
#[derive(Default, Debug)]
pub struct NamespaceConstantsRenderStep {
    using_added: bool,
}

impl RenderStep for NamespaceConstantsRenderStep {
    fn initialize(&mut self, meta: &mut MetaData<'_>) {
        self.using_added = meta.types.meta.types.modules.is_empty();
    }

    fn render_type(&mut self, ctx: &mut Context<'_, '_>) {
        if !self.using_added {
            self.using_added = true;
            ctx.add_root_usings(["xsd_parser::models::schema::Namespace"]);
        }
    }

    fn finish(&mut self, meta: &MetaData<'_>, module: &mut Module) {
        let namespace_constants = meta.types.meta.types.modules.values().filter_map(|module| {
            let ns = module.namespace.as_ref()?;
            let const_name = module.make_ns_const();
            let const_name = const_name.path.ident();
            let namespace = Literal::byte_string(&ns.0);

            Some(quote! {
                pub const #const_name: Namespace = Namespace::new_const(#namespace);
            })
        });

        module.prepend(quote! { #( #namespace_constants )* });
    }
}
