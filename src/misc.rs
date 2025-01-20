use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;

use anyhow::Error as AnyError;
use thiserror::Error;

use crate::types::{ElementMode, Ident, Type, Types};
use crate::{GeneratorError, InterpreterError, ParserError};

/// Trait that adds namespace information to a type.
pub trait WithNamespace {
    /// The default namespace prefix for this type.
    fn prefix() -> &'static str;

    /// The namespace for this type.
    fn namespace() -> &'static str;
}

/// Error emitted by the [`generate`](crate::generate) function.
#[derive(Debug, Error)]
pub enum Error {
    /// IO Error.
    #[error("IO Error: {0}")]
    IoError(#[from] IoError),

    /// Parser error.
    #[error("Parser error: {0}")]
    ParserError(ParserError<AnyError>),

    /// Interpreter error.
    #[error("Interpreter error: {0}")]
    InterpreterError(#[from] InterpreterError),

    /// Generator error.
    #[error("Generator error: {0}")]
    GeneratorError(#[from] GeneratorError),
}

impl<E> From<ParserError<E>> for Error
where
    AnyError: From<E>,
{
    fn from(value: ParserError<E>) -> Self {
        match value {
            ParserError::IoError(err) => Self::ParserError(ParserError::IoError(err)),
            ParserError::XmlError(err) => Self::ParserError(ParserError::XmlError(err)),
            ParserError::UrlParseError(err) => Self::ParserError(ParserError::UrlParseError(err)),
            ParserError::UnableToResolve(url) => {
                Self::ParserError(ParserError::UnableToResolve(url))
            }
            ParserError::Resolver(err) => {
                Self::ParserError(ParserError::Resolver(AnyError::from(err)))
            }
            ParserError::InvalidFilePath(path) => {
                Self::ParserError(ParserError::InvalidFilePath(path))
            }
            ParserError::MissingTargetNamespace => {
                Self::ParserError(ParserError::MissingTargetNamespace)
            }
            ParserError::MissingTargetNamespacePrefix => {
                Self::ParserError(ParserError::MissingTargetNamespacePrefix)
            }
        }
    }
}

/// Helper type that implements [`Debug`] and [`Display`] for a byte slice.
pub struct RawByteStr(Cow<'static, [u8]>);

impl RawByteStr {
    /// Create a new [`RawByteStr`] instance from the passed byte slice.
    #[must_use]
    pub fn from_slice(value: &[u8]) -> Self {
        Self(Cow::Owned(value.to_owned()))
    }
}

impl From<Vec<u8>> for RawByteStr {
    fn from(value: Vec<u8>) -> Self {
        Self(Cow::Owned(value))
    }
}

impl From<&'static [u8]> for RawByteStr {
    fn from(value: &'static [u8]) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl From<&'static str> for RawByteStr {
    fn from(value: &'static str) -> Self {
        Self(Cow::Borrowed(value.as_bytes()))
    }
}

impl Debug for RawByteStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        format_utf8_slice(&self.0, f)
    }
}

impl Display for RawByteStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        format_utf8_slice(&self.0, f)
    }
}

/* TypesPrinter */

pub(crate) struct TypesPrinter<'a> {
    types: &'a Types,
}

#[derive(Default)]
struct State {
    level: usize,
    visit: HashSet<Ident>,
}

impl<'a> TypesPrinter<'a> {
    pub(crate) fn new(types: &'a Types) -> Self {
        Self { types }
    }

    fn print_all(&self, f: &mut Formatter<'_>, s: &mut State) -> FmtResult {
        for (ident, ty) in &**self.types {
            self.print_type(f, s, ident, ty)?;
        }

        Ok(())
    }

    fn resolve_complex_type(
        &self,
        f: &mut Formatter<'_>,
        s: &mut State,
        ident: &Ident,
    ) -> FmtResult {
        if let Some(x) = self.types.get(ident) {
            self.print_type(f, s, ident, x)
        } else {
            writeln!(f, "NOT FOUND")?;

            Ok(())
        }
    }

    #[allow(clippy::too_many_lines)]
    fn print_type(
        &self,
        f: &mut Formatter<'_>,
        s: &mut State,
        ident: &Ident,
        ty: &Type,
    ) -> FmtResult {
        macro_rules! indent {
            ($( $tt:tt )*) => {{
                write!(f, "{0:1$}", "", 4 * s.level)?;
                write!(f, $( $tt )*)?;
            }};
        }
        macro_rules! indentln {
            ($( $tt:tt )*) => {{
                write!(f, "{0:1$}", "", 4 * s.level)?;
                writeln!(f, $( $tt )*)?;
            }};
        }

        if !s.visit.insert(ident.clone()) {
            writeln!(f, "LOOP DETECTED ({})", ident.name)?;

            return Ok(());
        }

        match ty {
            Type::BuildIn(x) => {
                writeln!(f, "{}: BuildIn", ident)?;

                s.level += 1;

                indentln!("type={x:?}");

                s.level -= 1;
            }
            Type::Union(x) => {
                writeln!(f, "{}: Union", ident)?;

                s.level += 1;

                indentln!("base={}", x.base);
                indentln!("types");

                s.level += 1;

                for ty in &*x.types {
                    indentln!("{}", &ty.type_);
                }

                s.level -= 2;
            }
            Type::Reference(x) => {
                writeln!(f, "{}: Reference", ident)?;

                s.level += 1;

                indentln!("min={}", x.min_occurs);
                indentln!("max={:?}", x.max_occurs);
                indentln!("type={}", x.type_);

                s.level -= 1;
            }
            Type::Abstract(x) => {
                writeln!(f, "{}: Abstract", ident)?;

                s.level += 1;

                indentln!("types");

                s.level += 1;

                for ty in &*x.derived_types {
                    indentln!("{}", ty);
                }

                s.level -= 2;
            }
            Type::Enumeration(x) => {
                writeln!(f, "{}: Enumeration", ident)?;

                s.level += 1;

                indentln!("base={}", x.base);
                indentln!("variants");

                s.level += 1;

                for var in &*x.variants {
                    indentln!("{}={:?}", var.ident.name, var.use_);
                }

                s.level -= 2;
            }
            Type::All(x) | Type::Choice(x) | Type::Sequence(x) => {
                match ty {
                    Type::All(_) => writeln!(f, "{}: All", ident)?,
                    Type::Choice(_) => writeln!(f, "{}: Choice", ident)?,
                    Type::Sequence(_) => writeln!(f, "{}: Sequence", ident)?,
                    _ => (),
                }

                s.level += 1;

                if let Some(x) = &x.any {
                    indentln!("any");

                    s.level += 1;

                    if let Some(x) = &x.min_occurs {
                        indentln!("min_occurs={x:?}");
                    }
                    if let Some(x) = &x.max_occurs {
                        indentln!("max_occurs={x:?}");
                    }
                    if let Some(x) = &x.namespace {
                        indentln!("namespace={x:?}");
                    }
                    if let Some(x) = &x.not_namespace {
                        indentln!("not_namespace={x:?}");
                    }
                    if let Some(x) = &x.not_q_name {
                        indentln!("not_q_name={x:?}");
                    }
                    if let Some(x) = &x.process_contents {
                        indentln!("process_contents={x:?}");
                    }

                    s.level -= 1;
                }

                for x in &*x.elements {
                    indentln!("element");

                    s.level += 1;

                    indentln!("name={}", x.ident.name);
                    indentln!("min_occurs={}", x.min_occurs);
                    indentln!("max_occurs={:?}", x.max_occurs);
                    indentln!("element_type={:?}", x.element_mode);

                    if x.element_mode == ElementMode::Element {
                        indentln!("type={}", x.type_);
                    } else {
                        indent!("type=");
                        self.resolve_complex_type(f, s, &x.type_)?;
                    }

                    s.level -= 1;
                }

                s.level -= 1;
            }
            Type::ComplexType(x) => {
                writeln!(f, "{}: ComplexType", ident)?;

                s.level += 1;

                indentln!("base={}", x.base);
                indentln!("min_occurs={}", x.min_occurs);
                indentln!("max_occurs={:?}", x.max_occurs);
                indentln!("is_abstract={}", x.is_abstract);

                if let Some(x) = &x.any_attribute {
                    indentln!("any_attribute");

                    s.level += 1;

                    if let Some(x) = &x.namespace {
                        indentln!("namespace={x:?}");
                    }
                    if let Some(x) = &x.not_namespace {
                        indentln!("not_namespace={x:?}");
                    }
                    if let Some(x) = &x.not_q_name {
                        indentln!("not_q_name={x:?}");
                    }
                    if let Some(x) = &x.process_contents {
                        indentln!("process_contents={x:?}");
                    }

                    s.level -= 1;
                }

                for x in &*x.attributes {
                    indentln!("attribute");

                    s.level += 1;

                    indentln!("name={}", x.ident.name);
                    indentln!("type={}", x.type_);
                    indentln!("use={:?}", x.use_);
                    indentln!("default={:?}", x.default);

                    s.level -= 1;
                }

                if let Some(content) = &x.content {
                    indentln!("content");

                    s.level += 1;

                    indent!("type=");
                    self.resolve_complex_type(f, s, content)?;

                    s.level -= 1;
                }

                s.level -= 1;
            }
        }

        s.visit.remove(ident);

        Ok(())
    }
}

impl Display for TypesPrinter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut s = State::default();

        self.print_all(f, &mut s)
    }
}

/* Helper */

pub(crate) fn format_utf8_slice(bytes: &[u8], f: &mut Formatter<'_>) -> FmtResult {
    for byte in bytes {
        if byte.is_ascii_control() {
            write!(f, r"\x{byte:02x}")?;
        } else {
            write!(f, "{}", *byte as char)?;
        }
    }

    Ok(())
}
