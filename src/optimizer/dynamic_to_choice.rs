use crate::types::{ComplexInfo, ElementInfo, ElementMode, GroupInfo, Ident, Type, VecHelper};

use super::Optimizer;

impl Optimizer {
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
    pub fn convert_dynamic_to_choice(mut self) -> Self {
        use std::collections::btree_map::Entry;

        tracing::debug!("convert_dynamic_to_choice");

        let idents = self
            .types
            .iter()
            .filter_map(|(ident, ty)| {
                if matches!(ty, Type::Dynamic(_)) {
                    Some(ident)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        for ident in idents {
            let content_ident = Ident::new(self.types.make_unnamed()).with_ns(ident.ns);

            let type_ = self.types.get_mut(&ident).unwrap();
            let Type::Dynamic(x) = type_ else {
                crate::unreachable!();
            };

            let mut si = GroupInfo::default();
            for derived in &x.derived_types {
                si.elements.find_or_insert(derived.clone(), |ident| {
                    ElementInfo::new(ident, derived.clone(), ElementMode::Element)
                });
            }

            *type_ = Type::ComplexType(ComplexInfo {
                content: Some(content_ident.clone()),
                is_dynamic: true,
                ..Default::default()
            });

            match self.types.entry(content_ident) {
                Entry::Vacant(e) => {
                    e.insert(Type::Choice(si));
                }
                Entry::Occupied(_) => crate::unreachable!(),
            }
        }

        self
    }
}
