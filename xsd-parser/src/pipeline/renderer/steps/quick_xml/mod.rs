mod collect_namespaces;
mod deserialize;
mod serialize;

use crate::models::{
    data::{ComplexDataElement, Occurs, ReferenceData},
    meta::{ElementMetaVariant, ElementMode, MetaTypeVariant},
};

use super::super::context::Context;

pub use self::collect_namespaces::QuickXmlCollectNamespacesRenderStep;
pub use self::deserialize::QuickXmlDeserializeRenderStep;
pub use self::serialize::{NamespaceSerialization, QuickXmlSerializeRenderStep};

impl ComplexDataElement<'_> {
    #[inline]
    fn treat_as_any(&self) -> bool {
        self.meta().is_any()
    }

    #[inline]
    fn treat_as_text(&self) -> bool {
        self.meta().is_text()
    }

    #[inline]
    fn treat_as_group(&self) -> bool {
        !self.treat_as_any()
            && !self.treat_as_text()
            && matches!(
                &self.meta().variant,
                ElementMetaVariant::Type {
                    mode: ElementMode::Group,
                    ..
                }
            )
    }

    #[inline]
    fn treat_as_group_or_dynamic(&self) -> bool {
        self.treat_as_group() || self.target_is_dynamic
    }

    #[inline]
    fn treat_as_element(&self) -> bool {
        !self.treat_as_any()
            && !self.treat_as_text()
            && !self.treat_as_group()
            && !self.target_is_dynamic
    }
}

impl ReferenceData<'_> {
    /// Returns `true` if this reference wraps a single complex type, so the
    /// (de)serializer of the target has to be used instead of the bytes path.
    fn target_is_complex(&self, ctx: &Context<'_, '_>) -> bool {
        self.occurs == Occurs::Single
            && matches!(
                ctx.types
                    .meta
                    .types
                    .get_resolved_type(&self.meta.type_)
                    .map(|x| &x.variant),
                Some(
                    MetaTypeVariant::ComplexType(_)
                        | MetaTypeVariant::Dynamic(_)
                        | MetaTypeVariant::All(_)
                        | MetaTypeVariant::Choice(_)
                        | MetaTypeVariant::Sequence(_)
                )
            )
    }
}
