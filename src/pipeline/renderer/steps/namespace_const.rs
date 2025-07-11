use proc_macro2::Literal;
use quote::quote;

use super::super::{MetaData, Module, RenderStep};

/// Implements a [`RenderStep`] that renders constants for the different namespaces
/// used in the schema.
#[derive(Debug)]
pub struct NamespaceConstantsRenderStep;

impl RenderStep for NamespaceConstantsRenderStep {
    fn finish(&mut self, meta: &MetaData<'_>, module: &mut Module) {
        module.usings(["xsd_parser::models::schema::Namespace"]);

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
