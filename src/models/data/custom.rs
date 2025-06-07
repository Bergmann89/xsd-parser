use crate::models::meta::CustomMeta;

/// Contains additional information for the rendering process
/// of a [`MetaTypeVariant::Custom`] type.
#[derive(Debug)]
pub struct CustomData<'types> {
    /// Reference to the original type information.
    pub meta: &'types CustomMeta,
}
