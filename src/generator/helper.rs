use std::collections::{BTreeMap, HashSet};

use crate::schema::xs::Use;
use crate::types::{ComplexInfo, Ident, Type, Types};

use super::misc::{Occurs, TypeRef};

/* Walk */

pub(super) struct Walk<'a> {
    types: &'a Types,
    cache: &'a BTreeMap<Ident, TypeRef>,
    visit: HashSet<Ident>,
}

impl<'a> Walk<'a> {
    pub(super) fn new(types: &'a Types, cache: &'a BTreeMap<Ident, TypeRef>) -> Self {
        Self {
            types,
            cache,
            visit: HashSet::new(),
        }
    }

    pub(super) fn is_loop(&mut self, origin: &Ident, current: &Ident) -> bool {
        if !self.visit.insert(current.clone()) {
            return false;
        }

        if origin == current {
            return true;
        }

        let mut ret = false;

        match self.types.get(current) {
            Some(Type::Reference(x)) => {
                let occurs = Occurs::from_occurs(x.min_occurs, x.max_occurs);

                ret = occurs.is_direct() && self.is_loop(origin, &x.type_);
            }
            Some(Type::Union(x)) => {
                for var in x.types.iter() {
                    if self.is_loop(origin, &var.type_) {
                        ret = true;
                        break;
                    }
                }
            }
            Some(Type::Enumeration(x)) => {
                for var in x.variants.iter() {
                    if let Some(type_) = &var.type_ {
                        if var.use_ != Use::Prohibited && self.is_loop(origin, type_) {
                            ret = true;
                            break;
                        }
                    }
                }
            }
            Some(Type::ComplexType(ComplexInfo {
                content: Some(content),
                min_occurs,
                max_occurs,
                ..
            })) => {
                let occurs = Occurs::from_occurs(*min_occurs, *max_occurs);

                ret = occurs.is_direct() && self.is_loop(origin, content);
            }
            Some(Type::All(x) | Type::Choice(x) | Type::Sequence(x)) => {
                for f in x.elements.iter() {
                    let already_boxed = self
                        .cache
                        .get(current)
                        .is_some_and(|x| x.boxed_elements.contains(&f.ident));
                    if already_boxed {
                        break;
                    }

                    let occurs = Occurs::from_occurs(f.min_occurs, f.max_occurs);
                    if occurs.is_direct() && self.is_loop(origin, &f.type_) {
                        ret = true;
                        break;
                    }
                }
            }
            _ => (),
        }

        self.visit.remove(current);

        ret
    }
}
