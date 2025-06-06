use std::collections::HashMap;

use crate::models::types::{AttributeType, Base, ElementType, TypeVariant};

use super::{get_typedefs, Optimizer};

impl Optimizer {
    /// This will resolve all simple type definitions and use the target type
    /// directly.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema.
    /// ```xml
    #[doc = include_str!("../../../tests/optimizer/complex_flatten.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../../tests/optimizer/expected0/resolve_typedefs.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../../tests/optimizer/expected1/resolve_typedefs.rs")]
    /// ```
    pub fn resolve_typedefs(mut self) -> Self {
        tracing::debug!("resolve_typedefs");

        let typedefs = get_typedefs!(self);

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

        for type_ in self.types.values_mut() {
            match &mut type_.variant {
                TypeVariant::Reference(x) if x.is_single() => {
                    let new_type = typedefs.resolve(&x.type_).clone();
                    replaced_references
                        .entry(x.type_.clone())
                        .or_insert_with(|| new_type.clone());
                    x.type_ = new_type;
                }
                TypeVariant::Union(x) => {
                    resolve_base!(&mut x.base);

                    for x in &mut *x.types {
                        x.type_ = typedefs.resolve(&x.type_).clone();
                    }
                }
                TypeVariant::Dynamic(x) => {
                    x.type_ = x.type_.as_ref().map(|x| typedefs.resolve(x)).cloned();

                    for x in &mut x.derived_types {
                        *x = typedefs.resolve(x).clone();
                    }
                }
                TypeVariant::Enumeration(x) => {
                    resolve_base!(&mut x.base);

                    for x in &mut *x.variants {
                        if let Some(x) = &mut x.type_ {
                            *x = typedefs.resolve(x).clone();
                        }
                    }
                }
                TypeVariant::ComplexType(x) => {
                    resolve_base!(&mut x.base);

                    if let Some(ident) = &mut x.content {
                        *ident = typedefs.resolve(ident).clone();
                    }

                    for attrib in &mut *x.attributes {
                        if let AttributeType::Type(type_) = &mut attrib.type_ {
                            *type_ = typedefs.resolve(type_).clone();
                        }
                    }
                }
                TypeVariant::All(x) | TypeVariant::Choice(x) | TypeVariant::Sequence(x) => {
                    for element in &mut *x.elements {
                        if let ElementType::Type(type_) = &mut element.type_ {
                            *type_ = typedefs.resolve(type_).clone();
                        }
                    }
                }
                _ => (),
            }
        }

        for type_ in self.types.values_mut() {
            let TypeVariant::Dynamic(di) = &mut type_.variant else {
                continue;
            };

            for derived in &mut di.derived_types {
                if let Some(new_type) = replaced_references.get(derived) {
                    *derived = new_type.clone();
                }
            }
        }

        self
    }
}
