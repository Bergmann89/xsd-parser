use proc_macro2::{Literal, TokenStream};
use quote::quote;
use xsd_parser_types::{misc::Namespace, xml::QName};

use crate::pipeline::{
    generator::{Context as GeneratorContext, Error, ValueGeneratorMode},
    renderer::{Context as RendererContext, ValueRendererBox},
};

/// Generates a default value or constant for the [`QName`](crate::models::schema::QName) type
/// from the passed `value` and returns it as a [`TokenStream`].
pub fn default(
    ctx: &GeneratorContext<'_, '_>,
    value: &str,
    mode: ValueGeneratorMode,
) -> Result<ValueRendererBox, Error> {
    if mode == ValueGeneratorMode::Literal {
        return Err(Error::InvalidDefaultValue {
            ident: ctx.ident.clone(),
            value: value.into(),
            mode,
        });
    }

    let name = QName::from_bytes(value.as_bytes().to_owned());
    let ns = name
        .prefix()
        .and_then(|prefix| {
            ctx.types.modules.values().find(|module| {
                module
                    .prefix()
                    .as_ref()
                    .is_some_and(|p| p.as_str().as_bytes() == prefix)
            })
        })
        .and_then(|module| module.namespace.clone());

    Ok(Box::new(
        move |ctx: &RendererContext<'_, '_>| -> TokenStream {
            let qname = ctx.resolve_ident_path("::xsd_parser_types::xml::QName");
            let namespace = ctx.resolve_ident_path("::xsd_parser_types::misc::Namespace");

            let ns = match &ns {
                None => quote::quote!(None),
                Some(x) if *x == Namespace::XS => quote::quote!(Some(#namespace::XS)),
                Some(x) if *x == Namespace::XSI => quote::quote!(Some(#namespace::XSI)),
                Some(x) if *x == Namespace::XML => quote::quote!(Some(#namespace::XML)),
                Some(x) => {
                    let x = Literal::byte_string(x.as_ref());

                    quote::quote!(Some(#namespace::new_const(#x)))
                }
            };
            let index = match name.index() {
                Some(index) => quote::quote!(Some(#index)),
                None => quote::quote!(None),
            };
            let raw = Literal::byte_string(name.as_bytes());

            quote!(#qname::new_const(#raw, #index, #ns))
        },
    ))
}
