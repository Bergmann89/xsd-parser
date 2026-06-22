use std::collections::btree_map::Entry;

use bitflags::Flags;

use crate::models::{
    meta::{
        ComplexMeta, DeriveRelationship, DynamicMeta, ElementMeta, ElementMetaFlags, ElementMode,
        GroupMeta, MetaType, MetaTypeVariant,
    },
    schema::xs::FormChoiceType,
    IdentType, TypeIdent,
};
use crate::traits::{NameBuilderExt as _, VecHelper};

use super::{Error, Optimizer};

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
    pub fn convert_dynamic_to_choice(mut self, ident: TypeIdent) -> Result<Self, Error> {
        tracing::debug!("convert_dynamic_to_choice(ident={ident:?})");

        self.convert_dynamic_to_choice_impl(ident)?;

        Ok(self)
    }

    /// This will convert all dynamic types to choices.
    ///
    /// For details see [`convert_dynamic_to_choice`](Self::convert_dynamic_to_choice).
    pub fn convert_dynamics_to_choices(mut self) -> Self {
        tracing::debug!("convert_dynamics_to_choices");

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
            let _ = self.convert_dynamic_to_choice_impl(ident);
        }

        self
    }

    fn convert_dynamic_to_choice_impl(&mut self, ident: TypeIdent) -> Result<(), Error> {
        let Some(ty) = self.types.items.get(&ident) else {
            return Err(Error::UnknownType(ident));
        };

        let MetaTypeVariant::Dynamic(dm) = &ty.variant else {
            return Err(Error::ExpectedDynamicType(ident));
        };

        let content_name = self.types.name_builder().shared_name("Content").finish();
        let content_ident = TypeIdent::new(content_name).with_ns(ident.ns);

        let mut si = GroupMeta::default();
        let ty = self.types.items.get(&ident).unwrap();
        add_elements(&mut si, ty.form(), dm);

        let ty = self.types.items.get_mut(&ident).unwrap();
        ty.variant = MetaTypeVariant::ComplexType(ComplexMeta {
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

        Ok(())
    }
}

fn add_elements(group: &mut GroupMeta, form: FormChoiceType, meta: &DynamicMeta) {
    for (key, meta) in &meta.derived_types {
        let el = group
            .elements
            .find_or_insert(key.to_property_ident(), |ident| {
                let mut el =
                    ElementMeta::new(ident, meta.type_.clone(), ElementMode::Element, form);

                el.flags.clear();

                if let Some(display_name) = &meta.display_name {
                    el.display_name = Some(display_name.clone());
                }

                el
            });

        match key.type_ {
            IdentType::Type => {
                el.flags.insert(ElementMetaFlags::IDENTIFY_BY_TYPE);

                if meta.relationship == DeriveRelationship::ConcreteType {
                    el.flags.insert(ElementMetaFlags::DEFAULT_ELEMENT);
                }
            }
            IdentType::Element => el.flags.insert(ElementMetaFlags::IDENTIFY_BY_TAG),
            _ => (),
        }
    }
}
