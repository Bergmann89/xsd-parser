use crate::types::{
    type_::{ComplexTypeVariant, TypeDescriptor},
    ComplexInfo, ElementInfo, ElementMode, GroupInfo, Ident, Type, Types, VecHelper,
};

use super::TypeTransformer;

/// This will use a enum that contains all known variants of the dynamic
/// type instead of a dynamic box.
///
/// # Examples
///
/// Consider the following XML schema.
/// ```xml
#[doc = include_str!("../../tests/optimizer/abstract.xsd")]
/// ```
///
/// Without this optimization this will result in the following code:
/// ```rust
#[doc = include_str!("../../tests/optimizer/expected0/convert_dynamic_to_choice.rs")]
/// ```
///
/// With this optimization the following code is generated:
/// ```rust
#[doc = include_str!("../../tests/optimizer/expected1/convert_dynamic_to_choice.rs")]
/// ```
#[derive(Debug)]
pub struct ConvertDynamicToChoice;

impl TypeTransformer for ConvertDynamicToChoice {
    type Error = super::Error;

    fn transform(&self, types: &mut Types) -> Result<(), super::Error> {
        use std::collections::btree_map::Entry;

        tracing::debug!("convert_dynamic_to_choice");

        let idents = types
            .iter()
            .filter_map(|(ident, ty)| {
                if matches!(
                    &ty,
                    Type::ComplexType(TypeDescriptor {
                        variant: ComplexTypeVariant::Dynamic(_),
                        ..
                    })
                ) {
                    Some(ident)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        for ident in idents {
            let content_name = types.name_builder().shared_name("Content").finish();
            let content_ident = Ident::new(content_name).with_ns(ident.ns);

            let type_ = types.get_mut(&ident).unwrap();
            let Type::ComplexType(TypeDescriptor {
                variant: variant @ ComplexTypeVariant::Dynamic(_),
                ..
            }) = type_
            else {
                crate::unreachable!();
            };

            let ComplexTypeVariant::Dynamic(x) = variant else {
                crate::unreachable!();
            };

            let mut si = GroupInfo::default();
            for derived in &x.derived_types {
                si.elements.find_or_insert(derived.clone(), |ident| {
                    ElementInfo::new(ident, derived.clone(), ElementMode::Element)
                });
            }

            *variant = ComplexTypeVariant::ComplexType(ComplexInfo {
                content: Some(content_ident.clone()),
                is_dynamic: true,
                ..Default::default()
            });

            match types.entry(content_ident) {
                Entry::Vacant(e) => {
                    e.insert(Type::ComplexType(TypeDescriptor::new(
                        ComplexTypeVariant::Choice(si),
                    )));
                }
                Entry::Occupied(_) => crate::unreachable!(),
            }
        }

        Ok(())
    }
}
