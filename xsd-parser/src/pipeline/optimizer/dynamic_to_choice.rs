use crate::models::{
    meta::{ComplexMeta, ElementMeta, ElementMode, GroupMeta, MetaType, MetaTypeVariant},
    Ident,
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
        tracing::debug!("convert_dynamic_to_choice");

        let idents = self
            .types
            .iter_items()
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
            let content_ident = Ident::new(content_name)
                .with_ns(ident.ns)
                .with_schema(ident.schema);

            let mut si = GroupMeta::default();
            let type_ = self.types.get_type(&ident).unwrap();
            self.add_elements(&mut si, type_);

            let type_ = self.types.get_type_mut(&ident).unwrap();
            type_.variant = MetaTypeVariant::ComplexType(ComplexMeta {
                content: Some(content_ident.clone()),
                is_dynamic: true,
                ..Default::default()
            });

            assert!(!self.types.contains_exact_type(&content_ident));
            self.types
                .insert_type(content_ident, MetaType::new(MetaTypeVariant::Choice(si)));
        }

        self
    }

    fn add_elements(&self, group: &mut GroupMeta, ty: &MetaType) {
        let form = ty.form();
        let MetaTypeVariant::Dynamic(x) = &ty.variant else {
            crate::unreachable!();
        };

        for derived in &x.derived_types {
            let derived_ty = self.types.get_resolved_type(derived).unwrap();
            if let MetaTypeVariant::Dynamic(_) = &derived_ty.variant {
                self.add_elements(group, derived_ty);
            } else {
                group.elements.find_or_insert(derived.clone(), |ident| {
                    ElementMeta::new(ident, derived.clone(), ElementMode::Element, form)
                });
            }
        }
    }
}
