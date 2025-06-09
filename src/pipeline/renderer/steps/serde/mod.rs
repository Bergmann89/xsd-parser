mod quick_xml;
mod serde_xml_rs;

use std::fmt::Display;
use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::quote;

use crate::models::code::IdentPath;

pub use self::quick_xml::SerdeQuickXmlTypesRenderStep;
pub use self::serde_xml_rs::SerdeXmlRsTypesRenderStep;

use super::super::Context;

fn get_derive<I>(ctx: &Context<'_, '_>, extra: I) -> TokenStream
where
    I: IntoIterator<Item = &'static str>,
    I::Item: Display,
{
    ctx.add_usings([quote!(serde::Serialize), quote!(serde::Deserialize)]);

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
