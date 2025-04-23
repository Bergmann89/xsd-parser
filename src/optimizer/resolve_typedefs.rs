use std::collections::HashMap;

use crate::types::{Base, ComplexTypeVariant, SimpleTypeVariant, Types};

use super::{Error, TypeTransformer};

/// This will resolve all simple type definitions and use the target type
/// directly.
///
/// # Examples
///
/// Consider the following XML schema.
/// ```xml
#[doc = include_str!("../../tests/optimizer/complex_flatten.xsd")]
/// ```
///
/// Without this optimization this will result in the following code:
/// ```rust
#[doc = include_str!("../../tests/optimizer/expected0/resolve_typedefs.rs")]
/// ```
///
/// With this optimization the following code is generated:
/// ```rust
#[doc = include_str!("../../tests/optimizer/expected1/resolve_typedefs.rs")]
/// ```
#[derive(Debug)]
pub struct ResolveTypedefs;

impl TypeTransformer for ResolveTypedefs {
    type Error = super::Error;

    fn transform(&self, types: &mut Types) -> Result<(), Error> {
        tracing::debug!("resolve_typedefs");

        let typedefs = crate::optimizer::TypedefMap::new(types);

        macro_rules! resolve_base {
            ($base:expr) => {
                match &mut $base {
                    Base::None => (),
                    Base::Extension(x) => *x = typedefs.resolve(x).clone(),
                    Base::Restriction(x) => *x = typedefs.resolve(x).clone(),
                }
            };
        }

        let mut replaced_references = HashMap::new();

        for (_, type_) in types.simple_types_iter_mut() {
            match &mut type_.variant {
                SimpleTypeVariant::Reference(x) if x.is_single() => {
                    let new_type = typedefs.resolve(&x.type_).clone();
                    replaced_references
                        .entry(x.type_.clone())
                        .or_insert_with(|| new_type.clone());
                    x.type_ = new_type;
                }
                SimpleTypeVariant::Union(x) => {
                    resolve_base!(&mut x.base);

                    for x in &mut *x.types {
                        x.type_ = typedefs.resolve(&x.type_).clone();
                    }
                }
                SimpleTypeVariant::Enumeration(x) => {
                    resolve_base!(&mut x.base);

                    for x in &mut *x.variants {
                        if let Some(x) = &mut x.type_ {
                            *x = typedefs.resolve(x).clone();
                        }
                    }
                }
                _ => (),
            }
        }

        for (_, type_) in types.complex_types_iter_mut() {
            match &mut type_.variant {
                ComplexTypeVariant::Reference(x) if x.is_single() => {
                    let new_type = typedefs.resolve(&x.type_).clone();
                    replaced_references
                        .entry(x.type_.clone())
                        .or_insert_with(|| new_type.clone());
                    x.type_ = new_type;
                }
                ComplexTypeVariant::Dynamic(x) => {
                    x.type_ = x.type_.as_ref().map(|x| typedefs.resolve(x)).cloned();

                    for x in &mut x.derived_types {
                        *x = typedefs.resolve(x).clone();
                    }
                }
                ComplexTypeVariant::ComplexType(x) => {
                    resolve_base!(&mut x.base);

                    if let Some(ident) = &mut x.content {
                        *ident = typedefs.resolve(ident).clone();
                    }

                    for attrib in &mut *x.attributes {
                        attrib.type_ = typedefs.resolve(&attrib.type_).clone();
                    }
                }
                ComplexTypeVariant::All(x)
                | ComplexTypeVariant::Choice(x)
                | ComplexTypeVariant::Sequence(x) => {
                    for element in &mut *x.elements {
                        element.type_ = typedefs.resolve(&element.type_).clone();
                    }
                }
                _ => (),
            }
        }

        for (_, type_) in types.complex_types_iter_mut() {
            let ComplexTypeVariant::Dynamic(di) = &mut type_.variant else {
                continue;
            };

            for derived in &mut di.derived_types {
                if let Some(new_type) = replaced_references.get(derived) {
                    *derived = new_type.clone();
                }
            }
        }

        Ok(())
    }
}
