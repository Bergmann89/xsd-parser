use std::collections::HashMap;

use crate::models::{
    meta::{MetaTypeVariant, MetaTypes},
    Ident,
};

#[derive(Debug)]
pub(crate) struct TypedefMap(HashMap<Ident, Ident>);

impl TypedefMap {
    pub(crate) fn new(types: &MetaTypes) -> Self {
        let mut ret = HashMap::new();

        for (ident, type_) in &types.items {
            if let MetaTypeVariant::Reference(x) = &type_.variant {
                if x.is_simple() {
                    ret.insert(ident.clone(), x.type_.clone());
                    ret.insert(ident.clone().with_schema(None), x.type_.clone());
                }
            }
        }

        Self(ret)
    }

    pub(crate) fn resolve<'a>(&'a self, ident: &'a Ident) -> &'a Ident {
        let x = self
            .0
            .get(ident)
            .or_else(|| {
                self.0
                    .get(&ident.clone().with_schema(None))
                    .filter(|&o| o != ident)
            })
            .map_or(ident, |x| self.resolve(x));

        x
    }
}
