use std::collections::HashMap;

use crate::types::{
    Base, ComplexTypeVariant, Ident, SimpleTypeVariant, Type, TypeDescriptor, Types,
};

#[derive(Debug)]
pub(crate) struct BaseMap(HashMap<Ident, Base>);

impl BaseMap {
    pub(crate) fn new(types: &Types) -> Self {
        let mut ret = HashMap::new();

        for (ident, type_) in &types.types {
            match type_ {
                Type::SimpleType(TypeDescriptor {
                    variant: SimpleTypeVariant::Enumeration(ei),
                    ..
                }) => {
                    if matches!(
                        ei.base
                            .as_ident()
                            .and_then(|base| types.get_simple_type(base).map(|a| &a.variant)),
                        Some(SimpleTypeVariant::Enumeration(_))
                    ) {
                        ret.insert(ident.clone(), ei.base.clone());
                    }
                }
                Type::SimpleType(TypeDescriptor {
                    variant: SimpleTypeVariant::Union(ei),
                    ..
                }) => {
                    if matches!(
                        ei.base
                            .as_ident()
                            .and_then(|base| types.get_simple_type(base).map(|a| &a.variant)),
                        Some(SimpleTypeVariant::Union(_))
                    ) {
                        ret.insert(ident.clone(), ei.base.clone());
                    }
                }
                Type::ComplexType(TypeDescriptor {
                    variant: ComplexTypeVariant::ComplexType(ci),
                    ..
                }) => {
                    if matches!(
                        ci.base
                            .as_ident()
                            .and_then(|base| types.get_complex_type(base).map(|a| &a.variant)),
                        Some(ComplexTypeVariant::ComplexType(_))
                    ) {
                        ret.insert(ident.clone(), ci.base.clone());
                    }
                }
                _ => (),
            }
        }

        Self(ret)
    }

    pub(crate) fn get_unrestricted<'a>(&'a self, ident: &'a Ident) -> &'a Ident {
        match self.0.get(ident) {
            Some(Base::Restriction(base)) => self.get_unrestricted(base),
            _ => ident,
        }
    }
}
