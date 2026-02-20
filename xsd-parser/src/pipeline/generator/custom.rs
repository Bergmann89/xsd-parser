use std::fmt::{Debug, Formatter, Result as FmtResult};

use crate::pipeline::renderer::ValueRenderer;

use super::{Context, Error};

/// Boxed version of [`ValueGenerator`].
pub type ValueGeneratorBox = Box<dyn ValueGenerator>;

impl Debug for ValueGeneratorBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("ValueGeneratorBox").finish()
    }
}

/// Trait that converts the value of a element specified in the XML schema to
/// actual code.
///
/// You can add a custom default value implementation to your custom type using
/// [`CustomMeta::with_default`](crate::models::meta::CustomMeta::with_default).
pub trait ValueGenerator: Send + Sync + 'static {
    /// Try to convert the passed string `value` that contains the default value from
    /// the XML schema to actual default code. If the value could not be converted
    /// to code `None` is returned.
    fn exec(
        &self,
        ctx: &Context<'_, '_>,
        value: &str,
        mode: ValueGeneratorMode,
    ) -> Result<Box<dyn ValueRenderer>, Error>;

    /// Clone this instance and return it as a box.
    fn clone(&self) -> Box<dyn ValueGenerator>;
}

/// Determines how the value of a element specified in the XML schema should be converted to
/// code.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ValueGeneratorMode {
    /// Render the value as a byte string literal.
    Literal,

    /// Render the value as a constant/compile time constructed value.
    Constant,

    /// Render the value as normal runtime constructed value.
    Value,
}

impl<X> ValueGenerator for X
where
    X: Fn(&Context<'_, '_>, &str, ValueGeneratorMode) -> Result<Box<dyn ValueRenderer>, Error>
        + Clone
        + Send
        + Sync
        + 'static,
{
    fn exec(
        &self,
        ctx: &Context<'_, '_>,
        value: &str,
        mode: ValueGeneratorMode,
    ) -> Result<Box<dyn ValueRenderer>, Error> {
        (*self)(ctx, value, mode)
    }

    fn clone(&self) -> Box<dyn ValueGenerator> {
        Box::new(self.clone())
    }
}
