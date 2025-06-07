mod complex;
mod dynamic;
mod enumeration;
mod reference;
mod type_;
mod union;

use proc_macro2::TokenStream;
use quote::quote;

use crate::models::{
    data::{BuildInData, CustomData, Occurs},
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
