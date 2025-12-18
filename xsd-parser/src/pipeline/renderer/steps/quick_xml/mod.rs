mod deserialize;
mod serialize;

use crate::models::{
    data::ComplexDataElement,
    meta::{ElementMetaVariant, ElementMode},
};

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
