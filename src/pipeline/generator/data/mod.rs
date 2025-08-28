mod complex;
mod dynamic;
mod enumeration;
mod reference;
mod simple;
mod type_;
mod union;

use std::mem::swap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::models::{
    code::IdentPath,
    data::{BuildInData, CustomData, Occurs, PathData},
    meta::{BuildInMeta, CustomMeta},
};

impl<'types> BuildInData<'types> {
    fn new(meta: &'types BuildInMeta) -> Self {
        Self { meta }
    }
}

impl<'types> CustomData<'types> {
    fn new(meta: &'types CustomMeta) -> Self {
        Self { meta }
    }
}

impl Occurs {
    /// Wrapped the passed type `ident` into a suitable rust type depending on
    /// the occurrence and the need of indirection (boxing).
    ///
    /// # Examples
    /// - `Occurs::Single` will return the type as is, or as `Box<T>`
    /// - `Occurs::Optional` will return the type as `Option<T>`
    /// - `Occurs::DynamicList` will return the type as `Vec<T>`
    /// - `Occurs::StaticList` will return the type as array `[T; SIZE]`
    #[must_use]
    pub fn make_type(self, ident: &TokenStream, need_indirection: bool) -> Option<TokenStream> {
        match self {
            Self::None => None,
            Self::Single if need_indirection => Some(quote! { Box<#ident> }),
            Self::Single => Some(quote! { #ident }),
            Self::Optional if need_indirection => Some(quote! { Option<Box<#ident>> }),
            Self::Optional => Some(quote! { Option<#ident> }),
            Self::DynamicList => Some(quote! { Vec<#ident> }),
            Self::StaticList(sz) if need_indirection => Some(quote! { [Box<#ident>; #sz] }),
            Self::StaticList(sz) => Some(quote! { [#ident; #sz] }),
        }
    }
}

impl PathData {
    fn from_path_data_nillable(is_mixed: bool, mut path: PathData) -> PathData {
        if is_mixed {
            let mut tmp = IdentPath::from_ident(format_ident!("Nillable"));

            swap(&mut path.path, &mut tmp);

            path.with_generic(tmp)
                .with_using("xsd_parser::xml::Nillable")
        } else {
            path
        }
    }

    fn from_path_data_mixed(is_mixed: bool, mut path: PathData) -> PathData {
        if is_mixed {
            let mut tmp = IdentPath::from_ident(format_ident!("Mixed"));

            swap(&mut path.path, &mut tmp);

            path.with_generic(tmp).with_using("xsd_parser::xml::Mixed")
        } else {
            path
        }
    }

    fn text() -> Self {
        let target_type = format_ident!("Text");
        let target_type = IdentPath::from_ident(target_type);

        Self::from_path(target_type).with_using("xsd_parser::xml::Text")
    }
}
