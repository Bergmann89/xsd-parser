mod large_arrays;
mod quick_xml;
mod serde_xml_rs_v7;
mod serde_xml_rs_v8;

use std::str::FromStr;

use proc_macro2::{TokenStream, TokenTree};
use quote::quote;

use crate::models::{
    code::IdentPath,
    data::{ComplexDataAttribute, ComplexDataContent, ComplexDataElement, Occurs, PathData},
    meta::{ElementMetaVariant, MetaTypes},
};
use crate::TypeIdent;

pub use self::quick_xml::SerdeQuickXmlTypesRenderStep;
pub use self::serde_xml_rs_v7::SerdeXmlRsV7TypesRenderStep;
pub use self::serde_xml_rs_v8::SerdeXmlRsV8TypesRenderStep;

use super::super::Context;

impl PathData {
    fn serde_target_ident<'a>(&self, ident: Option<&'a TypeIdent>) -> Option<&'a TypeIdent> {
        // Generic XML wrappers change the value being serialized. The schema
        // identifier describes their inner value, not the wrapper itself.
        ident.filter(|_| self.generics.is_empty())
    }
}

impl ComplexDataElement<'_> {
    fn serde_target_ident(&self) -> Option<&TypeIdent> {
        let ident = match &self.meta().variant {
            ElementMetaVariant::Type { type_, .. } => Some(type_),
            _ => None,
        };
        self.target_type.serde_target_ident(ident)
    }
}

impl ComplexDataContent<'_> {
    fn is_empty_string_content(&self, types: &MetaTypes) -> bool {
        if let Some(ident) = &self.simple_type {
            if let Some(ident) = types.get_resolved_ident(ident) {
                if ident.matches(&TypeIdent::STRING) {
                    return true;
                }
            }
        }

        false
    }
}

fn get_derive<I>(ctx: &Context<'_, '_>, extra: I) -> TokenStream
where
    I: IntoIterator<Item = &'static str>,
{
    ctx.add_usings(["serde::Serialize", "serde::Deserialize"]);

    super::get_derive(ctx, DERIVE.into_iter().chain(extra))
}

fn get_dyn_type_traits(ctx: &Context<'_, '_>) -> TokenStream {
    super::get_dyn_type_traits(
        ctx,
        [
            &IdentPath::from_str("serde::Serialize").unwrap(),
            &IdentPath::from_str("serde::de::DeserializeOwned").unwrap(),
        ],
    )
}

const DERIVE: [&str; 2] = ["Serialize", "Deserialize"];

#[derive(Debug, Clone, Copy)]
pub(crate) struct SerdeOptions {
    large_arrays: bool,
    arrays_as_vec: bool,
}

impl Occurs {
    fn array_to_vec(self) -> Self {
        match self {
            Self::StaticList(_) => Self::DynamicList,
            x => x,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct FieldOverrides {
    serialize: bool,
    deserialize: bool,
    has_default: bool,
}

fn field_overrides(attributes: &[TokenStream]) -> FieldOverrides {
    let mut result = FieldOverrides::default();
    for attribute in attributes {
        let mut tokens = attribute.clone().into_iter();
        let Some(TokenTree::Ident(name)) = tokens.next() else {
            continue;
        };
        if name == "serde_as" {
            result.serialize = true;
            result.deserialize = true;
            continue;
        }
        if name != "serde" {
            continue;
        }
        let Some(TokenTree::Group(arguments)) = tokens.next() else {
            continue;
        };
        for token in arguments.stream() {
            if let TokenTree::Ident(name) = token {
                match name.to_string().as_str() {
                    "with" => {
                        result.serialize = true;
                        result.deserialize = true;
                    }
                    "serialize_with" => result.serialize = true,
                    "deserialize_with" => result.deserialize = true,
                    "default" => result.has_default = true,
                    _ => (),
                }
            }
        }
    }
    result
}

impl ComplexDataElement<'_> {
    fn serde_field_attributes(
        &self,
        ctx: &Context<'_, '_>,
        options: SerdeOptions,
        has_default: bool,
    ) -> TokenStream {
        let mut overrides = field_overrides(&self.extra_attributes);
        overrides.has_default |= has_default;
        let array_attribute = self.serde_array_field_attribute(ctx, options, &overrides);
        quote!(#array_attribute)
    }
}

impl ComplexDataContent<'_> {
    fn serde_field_attributes(
        &self,
        ctx: &Context<'_, '_>,
        options: SerdeOptions,
        has_default: bool,
    ) -> TokenStream {
        let mut overrides = field_overrides(&self.extra_attributes);
        overrides.has_default |= has_default;
        let array_attribute = self.serde_array_attribute(ctx, options, &overrides);
        quote!(#array_attribute)
    }
}

impl ComplexDataAttribute<'_> {
    fn serde_field_attributes(
        &self,
        ctx: &Context<'_, '_>,
        options: SerdeOptions,
        has_default: bool,
    ) -> TokenStream {
        let mut overrides = field_overrides(&self.extra_attributes);
        overrides.has_default |= has_default;
        let array_attribute = self.serde_array_attribute(ctx, options, &overrides);
        quote!(#array_attribute)
    }
}
