use proc_macro2::Literal;
use quote::format_ident;

use crate::{
    config::GeneratorFlags,
    models::{
        data::{ConstrainsData, EnumerationData, EnumerationDataVariant, EnumerationVariantValue},
        meta::{BuildInMeta, EnumerationMeta, EnumerationMetaVariant, MetaTypeVariant, MetaTypes},
        schema::xs::Use,
        Name, TypeIdent,
    },
    pipeline::generator::ValueGeneratorMode,
};

use super::super::{Context, Error};

impl<'types> EnumerationData<'types> {
    pub(super) fn new(
        meta: &'types EnumerationMeta,
        ctx: &mut Context<'_, 'types>,
    ) -> Result<Self, Error> {
        let mut unknown = 0usize;
        let constrains = ConstrainsData::new(&meta.constrains, None, ctx)?;
        let type_ident = ctx.current_type_ref().path.ident().clone();
        let trait_impls = ctx.make_trait_impls()?;

        let simple_base_type = ctx
            .check_generator_flags(GeneratorFlags::ADVANCED_ENUMS)
            .then_some(())
            .and_then(|()| meta.base.as_ident())
            .and_then(|base| ctx.types.get_simple_enum_base_type(base));

        let variants = meta
            .variants
            .iter()
            .filter_map(|var| var.make_variant(ctx, &mut unknown, simple_base_type.as_ref()))
            .collect::<Result<Vec<_>, _>>()?;

        let simple_base_type = simple_base_type
            .map(|base| ctx.get_or_create_type_ref_for_value(&base, true))
            .transpose()?
            .map(|x| x.path.clone());

        Ok(EnumerationData {
            meta,
            constrains,
            type_ident,
            variants,
            trait_impls,
            simple_base_type,
        })
    }
}

impl EnumerationMetaVariant {
    fn make_variant<'types>(
        &'types self,
        ctx: &mut Context<'_, 'types>,
        unknown: &mut usize,
        simple_base: Option<&TypeIdent>,
    ) -> Option<Result<EnumerationDataVariant<'types>, Error>> {
        match self.use_ {
            Use::Prohibited => None,
            Use::Required | Use::Optional => {
                let types = ctx.types;
                let type_ref = if let Some(t) = &self.type_ {
                    match ctx.get_or_create_type_ref_for_value(t, true) {
                        Ok(target_ref) => Some(target_ref),
                        Err(error) => return Some(Err(error)),
                    }
                } else {
                    None
                };

                let variant_ident = if let Some(display_name) = self.display_name.as_deref() {
                    format_ident!("{display_name}")
                } else if let (Some(type_ref), true) = (type_ref, self.ident.name.is_generated()) {
                    type_ref.path.ident().clone()
                } else if self.ident.name.as_str().is_empty() {
                    *unknown += 1;

                    types.naming.make_unknown_variant(*unknown)
                } else {
                    types
                        .naming
                        .format_variant_ident(&self.ident.name, self.display_name.as_deref())
                };

                let s_name = self.ident.name.to_string();
                let b_name = Literal::byte_string(s_name.as_bytes());
                let target_type = type_ref.map(|x| x.path.clone());
                let extra_attributes = Vec::new();

                let value = self.ident.name.as_str();
                let value = simple_base
                    .and_then(|base| {
                        ctx.make_value_renderer(base, value, ValueGeneratorMode::Literal)
                            .ok()
                            .map(|code| {
                                let ident = types.naming.format_constant_ident(
                                    &Name::new_named(variant_ident.to_string()),
                                    None,
                                );

                                EnumerationVariantValue::ByteLiteral(ident, code)
                            })
                    })
                    .or_else(|| {
                        simple_base
                            .and_then(|base| {
                                ctx.make_value_renderer(base, value, ValueGeneratorMode::Constant)
                                    .ok()
                            })
                            .map(|code| {
                                let ident = types.naming.format_constant_ident(
                                    &Name::new_named(variant_ident.to_string()),
                                    None,
                                );

                                EnumerationVariantValue::Constant(ident, code)
                            })
                    })
                    .unwrap_or(EnumerationVariantValue::None);

                Some(Ok(EnumerationDataVariant {
                    meta: self,
                    s_name,
                    b_name,
                    value,
                    variant_ident,
                    target_type,
                    extra_attributes,
                }))
            }
        }
    }
}

impl MetaTypes {
    fn get_simple_enum_base_type(&self, ident: &TypeIdent) -> Option<TypeIdent> {
        match self.get_variant(ident)? {
            MetaTypeVariant::BuildIn(BuildInMeta::String) => Some(TypeIdent::STR),
            MetaTypeVariant::BuildIn(_)
            | MetaTypeVariant::Custom(_)
            | MetaTypeVariant::SimpleType(_) => Some(ident.clone()),
            MetaTypeVariant::Enumeration(x) => {
                let base = x.base.as_ident()?;
                self.get_simple_enum_base_type(base)
            }
            _ => None,
        }
    }
}
