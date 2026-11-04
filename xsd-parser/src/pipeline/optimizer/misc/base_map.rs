use std::collections::HashMap;

use crate::models::{
    meta::{Base, MetaTypeVariant, MetaTypes},
    TypeIdent,
};

#[derive(Debug)]
pub(crate) struct BaseMap(HashMap<TypeIdent, Base>);

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
                MetaTypeVariant::SimpleType(si) => {
                    ret.insert(ident.clone(), Base::Restriction(si.base.clone()));
                }
                MetaTypeVariant::ComplexType(ci) => {
                    let base = match &ci.base {
                        Base::None => None,
                        Base::Extension(x) => complex_base(types, x).map(Base::Extension),
                        Base::Restriction(x) => complex_base(types, x).map(Base::Restriction),
                    };

                    if let Some(base) = base {
                        ret.insert(ident.clone(), base);
                    }
                }
                _ => (),
            }
        }

        Self(ret)
    }

    pub(crate) fn get_unrestricted<'a>(&'a self, ident: &'a TypeIdent) -> &'a TypeIdent {
        match self.0.get(ident) {
            Some(Base::Restriction(base)) => self.get_unrestricted(base),
            _ => ident,
        }
    }
}

/// Returns the complex type of `ident`. For a dynamic type this is the type
/// that stores its content.
fn complex_base(types: &MetaTypes, ident: &TypeIdent) -> Option<TypeIdent> {
    match types.get_variant(ident)? {
        MetaTypeVariant::ComplexType(_) => Some(ident.clone()),
        MetaTypeVariant::Dynamic(dm) => dm
            .type_
            .clone()
            .filter(|x| matches!(types.get_variant(x), Some(MetaTypeVariant::ComplexType(_)))),
        _ => None,
    }
}
