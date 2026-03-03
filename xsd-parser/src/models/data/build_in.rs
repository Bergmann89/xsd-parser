use std::borrow::Cow;

use crate::models::meta::BuildInMeta;

/// Contains additional information for the rendering process of a
/// [`MetaTypeVariant::BuildIn`](crate::models::meta::MetaTypeVariant::BuildIn)
/// type.
#[derive(Debug)]
pub struct BuildInData<'types> {
    /// Reference to the original type information.
    pub meta: Cow<'types, BuildInMeta>,
}
