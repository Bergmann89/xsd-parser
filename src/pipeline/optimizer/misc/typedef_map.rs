use std::collections::HashMap;

use crate::models::{
    types::{TypeVariant, Types},
    Ident,
};

#[derive(Debug)]
pub(crate) struct TypedefMap(HashMap<Ident, Ident>);

impl TypedefMap {
    pub(crate) fn new(types: &Types) -> Self {
        let mut ret = HashMap::new();

        for (ident, type_) in &types.types {
            if let TypeVariant::Reference(x) = &type_.variant {
                if x.is_single() {
                    ret.insert(ident.clone(), x.type_.clone());
                }
            }
        }

        Self(ret)
    }

    pub(crate) fn resolve<'a>(&'a self, ident: &'a Ident) -> &'a Ident {
        let x = self.0.get(ident).map_or(ident, |x| self.resolve(x));

        x
    }
}
