use proc_macro2::Literal;
use quote::quote;

use super::{Config, Module, Renderer};

/// Implements a [`Renderer`] that renders constants for the different namespaces
/// used in the schema.
#[derive(Debug)]
pub struct NamespaceConstantsRenderer;

impl Renderer for NamespaceConstantsRenderer {
    fn finish(&mut self, config: &Config<'_>, module: &mut Module) {
        let xsd_parser = &config.xsd_parser_crate;

        module.usings([quote!(#xsd_parser::schema::Namespace)]);

        let namespace_constants = config.types.modules.values().filter_map(|module| {
            let ns = module.namespace.as_ref()?;
            let const_name = module.make_ns_const();
            let const_name = const_name.ident();
            let namespace = Literal::byte_string(&ns.0);

            Some(quote! {
                pub const #const_name: Namespace = Namespace::new_const(#namespace);
            })
        });

        module.prepend(quote! { #( #namespace_constants )* });
    }
}
