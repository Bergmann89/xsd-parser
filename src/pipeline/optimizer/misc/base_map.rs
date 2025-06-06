use std::collections::HashMap;

use crate::models::{
    types::{Base, TypeVariant, Types},
    Ident,
};

#[derive(Debug)]
pub(crate) struct BaseMap(HashMap<Ident, Base>);

impl BaseMap {
    pub(crate) fn new(types: &Types) -> Self {
        let mut ret = HashMap::new();

        for (ident, type_) in &types.types {
            match &type_.variant {
                TypeVariant::Enumeration(ei) => {
                    if matches!(
                        ei.base.as_ident().and_then(|base| types.get_variant(base)),
                        Some(TypeVariant::Enumeration(_))
                    ) {
                        ret.insert(ident.clone(), ei.base.clone());
                    }
                }
                TypeVariant::Union(ei) => {
                    if matches!(
                        ei.base.as_ident().and_then(|base| types.get_variant(base)),
                        Some(TypeVariant::Union(_))
                    ) {
                        ret.insert(ident.clone(), ei.base.clone());
                    }
                }
                TypeVariant::ComplexType(ci) => {
                    if matches!(
                        ci.base.as_ident().and_then(|base| types.get_variant(base)),
                        Some(TypeVariant::ComplexType(_))
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
