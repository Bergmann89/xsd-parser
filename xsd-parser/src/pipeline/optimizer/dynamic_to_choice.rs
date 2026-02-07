use crate::models::{
    meta::{ComplexMeta, ElementMeta, ElementMode, GroupMeta, MetaType, MetaTypeVariant},
    TypeIdent,
};
use crate::traits::{NameBuilderExt as _, VecHelper};

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
            let content_ident = TypeIdent::new(content_name).with_ns(ident.ns);

            let mut si = GroupMeta::default();
            let type_ = self.types.items.get(&ident).unwrap();
            self.add_elements(&mut si, type_);

            let type_ = self.types.items.get_mut(&ident).unwrap();
            type_.variant = MetaTypeVariant::ComplexType(ComplexMeta {
                content: Some(content_ident.clone()),
                is_dynamic: true,
                ..Default::default()
            });

            match self.types.items.entry(content_ident) {
                Entry::Vacant(e) => {
                    e.insert(MetaType::new(if si.elements.is_empty() {
                        MetaTypeVariant::Sequence(si)
                    } else {
                        MetaTypeVariant::Choice(si)
                    }));
                }
                Entry::Occupied(_) => crate::unreachable!(),
            }
        }

        self
    }

    fn add_elements(&self, group: &mut GroupMeta, ty: &MetaType) {
        let form = ty.form();
        let MetaTypeVariant::Dynamic(x) = &ty.variant else {
            crate::unreachable!();
        };

        for meta in &x.derived_types {
            let derived_ty = self.types.get_resolved_type(&meta.type_).unwrap();
            if let MetaTypeVariant::Dynamic(_) = &derived_ty.variant {
                self.add_elements(group, derived_ty);
            } else {
                group
                    .elements
                    .find_or_insert(meta.type_.to_property_ident(), |ident| {
                        let mut el =
                            ElementMeta::new(ident, meta.type_.clone(), ElementMode::Element, form);

                        if let Some(display_name) = &meta.display_name {
                            el.display_name = Some(display_name.clone());
                        }

                        el
                    });
            }
        }
    }
}
