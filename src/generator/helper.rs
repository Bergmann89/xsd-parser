use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::str::FromStr;

use proc_macro2::{Ident as Ident2, TokenStream};
use quote::quote;

use crate::generator::misc::IdentPath;
use crate::schema::xs::Use;
use crate::types::{ComplexInfo, Ident, TypeVariant, Types};

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

        match self.types.get_variant(current) {
            Some(TypeVariant::Reference(x)) => {
                let occurs = Occurs::from_occurs(x.min_occurs, x.max_occurs);

                ret = occurs.is_direct() && self.is_loop(origin, &x.type_);
            }
            Some(TypeVariant::Union(x)) => {
                for var in x.types.iter() {
                    if self.is_loop(origin, &var.type_) {
                        ret = true;
                        break;
                    }
                }
            }
            Some(TypeVariant::Enumeration(x)) => {
                for var in x.variants.iter() {
                    if let Some(type_) = &var.type_ {
                        if var.use_ != Use::Prohibited && self.is_loop(origin, type_) {
                            ret = true;
                            break;
                        }
                    }
                }
            }
            Some(TypeVariant::ComplexType(ComplexInfo {
                content: Some(content),
                min_occurs,
                max_occurs,
                ..
            })) => {
                let occurs = Occurs::from_occurs(*min_occurs, *max_occurs);

                ret = occurs.is_direct() && self.is_loop(origin, content);
            }
            Some(TypeVariant::All(x) | TypeVariant::Choice(x) | TypeVariant::Sequence(x)) => {
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

pub(super) fn render_usings<I>(usings: I) -> TokenStream
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    #[derive(Default)]
    struct Module {
        usings: BTreeSet<Ident2>,
        sub_modules: BTreeMap<Ident2, Module>,
    }

    impl Module {
        fn render(&self) -> TokenStream {
            let count = self.usings.len() + self.sub_modules.len();

            let usings = self.usings.iter().map(|ident| quote!(#ident));
            let sub_modules = self.sub_modules.iter().map(|(ident, module)| {
                let using = module.render();

                quote!(#ident::#using)
            });

            let items = usings.chain(sub_modules);

            if count > 1 {
                quote!({ #( #items ),* })
            } else {
                quote!(#( #items )*)
            }
        }
    }

    let mut root = Module::default();

    for using in usings {
        let using = using.as_ref();
        let Ok(ident) = IdentPath::from_str(using) else {
            continue;
        };

        let (ident, path) = ident.into_parts();

        let mut module = &mut root;
        for part in path.into_iter().flat_map(|x| x.0) {
            module = module.sub_modules.entry(part).or_default();
        }

        module.usings.insert(ident);
    }

    let mut ret = TokenStream::new();
    for (ident, module) in &root.sub_modules {
        let using = module.render();
        ret.extend(quote!(use #ident::#using;));
    }

    ret
}
