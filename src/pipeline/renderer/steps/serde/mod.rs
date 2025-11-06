mod quick_xml;
mod serde_xml_rs_v7;
mod serde_xml_rs_v8;

use std::str::FromStr;

use proc_macro2::TokenStream;

use crate::models::code::IdentPath;
use crate::models::data::ComplexDataContent;
use crate::models::meta::MetaTypes;
use crate::models::Ident;

pub use self::quick_xml::SerdeQuickXmlTypesRenderStep;
pub use self::serde_xml_rs_v7::SerdeXmlRsV7TypesRenderStep;
pub use self::serde_xml_rs_v8::SerdeXmlRsV8TypesRenderStep;

use super::super::Context;

impl ComplexDataContent<'_> {
    fn is_empty_string_content(&self, types: &MetaTypes) -> bool {
        if let Some(ident) = &self.simple_type {
            if let Some(ident) = types.get_resolved_ident(ident) {
                if *ident == Ident::STRING {
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
