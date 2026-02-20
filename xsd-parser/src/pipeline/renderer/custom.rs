use std::fmt::{Debug, Formatter, Result as FmtResult};

use proc_macro2::TokenStream;

use super::Context;

/// Boxed version of [`ValueRenderer`].
pub type ValueRendererBox = Box<dyn ValueRenderer>;

impl Debug for ValueRendererBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("ValueRendererBox").finish()
    }
}

/// Trait that is responsible for rendering values.
pub trait ValueRenderer {
    /// Renders the value and return the resulting token stream if possible.
    ///
    /// The `as_const` parameter indicates whether the value should be rendered
    /// as a constant code or not.
    fn render(&self, ctx: &Context<'_, '_>) -> TokenStream;
}

impl<X> ValueRenderer for X
where
    X: Fn(&Context<'_, '_>) -> TokenStream,
{
    fn render(&self, ctx: &Context<'_, '_>) -> TokenStream {
        (*self)(ctx)
    }
}

impl ValueRenderer for TokenStream {
    fn render(&self, _ctx: &Context<'_, '_>) -> TokenStream {
        self.clone()
    }
}
