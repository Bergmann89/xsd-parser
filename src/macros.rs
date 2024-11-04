#![allow(unused_macro_rules)]

macro_rules! unimplemented {
    () => {{
        tracing::error!("unimplemented");
        core::unimplemented!()
    }};
    ($( $tt:tt )*) => {{
        tracing::error!("unimplemented: {}", format!($( $tt )*));
        core::unimplemented!($( $tt )*)
    }};
}

macro_rules! unreachable {
    () => {{
        tracing::error!("unreachable");
        core::unreachable!()
    }};
    ($( $tt:tt )*) => {{
        tracing::error!("unreachable: {}", format!($( $tt )*));
        core::unreachable!($( $tt )*)
    }};
}

macro_rules! assert_eq {
    ($a:expr, $b:expr $(,)?) => {{
        if $a != $b {
            tracing::error!(a = stringify!($a), b = stringify!($b), "assertion failed");
            core::assert_eq!($a, $b);
        }
    }};
    ($a:expr, $b:expr, $($arg:tt)+) => {{
        if $a != $b {
            tracing::error!(a = stringify!($a), b = stringify!($b), message = format!($($arg)+));
            core::assert_eq!($a, $b, $($arg)+);
        }
    }};
}

pub(crate) use assert_eq;
pub(crate) use unimplemented;
pub(crate) use unreachable;
