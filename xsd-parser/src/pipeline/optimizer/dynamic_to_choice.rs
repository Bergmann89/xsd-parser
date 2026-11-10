use std::collections::{btree_map::Entry, BTreeSet};

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
    /// Abstract descendants are excluded. A type without concrete descendants
    /// becomes an empty choice, representing an uninhabited type.
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

        let abstract_types = self.abstract_types.get_or_insert_with(|| {
            self.types
                .items
                .iter()
                .filter_map(|(ident, ty)| match &ty.variant {
                    MetaTypeVariant::Dynamic(meta) if meta.is_abstract => Some(ident.clone()),
                    _ => None,
                })
                .collect()
        });
        let content_name = self.types.name_builder().shared_name("Content").finish();
        let content_ident = TypeIdent::new(content_name).with_ns(ident.ns);

        let mut si = GroupMeta::default();
        add_elements(&mut si, dm, abstract_types);

        let ty = self.types.items.get_mut(&ident).unwrap();
        ty.variant = MetaTypeVariant::ComplexType(ComplexMeta {
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

        Ok(())
    }
}

fn add_elements(group: &mut GroupMeta, meta: &DynamicMeta, abstract_types: &BTreeSet<TypeIdent>) {
    for (key, meta) in &meta.derived_types {
        if abstract_types.contains(key) {
            continue;
        }
        let el = group
            .elements
            .find_or_insert(key.to_property_ident(), |ident| {
                // Substitution group members are global elements and `xsi:type` values are
                // type names, both are always namespace-qualified.
                let mut el = ElementMeta::new(
                    ident,
                    meta.type_.clone(),
                    ElementMode::Element,
                    FormChoiceType::Qualified,
                );

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
