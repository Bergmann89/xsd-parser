use crate::{
    schema::{MaxOccurs, MinOccurs},
    types::{ElementMode, GroupInfo, Ident, Type, VecHelper},
};

use super::{Error, Optimizer};

struct FlattenComplexInfo {
    info: GroupInfo,
    count: usize,
    is_choice: bool,
}

impl Optimizer {
    /// This will flatten the nested groups (`xs::all`, `xs::choice` or `xs::sequence`)
    /// of the complex type identified by `ident` to one type instead of rendering
    /// nested structures.
    ///
    /// # Errors
    ///
    /// Returns an error if the passed `ident` could not be found, the referenced
    /// type is not complex type or the complex type has no content.
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
    #[doc = include_str!("../../tests/optimizer/expected0/flatten_complex_types.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected1/flatten_complex_types.rs")]
    /// ```
    pub fn flatten_complex_type(mut self, ident: Ident) -> Result<Self, Error> {
        tracing::debug!("flatten_complex_type(ident={ident:?})");

        let Some(ty) = self.types.get(&ident) else {
            return Err(Error::UnknownType(ident));
        };

        let Type::ComplexType(ci) = ty else {
            return Err(Error::ExpectedComplexType(ident));
        };

        let Some(content_ident) = ci.content.clone() else {
            return Err(Error::MissingContentType(ident));
        };

        let mut info = FlattenComplexInfo {
            info: GroupInfo::default(),
            count: 0,
            is_choice: false,
        };

        self.flatten_complex_type_impl(&content_ident, 1, MaxOccurs::Bounded(1), &mut info);

        if info.count > 1 {
            let type_ = if info.is_choice {
                Type::Choice(info.info)
            } else {
                Type::Sequence(info.info)
            };

            self.types.insert(content_ident, type_);
        }

        Ok(self)
    }

    /// This will flatten all complex types.
    ///
    /// For details see [`flatten_complex_type`](Self::flatten_complex_type).
    pub fn flatten_complex_types(mut self) -> Self {
        tracing::debug!("flatten_complex_types");

        let idents = self
            .types
            .iter()
            .filter_map(|(ident, type_)| {
                if matches!(type_, Type::ComplexType(ci) if ci.has_complex_content(&self.types)) {
                    Some(ident)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        for ident in idents {
            self = self.flatten_complex_type(ident).unwrap();
        }

        self
    }

    fn flatten_complex_type_impl(
        &self,
        ident: &Ident,
        min: MinOccurs,
        max: MaxOccurs,
        next: &mut FlattenComplexInfo,
    ) {
        let Some(type_) = self.types.get(ident) else {
            return;
        };

        let mut is_choice = false;
        let si = match type_ {
            Type::Choice(si) => {
                is_choice = true;
                next.is_choice = true;

                si
            }
            Type::All(si) | Type::Sequence(si) => si,
            Type::Reference(ti) if ti.is_single() => {
                self.flatten_complex_type_impl(
                    &ti.type_,
                    min.min(ti.min_occurs),
                    max.max(ti.max_occurs),
                    next,
                );

                return;
            }
            x => crate::unreachable!("{x:#?}"),
        };

        next.count += 1;

        for x in &*si.elements {
            match x.element_mode {
                ElementMode::Element => {
                    let element = next
                        .info
                        .elements
                        .find_or_insert(x.ident.clone(), |_| x.clone());
                    element.min_occurs = min.min(x.min_occurs);
                    element.max_occurs = max.max(x.max_occurs);
                }
                ElementMode::Group => {
                    let (min, max) = if is_choice {
                        (min.min(x.min_occurs), max.max(x.max_occurs))
                    } else {
                        (min * x.min_occurs, max * x.max_occurs)
                    };

                    self.flatten_complex_type_impl(&x.type_, min, max, next);
                }
            }
        }

        if let Some(any) = &si.any {
            next.info.any = Some(any.clone());
        }
    }
}
