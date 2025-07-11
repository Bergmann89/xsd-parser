use std::mem::take;

use smallvec::{smallvec, SmallVec};

use crate::{
    models::{
        meta::{ElementMeta, MetaTypeVariant},
        schema::MaxOccurs,
        Ident,
    },
    Name,
};

use super::{Error, Optimizer};

impl Optimizer {
    pub fn simplify_mixed_type(mut self, ident: Ident) -> Result<Self, Error> {
        tracing::debug!("simplify_mixed_type(ident={ident:?})");

        let Some(ty) = self.types.items.get(&ident) else {
            return Err(Error::UnknownType(ident));
        };

        let MetaTypeVariant::ComplexType(ci) = &ty.variant else {
            return Err(Error::ExpectedComplexType(ident));
        };

        let max_occurs = ci.max_occurs;
        let Some(content_ident) = ci.content.clone() else {
            return Err(Error::MissingContentType(ident));
        };

        let Some(content) = self.types.items.get_mut(&content_ident) else {
            return Err(Error::MissingContentType(ident));
        };

        match &mut content.variant {
            MetaTypeVariant::Choice(gi) => {
                if max_occurs != MaxOccurs::Unbounded {
                    return Err(Error::ExpectedUnboundContent(ident));
                }

                gi.elements.push(ElementMeta::text(Ident::element("text")));
            }
            MetaTypeVariant::Sequence(gi) => {
                let mut text_before = ElementMeta::text(Ident::element("text_before"));
                text_before.min_occurs = 0;

                let pairs = take(&mut gi.elements.0).into_iter().map(
                    |e| -> SmallVec<[Result<ElementMeta, Error>; 2]> {
                        if e.min_occurs != 1 || e.max_occurs != MaxOccurs::Bounded(1) {
                            return smallvec![Err(Error::UnexpectedElementInContent(
                                ident.clone()
                            ))];
                        }

                        let name = Name::new_named(format!("text_after_{}", e.ident.name));
                        let ident = Ident::new(name);

                        let mut text_after = ElementMeta::text(ident);
                        text_after.min_occurs = 0;

                        smallvec![Ok(e), Ok(text_after)]
                    },
                );

                gi.elements.0 = Some(Ok(text_before))
                    .into_iter()
                    .chain(pairs.flatten())
                    .collect::<Result<Vec<_>, _>>()?;
            }
            _ => return Err(Error::UnexpectedContentType(ident)),
        };

        if let MetaTypeVariant::ComplexType(ci) =
            &mut self.types.items.get_mut(&ident).unwrap().variant
        {
            ci.is_mixed = false;
        }

        Ok(self)
    }

    pub fn simplify_mixed_types(mut self) -> Self {
        tracing::debug!("simplify_mixed_types");

        let idents = self
            .types
            .items
            .iter()
            .filter_map(|(ident, type_)| {
                if let MetaTypeVariant::ComplexType(ci) = &type_.variant {
                    if ci.is_mixed {
                        match ci.content_meta(&self.types).map(|x| &x.variant) {
                            Some(MetaTypeVariant::Sequence(x))
                                if x.elements.iter().all(|e| {
                                    e.min_occurs <= 1 && e.max_occurs == MaxOccurs::Bounded(1)
                                }) =>
                            {
                                return Some(ident)
                            }
                            Some(MetaTypeVariant::Choice(_))
                                if ci.max_occurs == MaxOccurs::Unbounded =>
                            {
                                return Some(ident)
                            }
                            _ => (),
                        }
                    }
                }

                None
            })
            .cloned()
            .collect::<Vec<_>>();

        for ident in idents {
            self = self.simplify_mixed_type(ident).unwrap();
        }

        self
    }
}
