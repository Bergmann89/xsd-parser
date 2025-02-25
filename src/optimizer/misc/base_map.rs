use std::collections::HashMap;

use crate::types::{Base, Ident, Type, Types};

#[derive(Debug)]
pub(crate) struct BaseMap(HashMap<Ident, Base>);

impl BaseMap {
    pub(crate) fn new(types: &Types) -> Self {
        let mut ret = HashMap::new();

        for (ident, type_) in &types.types {
            match type_ {
                Type::Enumeration(ei) => {
                    if matches!(
                        ei.base.as_ident().and_then(|base| types.get(base)),
                        Some(Type::Enumeration(_))
                    ) {
                        ret.insert(ident.clone(), ei.base.clone());
                    }
                }
                Type::Union(ei) => {
                    if matches!(
                        ei.base.as_ident().and_then(|base| types.get(base)),
                        Some(Type::Union(_))
                    ) {
                        ret.insert(ident.clone(), ei.base.clone());
                    }
                }
                Type::ComplexType(ci) => {
                    if matches!(
                        ci.base.as_ident().and_then(|base| types.get(base)),
                        Some(Type::ComplexType(_))
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
