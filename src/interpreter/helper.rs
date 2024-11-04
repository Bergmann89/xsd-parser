use std::fmt::Display;

use crate::types::Name;

use super::state::State;

/* NameFallback */

pub(super) trait NameFallback<T>: Sized {
    type Output;

    fn or_fallback(self, fallback: T) -> Self::Output;
}

impl<X, T> NameFallback<T> for Option<X>
where
    Name: From<X>,
    T: NameInternals,
{
    type Output = T::Unwrap;

    fn or_fallback(self, fallback: T) -> Self::Output {
        T::unwrap(self.map(Name::from).or_else(|| fallback.into_name()))
    }
}

/* NameExtend */

pub(super) trait NameExtend: Sized {
    type Output;

    fn extend<I>(self, replace: bool, iter: I) -> Self::Output
    where
        I: IntoIterator,
        I::Item: Display;

    fn auto_extend(
        self,
        stop_at_group: bool,
        replace: bool,
        state: &mut State<'_>,
    ) -> Self::Output {
        self.extend(replace, state.last_named_type(stop_at_group))
    }
}

impl<X> NameExtend for X
where
    Name: From<X>,
{
    type Output = Name;

    fn extend<I>(self, replace: bool, iter: I) -> Self::Output
    where
        I: IntoIterator,
        I::Item: Display,
    {
        Name::from(self).extend(replace, iter)
    }
}

impl<X> NameExtend for Option<X>
where
    Name: From<X>,
{
    type Output = Option<Name>;

    fn extend<I>(self, replace: bool, iter: I) -> Self::Output
    where
        I: IntoIterator,
        I::Item: Display,
    {
        self.map(Name::from).map(|n| n.extend(replace, iter))
    }
}

/* NameUnwrap */

pub(super) trait NameUnwrap: Sized {
    fn unwrap_or_unnamed(self, state: &mut State<'_>) -> Name;
}

impl<T> NameUnwrap for Option<T>
where
    Name: From<T>,
{
    fn unwrap_or_unnamed(self, state: &mut State<'_>) -> Name {
        self.map_or_else(|| state.make_unnamed(), Name::from)
    }
}

/* NameInternals */

pub(super) trait NameInternals {
    type Unwrap;

    fn into_name(self) -> Option<Name>;
    fn unwrap(name: Option<Name>) -> Self::Unwrap;
}

impl NameInternals for Name {
    type Unwrap = Name;

    fn into_name(self) -> Option<Name> {
        Some(self)
    }

    fn unwrap(name: Option<Name>) -> Self::Unwrap {
        name.unwrap()
    }
}

impl NameInternals for &Name {
    type Unwrap = Name;

    fn into_name(self) -> Option<Name> {
        Some(self.clone())
    }

    fn unwrap(name: Option<Name>) -> Self::Unwrap {
        name.unwrap()
    }
}

impl<X> NameInternals for Option<X>
where
    Name: From<X>,
{
    type Unwrap = Option<Name>;

    fn into_name(self) -> Option<Name> {
        self.map(Name::from)
    }

    fn unwrap(name: Option<Name>) -> Self::Unwrap {
        name
    }
}

impl<F, R> NameInternals for F
where
    F: FnOnce() -> R,
    R: NameInternals,
{
    type Unwrap = R::Unwrap;

    fn into_name(self) -> Option<Name> {
        self().into_name()
    }

    fn unwrap(name: Option<Name>) -> Self::Unwrap {
        R::unwrap(name)
    }
}
