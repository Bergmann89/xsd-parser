use crate::models::{
    meta::{ComplexMeta, ElementMeta, ElementMode, GroupMeta, MetaType, MetaTypeVariant},
    Ident,
};
use crate::traits::VecHelper;

use super::Optimizer;

impl Optimizer {
    /// This will use a enum that contains all known variants of the dynamic
    /// type instead of a dynamic box.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema.
    /// ```xml
    #[doc = include_str!("../../../tests/optimizer/abstract.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../../tests/optimizer/expected0/convert_dynamic_to_choice.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../../tests/optimizer/expected1/convert_dynamic_to_choice.rs")]
    /// ```
    pub fn convert_dynamic_to_choice(mut self) -> Self {
        use std::collections::btree_map::Entry;

        tracing::debug!("convert_dynamic_to_choice");

        let idents = self
            .types
            .items
            .iter()
            .filter_map(|(ident, ty)| {
                if matches!(&ty.variant, MetaTypeVariant::Dynamic(_)) {
                    Some(ident)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        for ident in idents {
            let content_name = self.types.name_builder().shared_name("Content").finish();
            let content_ident = Ident::new(content_name).with_ns(ident.ns);

            let type_ = self.types.items.get_mut(&ident).unwrap();
            let MetaTypeVariant::Dynamic(x) = &mut type_.variant else {
                crate::unreachable!();
            };

            let mut si = GroupMeta::default();
            for derived in &x.derived_types {
                si.elements.find_or_insert(derived.clone(), |ident| {
                    ElementMeta::new(ident, derived.clone(), ElementMode::Element)
                });
            }

            type_.variant = MetaTypeVariant::ComplexType(ComplexMeta {
                content: Some(content_ident.clone()),
                is_dynamic: true,
                ..Default::default()
            });

            match self.types.items.entry(content_ident) {
                Entry::Vacant(e) => {
                    e.insert(MetaType::new(MetaTypeVariant::Choice(si)));
                }
                Entry::Occupied(_) => crate::unreachable!(),
            }
        }

        self
    }
}
