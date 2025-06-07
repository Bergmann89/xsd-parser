use std::collections::HashMap;

use crate::models::{
    meta::{Base, MetaTypeVariant, MetaTypes},
    Ident,
};

#[derive(Debug)]
pub(crate) struct BaseMap(HashMap<Ident, Base>);

impl BaseMap {
    pub(crate) fn new(types: &MetaTypes) -> Self {
        let mut ret = HashMap::new();

        for (ident, type_) in &types.items {
            match &type_.variant {
                MetaTypeVariant::Enumeration(ei) => {
                    if matches!(
                        ei.base.as_ident().and_then(|base| types.get_variant(base)),
                        Some(MetaTypeVariant::Enumeration(_))
                    ) {
                        ret.insert(ident.clone(), ei.base.clone());
                    }
                }
                MetaTypeVariant::Union(ei) => {
                    if matches!(
                        ei.base.as_ident().and_then(|base| types.get_variant(base)),
                        Some(MetaTypeVariant::Union(_))
                    ) {
                        ret.insert(ident.clone(), ei.base.clone());
                    }
                }
                MetaTypeVariant::ComplexType(ci) => {
                    if matches!(
                        ci.base.as_ident().and_then(|base| types.get_variant(base)),
                        Some(MetaTypeVariant::ComplexType(_))
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
