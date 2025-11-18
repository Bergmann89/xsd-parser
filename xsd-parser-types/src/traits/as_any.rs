/// Trait that is used to get the [`Any`](core::any::Any) trait for a specific type.
pub trait AsAny: core::any::Any {
    /// Convert the boxed object into a boxed any.
    fn into_any(self: Box<Self>) -> Box<dyn core::any::Any>;

    /// Get a reference to the current object as [`Any`](core::any::Any).
    fn as_any(&self) -> &dyn core::any::Any;

    /// Get a mutable reference to the current object as [`Any`](core::any::Any).
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any;
}

impl<X: 'static> AsAny for X {
    #[inline]
    fn into_any(self: Box<Self>) -> Box<dyn core::any::Any> {
        self
    }

    #[inline]
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }

    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}
