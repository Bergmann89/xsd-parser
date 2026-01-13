#![allow(unused_macros)]
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

macro_rules! assert {
    ($x:expr $(,)?) => {{
        if !$x {
            tracing::error!(x = stringify!($x), "assertion failed");
            core::assert!($x);
        }
    }};
    ($x:expr, $($arg:tt)+) => {{
        if !$x {
            tracing::error!(x = stringify!($x), message = format!($($arg)+));
            core::assert!($x, $b, $($arg)+);
        }
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

pub(crate) use assert;
pub(crate) use unreachable;
