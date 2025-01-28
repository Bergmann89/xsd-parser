use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use tracing::instrument;

use super::super::data::{
    AttributeData, ComplexTypeData, DynamicData, EnumerationData, ReferenceData, UnionData,
};
use super::super::misc::StateFlags;

pub(crate) struct ImplRenderer;

impl ImplRenderer {
    #[instrument(level = "trace", skip(self))]
    pub fn render_union(&mut self, data: &mut UnionData<'_, '_>) {
        let _self = self;

        data.current_type_ref_mut()
            .add_flag_checked(StateFlags::HAS_TYPE);
    }

    #[instrument(level = "trace", skip(self))]
    pub fn render_dynamic(&mut self, data: &mut DynamicData<'_, '_>) {
        let _self = self;

        data.current_type_ref_mut()
            .add_flag_checked(StateFlags::HAS_TYPE);
    }

    #[instrument(level = "trace", skip(self))]
    pub fn render_reference(&mut self, data: &mut ReferenceData<'_, '_>) {
        let _self = self;

        data.current_type_ref_mut()
            .add_flag_checked(StateFlags::HAS_TYPE);
    }

    #[instrument(level = "trace", skip(self))]
    pub fn render_enumeration(&mut self, data: &mut EnumerationData<'_, '_>) {
        let _self = self;

        data.current_type_ref_mut()
            .add_flag_checked(StateFlags::HAS_TYPE);
    }

    #[instrument(level = "trace", skip(self))]
    pub fn render_complex_type(&mut self, data: &mut ComplexTypeData<'_, '_>) {
        let type_ref = data.current_type_ref();
        let type_ident = &type_ref.type_ident;
        let mut has_attributes = false;
        let attribute_defaults = data
            .attributes
            .iter()
            .filter_map(AttributeData::render_attribute_default_fn)
            .inspect(|_| has_attributes = true);

        let code = quote! {
            impl #type_ident {
                #( #attribute_defaults )*
            }
        };

        if has_attributes {
            data.add_code(code);
        }
    }
}

impl AttributeData<'_> {
    fn render_attribute_default_fn(&self) -> Option<TokenStream> {
        let default = self.default_value.as_ref()?;
        let target_type = &self.target_type;
        let default_fn_ident = format_ident!("default_{}", self.field_ident);

        Some(quote! {
            #[must_use]
            pub fn #default_fn_ident() -> #target_type {
                #default
            }
        })
    }
}
