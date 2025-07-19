//! This module implements the [`XPath`] type according to the
//! [official specification](https://www.w3.org/TR/xpath-31/#prod-xpath31-XPath).

#![allow(missing_docs)]

use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Deref, DerefMut};
use std::str::{from_utf8, FromStr};

use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, tag_no_case, take, take_until, take_while, take_while1},
    character::complete::{digit0, digit1, multispace0, one_of},
    combinator::{opt, recognize, verify},
    error::{Error, ParseError},
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, preceded, terminated},
    AsChar, Err, IResult, Input, Parser,
};
use thiserror::Error;

use crate::models::format_utf8_slice;

pub use nom::error::ErrorKind;

macro_rules! impl_deref {
    ($type:ident $( < $lt:lifetime > )?, $target:ty) => {
        impl $( < $lt > )? Deref for $type $( < $lt > )? {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

macro_rules! impl_deref_mut {
    ($type:ident $( < $lt:lifetime > )?, $target:ty) => {
        impl_deref!($type $( < $lt > )?, $target);

        impl$( < $lt > )? DerefMut for $type $( < $lt > )? {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

macro_rules! impl_from_str {
    ($ty:ident $( :: <$lt:lifetime> :: $convert:ident )?) => {
        impl FromStr for $ty $( <$lt> )? {
            type Err = FromStrError;

            fn from_str(s: &str) -> Result<Self, FromStrError> {
                match $ty::nom(s.as_bytes()) {
                    Ok((rest, value)) => {
                        if rest.is_empty() {
                            Ok(value $( .$convert() )?)
                        } else {
                            Err(FromStrError::Incomplete {
                                expr: s.into(),
                                rest: from_utf8(rest).ok().map(Into::into),
                            })
                        }
                    }
                    Err(Err::Incomplete(_)) => unreachable!(),
                    Err(Err::Error(error) | Err::Failure(error)) => Err(FromStrError::Error {
                        expr: s.into(),
                        code: error.code,
                        rest: from_utf8(error.input).ok().map(Into::into),
                    })
                }
            }
        }
    };
}

#[cfg(not(feature = "xpath-debug"))]
macro_rules! impl_nom_parser {
    (
        fn $ty:ident::$name:ident ( $input:ident: &[u8] ) -> IResult<&[u8], Self>
            $body:block
    ) => {
        impl $ty {
            /// Parses the type from the `input` bytes using [`nom`].
            ///
            /// # Errors
            /// Returns `nom::Err<nom::error::Error>` if parsing the type failed.
            pub fn nom($input: &[u8]) -> IResult<&[u8], Self>
                $body
        }

        impl_from_str!($ty);
    };
    (
        fn $ty:ident<'a>::$name:ident ( $input:ident: &'a [u8] ) -> IResult<&'a [u8], Self>
            $body:block
    ) => {
        impl<'a> $ty<'a> {
            /// Parses the type from the `input` bytes using [`nom`].
            ///
            /// # Errors
            /// Returns `nom::Err<nom::error::Error>` if parsing the type failed.
            pub fn nom($input: &'a [u8]) -> IResult<&'a [u8], Self>
                $body
        }

        impl_from_str!($ty::<'static>::into_owned);
    };

}

#[cfg(feature = "xpath-debug")]
macro_rules! impl_nom_parser {
    (
        fn $ty:ident::$name:ident ( $input:ident: &[u8] ) -> IResult<&[u8], Self>
            $body:block
    ) => {
        impl $ty {
            /// Parses the type from the `input` bytes using [`nom`].
            ///
            /// # Errors
            /// Returns `nom::Err<nom::error::Error>` if parsing the type failed.
            pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
                let ty = stringify!($ty);
                debug::enter(ty, input);
                debug::leave(Self::nom_(input))
            }

            fn nom_($input: &[u8]) -> IResult<&[u8], Self>
                $body
        }

        impl_from_str!($ty);
    };
    (
        fn $ty:ident<'a>::$name:ident ( $input:ident: &'a [u8] ) -> IResult<&'a [u8], Self>
            $body:block
    ) => {
        impl<'a> $ty<'a> {
            /// Parses the type from the `input` bytes using [`nom`].
            ///
            /// # Errors
            /// Returns `nom::Err<nom::error::Error>` if parsing the type failed.
            pub fn nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
                let ty = stringify!($ty);
                debug::enter(ty, input);
                debug::leave(Self::nom_(input))
            }

            fn nom_($input: &'a [u8]) -> IResult<&'a [u8], Self>
                $body
        }

        impl_from_str!($ty::<'static>::into_owned);
    };
}

/// Implementation of [`XPath`](https://www.w3.org/TR/xpath-31/#prod-xpath31-XPath).
///
/// ```
/// [1] XPath ::= Expr
/// ```
pub type XPath<'a> = Expr<'a>;

/// Implementation of [`ParamList`](https://www.w3.org/TR/xpath-31/#prod-xpath31-ParamList).
///
/// ```
/// [2] ParamList ::= Param ("," Param)*
/// ````
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParamList<'a>(pub Vec<Param<'a>>);

impl ParamList<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> ParamList<'static> {
        ParamList(self.0.into_iter().map(Param::into_owned).collect())
    }
}

impl_deref_mut!(ParamList<'a>, Vec<Param<'a>>);

impl_nom_parser! {
    fn ParamList<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        separated_list1(trim0(tag(",")), Param::nom)
            .map(Self)
            .parse(rest)
    }
}

impl Display for ParamList<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for (index, param) in self.0.iter().enumerate() {
            if index > 0 {
                write!(f, ", {param}")?;
            } else {
                write!(f, "{param}")?;
            }
        }

        Ok(())
    }
}

/// Implementation of [`ParamList`](https://www.w3.org/TR/xpath-31/#prod-xpath31-Param).
///
/// ```
/// [3] Param ::= "$" EQName TypeDeclaration?
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Param<'a> {
    pub name: EQName<'a>,
    pub type_: Option<TypeDeclaration<'a>>,
}

impl Param<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> Param<'static> {
        Param {
            name: self.name.into_owned(),
            type_: self.type_.map(TypeDeclaration::into_owned),
        }
    }
}

impl_nom_parser! {
    fn Param<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        let (rest, (name, type_)) =
            preceded(tag("$"), (EQName::nom, opt(TypeDeclaration::nom))).parse(rest)?;

        Ok((rest, Self { name, type_ }))
    }
}

impl Display for Param<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { name, type_ } = self;

        write!(f, "${name}")?;
        if let Some(type_) = type_ {
            write!(f, "{type_}")?;
        }

        Ok(())
    }
}

/// Implementation of [`FunctionBody`](https://www.w3.org/TR/xpath-31/#prod-xpath31-FunctionBody).
///
/// ```
/// [4] FunctionBody ::= EnclosedExpr
/// ```
pub type FunctionBody<'a> = EnclosedExpr<'a>;

/// Implementation of [`EnclosedExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-EnclosedExpr).
///
/// ```
/// [5] EnclosedExpr ::= "{" Expr? "}"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EnclosedExpr<'a>(pub Option<Expr<'a>>);

impl EnclosedExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> EnclosedExpr<'static> {
        EnclosedExpr(self.0.map(Expr::into_owned))
    }
}

impl_deref_mut!(EnclosedExpr<'a>, Option<Expr<'a>>);

impl_nom_parser! {
    fn EnclosedExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        delimited(trim0(tag("{")), opt(Expr::nom), trim0(tag("}")))
            .map(Self)
            .parse(rest)
    }
}

impl Display for EnclosedExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.0 {
            Some(expr) => write!(f, "{{{expr}}}"),
            None => write!(f, "{{}}"),
        }
    }
}

/// Implementation of [`Expr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-Expr).
///
/// ```
/// [6] Expr ::= ExprSingle ("," ExprSingle)*
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Expr<'a>(pub Vec<ExprSingle<'a>>);

impl Expr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> Expr<'static> {
        Expr(self.0.into_iter().map(ExprSingle::into_owned).collect())
    }
}

impl_deref_mut!(Expr<'a>, Vec<ExprSingle<'a>>);

impl_nom_parser! {
    fn Expr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        separated_list1(trim0(tag(",")), ExprSingle::nom)
            .map(Self)
            .parse(rest)
    }
}

impl Display for Expr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for (index, expr) in self.0.iter().enumerate() {
            if index > 0 {
                write!(f, ", {expr}")?;
            } else {
                write!(f, "{expr}")?;
            }
        }
        Ok(())
    }
}

/// Implementation of [`ExprSingle`](https://www.w3.org/TR/xpath-31/#prod-xpath31-ExprSingle).
///
/// ```
/// [7] ExprSingle ::= ForExpr | LetExpr | QuantifiedExpr | IfExpr | OrExpr
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExprSingle<'a> {
    If(Box<IfExpr<'a>>),
    Or(Box<OrExpr<'a>>),
    For(Box<ForExpr<'a>>),
    Let(Box<LetExpr<'a>>),
    Quantified(Box<QuantifiedExpr<'a>>),
}

impl ExprSingle<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> ExprSingle<'static> {
        match self {
            Self::For(x) => ExprSingle::For(Box::new(x.into_owned())),
            Self::Let(x) => ExprSingle::Let(Box::new(x.into_owned())),
            Self::Quantified(x) => ExprSingle::Quantified(Box::new(x.into_owned())),
            Self::If(x) => ExprSingle::If(Box::new(x.into_owned())),
            Self::Or(x) => ExprSingle::Or(Box::new(x.into_owned())),
        }
    }
}

impl_nom_parser! {
    fn ExprSingle<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {

        #[inline]
        fn is_for_expr(input: &[u8]) -> bool {
            tag::<_, _, Error<&[u8]>>("for").parse(input).is_ok()
        }

        #[inline]
        fn is_let_expr(input: &[u8]) -> bool {
            tag::<_, _, Error<&[u8]>>("let").parse(input).is_ok()
        }

        #[inline]
        fn is_quantified_expr(input: &[u8]) -> bool {
            alt((tag::<_, _, Error<&[u8]>>("some"), tag::<_, _, Error<&[u8]>>("every"))).parse(input).is_ok()
        }

        #[inline]
        fn is_if_expr(input: &[u8]) -> bool {
            tag::<_, _, Error<&[u8]>>("if").parse(input).is_ok()
        }

        let rest = input.trim_ascii();

        if is_for_expr(rest) {
            ForExpr::nom.map(Box::new).map(Self::For).parse(rest)
        } else if is_let_expr(rest) {
            LetExpr::nom.map(Box::new).map(Self::Let).parse(rest)
        } else if is_quantified_expr(rest) {
            QuantifiedExpr::nom.map(Box::new).map(Self::Quantified).parse(rest)
        } else if is_if_expr(rest) {
            IfExpr::nom.map(Box::new).map(Self::If).parse(rest)
        } else {
            OrExpr::nom.map(Box::new).map(Self::Or).parse(rest)
        }
    }
}

impl Display for ExprSingle<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::For(expr) => write!(f, "{expr}"),
            Self::Let(expr) => write!(f, "{expr}"),
            Self::Quantified(expr) => write!(f, "{expr}"),
            Self::If(expr) => write!(f, "{expr}"),
            Self::Or(expr) => write!(f, "{expr}"),
        }
    }
}

/// Implementation of [`ForExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-ForExpr).
///
/// ```
/// [8] ForExpr ::= SimpleForClause "return" ExprSingle
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ForExpr<'a> {
    pub clause: SimpleForClause<'a>,
    pub return_expr: ExprSingle<'a>,
}

impl ForExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> ForExpr<'static> {
        ForExpr {
            clause: self.clause.into_owned(),
            return_expr: self.return_expr.into_owned(),
        }
    }
}

impl_nom_parser! {
    fn ForExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        let (rest, (clause, _, return_expr)) = (
            SimpleForClause::nom,
            trim0(tag_no_case("return")),
            ExprSingle::nom,
        )
            .parse(rest)?;

        Ok((
            rest,
            Self {
                clause,
                return_expr,
            },
        ))
    }
}

impl Display for ForExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} return {}", self.clause, self.return_expr)
    }
}

/// Implementation of [`SimpleForClause`](https://www.w3.org/TR/xpath-31/#prod-xpath31-SimpleForClause).
///
/// ```
/// [9] SimpleForClause ::= "for" SimpleForBinding ("," SimpleForBinding)*
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SimpleForClause<'a>(pub Vec<SimpleForBinding<'a>>);

impl SimpleForClause<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> SimpleForClause<'static> {
        SimpleForClause(
            self.0
                .into_iter()
                .map(SimpleForBinding::into_owned)
                .collect(),
        )
    }
}

impl_deref_mut!(SimpleForClause<'a>, Vec<SimpleForBinding<'a>>);

impl_nom_parser! {
    fn SimpleForClause<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        preceded(
            trim0(tag_no_case("for")),
            separated_list1(trim0(tag(",")), SimpleForBinding::nom),
        )
        .map(Self)
        .parse(rest)
    }
}

impl Display for SimpleForClause<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "for ")?;

        for (i, binding) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", {binding}")?;
            } else {
                write!(f, "{binding}")?;
            }
        }

        Ok(())
    }
}

/// Implementation of [`SimpleForBinding`](https://www.w3.org/TR/xpath-31/#prod-xpath31-SimpleForBinding).
///
/// ```
/// [10] SimpleForBinding ::= "$" VarName "in" ExprSingle
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SimpleForBinding<'a> {
    pub var: VarName<'a>,
    pub in_expr: ExprSingle<'a>,
}

impl SimpleForBinding<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> SimpleForBinding<'static> {
        SimpleForBinding {
            var: self.var.into_owned(),
            in_expr: self.in_expr.into_owned(),
        }
    }
}

impl_nom_parser! {
    fn SimpleForBinding<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        let (rest, (_, var, _, in_expr)) = (
            tag("$"),
            VarName::nom,
            trim0(tag_no_case("in")),
            ExprSingle::nom,
        )
            .parse(rest)?;

        Ok((rest, Self { var, in_expr }))
    }
}

impl Display for SimpleForBinding<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "${} in {}", self.var, self.in_expr)
    }
}

/// Implementation of [`LetExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-LetExpr).
///
/// ```
/// [11] LetExpr ::= SimpleLetClause "return" ExprSingle
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LetExpr<'a> {
    pub clause: SimpleLetClause<'a>,
    pub return_expr: ExprSingle<'a>,
}

impl LetExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> LetExpr<'static> {
        LetExpr {
            clause: self.clause.into_owned(),
            return_expr: self.return_expr.into_owned(),
        }
    }
}

impl_nom_parser! {
    fn LetExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        let (rest, (clause, _, return_expr)) = (
            SimpleLetClause::nom,
            trim0(tag_no_case("return")),
            ExprSingle::nom,
        )
            .parse(rest)?;

        Ok((
            rest,
            Self {
                clause,
                return_expr,
            },
        ))
    }
}

impl Display for LetExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} return {}", self.clause, self.return_expr)
    }
}

/// Implementation of [`SimpleLetClause`](https://www.w3.org/TR/xpath-31/#prod-xpath31-SimpleLetClause).
///
/// ```
/// [12] SimpleLetClause ::= "let" SimpleLetBinding ("," SimpleLetBinding)*
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SimpleLetClause<'a>(pub Vec<SimpleLetBinding<'a>>);

impl SimpleLetClause<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> SimpleLetClause<'static> {
        SimpleLetClause(
            self.0
                .into_iter()
                .map(SimpleLetBinding::into_owned)
                .collect(),
        )
    }
}

impl_deref_mut!(SimpleLetClause<'a>, Vec<SimpleLetBinding<'a>>);

impl_nom_parser! {
    fn SimpleLetClause<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        preceded(
            trim0(tag_no_case("let")),
            separated_list1(trim0(tag(",")), SimpleLetBinding::nom),
        )
        .map(Self)
        .parse(rest)
    }
}

impl Display for SimpleLetClause<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "let ")?;

        for (i, binding) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", {binding}")?;
            } else {
                write!(f, "{binding}")?;
            }
        }

        Ok(())
    }
}

/// Implementation of [`SimpleLetBinding`](https://www.w3.org/TR/xpath-31/#prod-xpath31-SimpleLetBinding).
///
/// ```
/// [13] SimpleLetBinding ::= "$" VarName ":=" ExprSingle
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SimpleLetBinding<'a> {
    pub var: VarName<'a>,
    pub value: ExprSingle<'a>,
}

impl SimpleLetBinding<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> SimpleLetBinding<'static> {
        SimpleLetBinding {
            var: self.var.into_owned(),
            value: self.value.into_owned(),
        }
    }
}

impl_nom_parser! {
    fn SimpleLetBinding<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        let (rest, (_, var, _, value)) =
            (tag("$"), VarName::nom, trim0(tag(":=")), ExprSingle::nom).parse(rest)?;

        Ok((rest, Self { var, value }))
    }
}

impl Display for SimpleLetBinding<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "${} := {}", self.var, self.value)
    }
}

/// Implementation of [`QuantifiedExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-QuantifiedExpr).
///
/// ```
/// [14] QuantifiedExpr ::= ("some" | "every") "$" VarName "in" ExprSingle ("," "$" VarName "in" ExprSingle)* "satisfies" ExprSingle
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuantifiedExpr<'a> {
    pub quantifier: Quantifier,
    pub bindings: Vec<QuantifiedBinding<'a>>,
    pub satisfies: ExprSingle<'a>,
}

/// Helper type used in [`QuantifiedExpr`].
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Quantifier {
    Some,
    Every,
}

/// Helper type used in [`QuantifiedExpr`].
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuantifiedBinding<'a> {
    pub var: VarName<'a>,
    pub in_expr: ExprSingle<'a>,
}

impl QuantifiedExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> QuantifiedExpr<'static> {
        QuantifiedExpr {
            quantifier: self.quantifier,
            bindings: self
                .bindings
                .into_iter()
                .map(QuantifiedBinding::into_owned)
                .collect(),
            satisfies: self.satisfies.into_owned(),
        }
    }
}

impl_nom_parser! {
    fn QuantifiedExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        let (rest, (quantifier, bindings, _, satisfies)) = (
            alt((
                trim0(tag_no_case("some")).map(|_| Quantifier::Some),
                trim0(tag_no_case("every")).map(|_| Quantifier::Every),
            )),
            separated_list1(trim0(tag(",")), QuantifiedBinding::nom),
            trim0(tag_no_case("satisfies")),
            ExprSingle::nom,
        )
            .parse(rest)?;

        Ok((
            rest,
            Self {
                quantifier,
                bindings,
                satisfies,
            },
        ))
    }
}

impl Display for QuantifiedExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let keyword = match self.quantifier {
            Quantifier::Some => "some",
            Quantifier::Every => "every",
        };

        write!(f, "{keyword} ")?;
        for (i, binding) in self.bindings.iter().enumerate() {
            if i > 0 {
                write!(f, ", {binding}")?;
            } else {
                write!(f, "{binding}")?;
            }
        }
        write!(f, " satisfies {}", self.satisfies)
    }
}

impl QuantifiedBinding<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> QuantifiedBinding<'static> {
        QuantifiedBinding {
            var: self.var.into_owned(),
            in_expr: self.in_expr.into_owned(),
        }
    }
}

impl_nom_parser! {
    fn QuantifiedBinding<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        let (rest, (_, var, _, in_expr)) = (
            tag("$"),
            VarName::nom,
            trim0(tag_no_case("in")),
            ExprSingle::nom,
        )
            .parse(rest)?;

        Ok((rest, Self { var, in_expr }))
    }
}

impl Display for QuantifiedBinding<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "${} in {}", self.var, self.in_expr)
    }
}

/// Implementation of [`IfExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-IfExpr).
///
/// ```
/// [15] IfExpr ::= "if" "(" Expr ")" "then" ExprSingle "else" ExprSingle
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IfExpr<'a> {
    pub condition: Expr<'a>,
    pub then_: ExprSingle<'a>,
    pub else_: ExprSingle<'a>,
}

impl IfExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> IfExpr<'static> {
        IfExpr {
            condition: self.condition.into_owned(),
            then_: self.then_.into_owned(),
            else_: self.else_.into_owned(),
        }
    }
}

impl_nom_parser! {
    fn IfExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        let (rest, (_, _, condition, _, _, then_, _, else_)) = (
            trim0(tag_no_case("if")),
            trim0(tag("(")),
            Expr::nom,
            trim0(tag(")")),
            trim0(tag_no_case("then")),
            ExprSingle::nom,
            trim0(tag_no_case("else")),
            ExprSingle::nom,
        )
            .parse(rest)?;

        Ok((
            rest,
            Self {
                condition,
                then_,
                else_,
            },
        ))
    }
}

impl Display for IfExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "if ({}) then {} else {}",
            self.condition, self.then_, self.else_
        )
    }
}

/// Implementation of [`OrExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-OrExpr).
///
/// ```
/// [16] OrExpr ::= AndExpr ( "or" AndExpr )*
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OrExpr<'a>(pub Vec<AndExpr<'a>>);

impl OrExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> OrExpr<'static> {
        OrExpr(self.0.into_iter().map(AndExpr::into_owned).collect())
    }
}

impl_deref_mut!(OrExpr<'a>, Vec<AndExpr<'a>>);

impl_nom_parser! {
    fn OrExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        separated_list1(trim0(tag("or")), AndExpr::nom).map(Self).parse(rest)
    }
}

impl Display for OrExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for (i, expr) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " or {expr}")?;
            } else {
                expr.fmt(f)?;
            }
        }

        Ok(())
    }
}

/// Implementation of [`AndExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-AndExpr).
///
/// ```
/// [17] AndExpr ::= ComparisonExpr ( "and" ComparisonExpr )*
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AndExpr<'a>(pub Vec<ComparisonExpr<'a>>);

impl AndExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> AndExpr<'static> {
        AndExpr(self.0.into_iter().map(ComparisonExpr::into_owned).collect())
    }
}

impl_deref_mut!(AndExpr<'a>, Vec<ComparisonExpr<'a>>);

impl_nom_parser! {
    fn AndExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        separated_list1(trim0(tag("and")), ComparisonExpr::nom).map(Self).parse(rest)
    }
}

impl Display for AndExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for (i, expr) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " and {expr}")?;
            } else {
                expr.fmt(f)?;
            }
        }

        Ok(())
    }
}

/// Implementation of [`ComparisonExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-ComparisonExpr).
///
/// ```
/// [18] ComparisonExpr ::= StringConcatExpr ( ( ValueComp | GeneralComp | NodeComp ) StringConcatExpr )?
/// [32] GeneralComp    ::= "=" | "!=" | "<" | "<=" | ">" | ">="
/// [33] ValueComp      ::= "eq" | "ne" | "lt" | "le" | "gt" | "ge"
/// [34] NodeComp       ::= "is" | "<<" | ">>"
/// ````
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ComparisonExpr<'a> {
    pub head: StringConcatExpr<'a>,
    pub tail: Vec<(ComparisonExprOp, StringConcatExpr<'a>)>,
}

impl ComparisonExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> ComparisonExpr<'static> {
        ComparisonExpr {
            head: self.head.into_owned(),
            tail: self
                .tail
                .into_iter()
                .map(|(op, expr)| (op, expr.into_owned()))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ComparisonExprOp {
    Equal,
    NotEqual,
    LowerThan,
    LowerEqual,
    GreaterThan,
    GreaterEqual,
    Is,
    Precedes,
    Follows,
}

impl_nom_parser! {
    fn ComparisonExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        let (rest, head) = StringConcatExpr::nom.parse(rest)?;
        let (rest, tail) = many0((
            ComparisonExprOp::nom,
            StringConcatExpr::nom,
        )).parse(rest)?;

        Ok((rest, Self {
            head,
            tail,
        }))
    }
}

impl_nom_parser! {
    fn ComparisonExprOp::nom(input: &[u8]) -> IResult<&[u8], Self> {
        let rest = input.trim_ascii();

        alt((
            tag("!=").map(|_| Self::NotEqual),
            tag("<=").map(|_| Self::LowerEqual),
            tag(">=").map(|_| Self::GreaterEqual),
            tag("=").map(|_| Self::Equal),
            tag("<").map(|_| Self::LowerEqual),
            tag(">").map(|_| Self::GreaterThan),
            tag_no_case("eq").map(|_| Self::Equal),
            tag_no_case("ne").map(|_| Self::NotEqual),
            tag_no_case("lt").map(|_| Self::LowerThan),
            tag_no_case("le").map(|_| Self::LowerEqual),
            tag_no_case("gt").map(|_| Self::GreaterThan),
            tag_no_case("ge").map(|_| Self::GreaterEqual),
            tag_no_case("is").map(|_| Self::Is),
            tag("<<").map(|_| Self::Precedes),
            tag(">>").map(|_| Self::Follows),
        )).parse(rest)
    }
}

impl Display for ComparisonExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { head, tail } = self;

        head.fmt(f)?;

        for (op, expr) in tail {
            write!(f, " {op} {expr}")?;
        }

        Ok(())
    }
}

impl Display for ComparisonExprOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Equal => write!(f, "eq"),
            Self::NotEqual => write!(f, "ne"),
            Self::LowerThan => write!(f, "lt"),
            Self::LowerEqual => write!(f, "le"),
            Self::GreaterThan => write!(f, "gt"),
            Self::GreaterEqual => write!(f, "ge"),
            Self::Is => write!(f, "is"),
            Self::Precedes => write!(f, "<<"),
            Self::Follows => write!(f, ">>"),
        }
    }
}

/// Implementation of [`StringConcatExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-StringConcatExpr).
///
/// ```
/// [19] StringConcatExpr ::= RangeExpr ( "||" RangeExpr )*
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StringConcatExpr<'a>(pub Vec<RangeExpr<'a>>);

impl StringConcatExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> StringConcatExpr<'static> {
        StringConcatExpr(self.0.into_iter().map(RangeExpr::into_owned).collect())
    }
}

impl_deref_mut!(StringConcatExpr<'a>, Vec<RangeExpr<'a>>);

impl_nom_parser! {
    fn StringConcatExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        separated_list1(trim0(tag("||")), RangeExpr::nom).map(Self).parse(rest)
    }
}

impl Display for StringConcatExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for (i, expr) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " || {expr}")?;
            } else {
                expr.fmt(f)?;
            }
        }

        Ok(())
    }
}

/// Implementation of [`RangeExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-RangeExpr).
///
/// ```
/// [20] RangeExpr ::= AdditiveExpr ( "to" AdditiveExpr )?
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RangeExpr<'a> {
    Single(AdditiveExpr<'a>),
    Range(AdditiveExpr<'a>, AdditiveExpr<'a>),
}

impl RangeExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> RangeExpr<'static> {
        match self {
            Self::Single(x) => RangeExpr::Single(x.into_owned()),
            Self::Range(a, b) => RangeExpr::Range(a.into_owned(), b.into_owned()),
        }
    }
}

impl_nom_parser! {
    fn RangeExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        let (rest, first) = AdditiveExpr::nom(rest)?;
        let (rest, second) =
            opt(preceded(trim0(tag_no_case("to")), AdditiveExpr::nom)).parse(rest)?;

        let ret = match second {
            Some(second) => Self::Range(first, second),
            None => Self::Single(first),
        };

        Ok((rest, ret))
    }
}

impl Display for RangeExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Single(x) => x.fmt(f),
            Self::Range(a, b) => write!(f, "{a} to {b}"),
        }
    }
}

/// Implementation of [`AdditiveExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-AdditiveExpr).
///
/// ```
/// [21] AdditiveExpr ::= MultiplicativeExpr ( ( "+" | "-" ) MultiplicativeExpr )*
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AdditiveExpr<'a> {
    pub head: MultiplicativeExpr<'a>,
    pub tail: Vec<(AdditiveExprOp, MultiplicativeExpr<'a>)>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AdditiveExprOp {
    Add,
    Sub,
}

impl AdditiveExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> AdditiveExpr<'static> {
        AdditiveExpr {
            head: self.head.into_owned(),
            tail: self
                .tail
                .into_iter()
                .map(|(op, expr)| (op, expr.into_owned()))
                .collect(),
        }
    }
}

impl_nom_parser! {
    fn AdditiveExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        let (rest, head) = MultiplicativeExpr::nom.parse(rest)?;
        let (rest, tail) = many0((
            AdditiveExprOp::nom,
            MultiplicativeExpr::nom,
        )).parse(rest)?;

        Ok((rest, Self {
            head,
            tail,
        }))
    }
}

impl_nom_parser! {
    fn AdditiveExprOp::nom(input: &[u8]) -> IResult<&[u8], Self> {
        let rest = input.trim_ascii();

        alt((
            tag("+").map(|_| Self::Add),
            tag("-").map(|_| Self::Sub),
        )).parse(rest)
    }
}

impl Display for AdditiveExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { head, tail } = self;

        head.fmt(f)?;

        for (op, expr) in tail {
            write!(f, " {op} {expr}")?;
        }

        Ok(())
    }
}

impl Display for AdditiveExprOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
        }
    }
}

/// Implementation of [`MultiplicativeExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-MultiplicativeExpr).
///
/// ```
/// [22] MultiplicativeExpr ::= UnionExpr ( ( "*" | "div" | "idiv" | "mod" ) UnionExpr )*
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MultiplicativeExpr<'a> {
    pub head: UnionExpr<'a>,
    pub tail: Vec<(MultiplicativeExprOp, UnionExpr<'a>)>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MultiplicativeExprOp {
    Mul,
    Div,
    IDiv,
    Mod,
}

impl MultiplicativeExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> MultiplicativeExpr<'static> {
        MultiplicativeExpr {
            head: self.head.into_owned(),
            tail: self
                .tail
                .into_iter()
                .map(|(op, expr)| (op, expr.into_owned()))
                .collect(),
        }
    }
}

impl_nom_parser! {
    fn MultiplicativeExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        let (rest, head) = UnionExpr::nom.parse(rest)?;
        let (rest, tail) = many0((
            MultiplicativeExprOp::nom,
            UnionExpr::nom,
        )).parse(rest)?;

        Ok((rest, Self {
            head,
            tail,
        }))
    }
}

impl_nom_parser! {
    fn MultiplicativeExprOp::nom(input: &[u8]) -> IResult<&[u8], Self> {
        let rest = input.trim_ascii();

        alt((
            tag("*").map(|_| Self::Mul),
            tag_no_case("div").map(|_| Self::Div),
            tag_no_case("idiv").map(|_| Self::IDiv),
            tag_no_case("mod").map(|_| Self::Mod),
        )).parse(rest)
    }
}

impl Display for MultiplicativeExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { head, tail } = self;

        head.fmt(f)?;

        for (op, expr) in tail {
            write!(f, " {op} {expr}")?;
        }

        Ok(())
    }
}

impl Display for MultiplicativeExprOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "div"),
            Self::IDiv => write!(f, "idiv"),
            Self::Mod => write!(f, "mod"),
        }
    }
}

/// Implementation of [`UnionExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-UnionExpr).
///
/// ```
/// [23] UnionExpr ::= IntersectExceptExpr ( ( "union" | "|" ) IntersectExceptExpr )*
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnionExpr<'a>(pub Vec<IntersectExceptExpr<'a>>);

impl UnionExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> UnionExpr<'static> {
        UnionExpr(
            self.0
                .into_iter()
                .map(IntersectExceptExpr::into_owned)
                .collect(),
        )
    }
}

impl_deref_mut!(UnionExpr<'a>, Vec<IntersectExceptExpr<'a>>);

impl_nom_parser! {
    fn UnionExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        separated_list1(trim0(alt((tag_no_case("union"), tag("|")))), IntersectExceptExpr::nom)
            .map(Self)
            .parse(rest)
    }
}

impl Display for UnionExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for (i, expr) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " | {expr}")?;
            } else {
                expr.fmt(f)?;
            }
        }

        Ok(())
    }
}

/// Implementation of [`IntersectExceptExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-IntersectExceptExpr).
///
/// ```
/// [24] IntersectExceptExpr ::= InstanceofExpr ( ( "intersect" | "except" ) InstanceofExpr )*
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IntersectExceptExpr<'a> {
    pub head: InstanceOfExpr<'a>,
    pub tail: Vec<(IntersectExceptExprOp, InstanceOfExpr<'a>)>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum IntersectExceptExprOp {
    Except,
    Intersect,
}

impl IntersectExceptExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> IntersectExceptExpr<'static> {
        IntersectExceptExpr {
            head: self.head.into_owned(),
            tail: self
                .tail
                .into_iter()
                .map(|(op, expr)| (op, expr.into_owned()))
                .collect(),
        }
    }
}

impl_nom_parser! {
    fn IntersectExceptExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        let (rest, head) = InstanceOfExpr::nom.parse(rest)?;
        let (rest, tail) = many0((
            IntersectExceptExprOp::nom,
            InstanceOfExpr::nom,
        )).parse(rest)?;

        Ok((rest, Self {
            head,
            tail,
        }))
    }
}

impl_nom_parser! {
    fn IntersectExceptExprOp::nom(input: &[u8]) -> IResult<&[u8], Self> {
        let rest = input.trim_ascii();

        alt((
            tag_no_case("except").map(|_| Self::Except),
            tag_no_case("intersect").map(|_| Self::Intersect),
        )).parse(rest)
    }
}

impl Display for IntersectExceptExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { head, tail } = self;

        head.fmt(f)?;

        for (op, expr) in tail {
            write!(f, " {op} {expr}")?;
        }

        Ok(())
    }
}

impl Display for IntersectExceptExprOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Except => write!(f, "except"),
            Self::Intersect => write!(f, "intersect"),
        }
    }
}

/// Implementation of [`InstanceofExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-InstanceofExpr).
///
/// ```
/// [25] InstanceofExpr ::= TreatExpr ( "instance" "of" SequenceType )?
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InstanceOfExpr<'a> {
    pub expr: TreatExpr<'a>,
    pub type_: Option<SequenceType<'a>>,
}

impl InstanceOfExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> InstanceOfExpr<'static> {
        InstanceOfExpr {
            expr: self.expr.into_owned(),
            type_: self.type_.map(SequenceType::into_owned),
        }
    }
}

impl_nom_parser! {
    fn InstanceOfExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        let (rest, expr) = TreatExpr::nom(rest)?;
        let (rest, type_) = opt(preceded(
            (
                trim0(tag_no_case("instance")),
                trim0(tag_no_case("of")),
            ),
            SequenceType::nom,
        ))
        .parse(rest)?;

        Ok((rest, Self { expr, type_ }))
    }
}

impl Display for InstanceOfExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { expr, type_ } = self;

        expr.fmt(f)?;

        if let Some(type_) = type_ {
            write!(f, " instance of {type_}")?;
        }

        Ok(())
    }
}

/// Implementation of [`TreatExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-TreatExpr).
///
/// ```
/// [26] TreatExpr ::= CastableExpr ( "treat" "as" SequenceType )?
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TreatExpr<'a> {
    pub expr: CastableExpr<'a>,
    pub type_: Option<SequenceType<'a>>,
}

impl TreatExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> TreatExpr<'static> {
        TreatExpr {
            expr: self.expr.into_owned(),
            type_: self.type_.map(SequenceType::into_owned),
        }
    }
}

impl_nom_parser! {
    fn TreatExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        let (rest, expr) = CastableExpr::nom(rest)?;
        let (rest, type_) = opt(preceded(
            (
            trim0(tag_no_case("treat")),
            trim0(tag_no_case("as")),
            ),
            SequenceType::nom,
        ))
        .parse(rest)?;

        Ok((rest, Self { expr, type_ }))
    }
}

impl Display for TreatExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { expr, type_ } = self;

        expr.fmt(f)?;

        if let Some(type_) = type_ {
            write!(f, " treat as {type_}")?;
        }

        Ok(())
    }
}

/// Implementation of [`CastableExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-CastableExpr).
///
/// ```
/// [27] CastableExpr ::= CastExpr ( "castable" "as" SingleType )?
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CastableExpr<'a> {
    pub expr: CastExpr<'a>,
    pub type_: Option<SingleType<'a>>,
}

impl CastableExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> CastableExpr<'static> {
        CastableExpr {
            expr: self.expr.into_owned(),
            type_: self.type_.map(SingleType::into_owned),
        }
    }
}

impl_nom_parser! {
    fn CastableExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        let (rest, expr) = CastExpr::nom(rest)?;
        let (rest, type_) = opt(preceded(
            (
                trim0(tag_no_case("castable")),
                trim0(tag_no_case("as")),
            ),
            SingleType::nom,
        ))
        .parse(rest)?;

        Ok((rest, Self { expr, type_ }))
    }
}

impl Display for CastableExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { expr, type_ } = self;

        expr.fmt(f)?;

        if let Some(type_) = type_ {
            write!(f, " castable as {type_}")?;
        }

        Ok(())
    }
}

/// Implementation of [`CastExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-CastExpr).
///
/// ```
/// [28] CastExpr ::= ArrowExpr ( "cast" "as" SingleType )?
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CastExpr<'a> {
    pub expr: ArrowExpr<'a>,
    pub type_: Option<SingleType<'a>>,
}

impl CastExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> CastExpr<'static> {
        CastExpr {
            expr: self.expr.into_owned(),
            type_: self.type_.map(SingleType::into_owned),
        }
    }
}

impl_nom_parser! {
    fn CastExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        let (rest, expr) = ArrowExpr::nom(rest)?;
        let (rest, type_) = opt(preceded(
            (
            trim0(tag_no_case("cast")),
            trim0(tag_no_case("as")),
            ),
            SingleType::nom,
        ))
        .parse(rest)?;

        Ok((rest, Self { expr, type_ }))
    }
}

impl Display for CastExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { expr, type_ } = self;

        expr.fmt(f)?;

        if let Some(type_) = type_ {
            write!(f, " cast as {type_}")?;
        }

        Ok(())
    }
}

/// Implementation of [`ArrowExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-ArrowExpr).
///
/// ```
/// [29] ArrowExpr ::= UnaryExpr ( "=>" ArrowFunctionSpecifier ArgumentList )*
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArrowExpr<'a> {
    pub expr: UnaryExpr<'a>,
    pub functions: Vec<(ArrowFunctionSpecifier<'a>, ArgumentList<'a>)>,
}

impl ArrowExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> ArrowExpr<'static> {
        ArrowExpr {
            expr: self.expr.into_owned(),
            functions: self
                .functions
                .into_iter()
                .map(|(a, b)| (a.into_owned(), b.into_owned()))
                .collect(),
        }
    }
}

impl_nom_parser! {
    fn ArrowExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        let (rest, expr) = UnaryExpr::nom(rest)?;
        let (rest, functions) = many0((
            preceded(trim0(tag("=>")), trim0(ArrowFunctionSpecifier::nom)),
            trim0(ArgumentList::nom),
        ))
        .parse(rest)?;

        Ok((rest, Self { expr, functions }))
    }
}

impl Display for ArrowExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.expr)?;

        for (func, args) in &self.functions {
            write!(f, " => {func}{args}")?;
        }

        Ok(())
    }
}

/// Implementation of [`UnaryExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-UnaryExpr).
///
/// ```
/// [30] UnaryExpr ::= ("-" | "+")* ValueExpr
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnaryExpr<'a> {
    pub operators: Vec<UnaryExprOp>,
    pub expr: ValueExpr<'a>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnaryExprOp {
    Pos,
    Neg,
}

impl UnaryExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> UnaryExpr<'static> {
        UnaryExpr {
            operators: self.operators,
            expr: self.expr.into_owned(),
        }
    }
}

impl_nom_parser! {
    fn UnaryExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        let (rest, operators) = many0(alt((
            tag("+").map(|_| UnaryExprOp::Pos),
            tag("-").map(|_| UnaryExprOp::Neg),
        )))
        .parse(rest)?;
        let (rest, expr) = ValueExpr::nom(rest)?;

        Ok((rest, Self { operators, expr }))
    }
}

impl Display for UnaryExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for op in &self.operators {
            write!(f, "{op}")?;
        }

        write!(f, "{}", self.expr)
    }
}

impl Display for UnaryExprOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Pos => write!(f, "+"),
            Self::Neg => write!(f, "-"),
        }
    }
}

/// Implementation of [`ValueExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-ValueExpr).
///
/// ```
/// [31] ValueExpr ::= SimpleMapExpr
/// ```
pub type ValueExpr<'a> = SimpleMapExpr<'a>;

/// Implementation of [`SimpleMapExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-SimpleMapExpr).
///
/// ```
/// [35] SimpleMapExpr ::= PathExpr ("!" PathExpr)*
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SimpleMapExpr<'a>(pub Vec<PathExpr<'a>>);

impl SimpleMapExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> SimpleMapExpr<'static> {
        SimpleMapExpr(self.0.into_iter().map(PathExpr::into_owned).collect())
    }
}

impl_deref_mut!(SimpleMapExpr<'a>, Vec<PathExpr<'a>>);

impl_nom_parser! {
    fn SimpleMapExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        separated_list1(trim0(tag("!")), PathExpr::nom)
            .map(Self)
            .parse(rest)
    }
}

impl Display for SimpleMapExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for (i, expr) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, "!{expr}")?;
            } else {
                expr.fmt(f)?;
            }
        }

        Ok(())
    }
}

/// Implementation of [`PathExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-PathExpr).
///
/// ```
/// [36] PathExpr ::= ("/" RelativePathExpr?) | ("//" RelativePathExpr) | RelativePathExpr
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PathExpr<'a> {
    Root(Option<RelativePathExpr<'a>>),
    Descendant(RelativePathExpr<'a>),
    Relative(RelativePathExpr<'a>),
}

impl PathExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> PathExpr<'static> {
        match self {
            Self::Root(x) => PathExpr::Root(x.map(RelativePathExpr::into_owned)),
            Self::Descendant(x) => PathExpr::Descendant(x.into_owned()),
            Self::Relative(x) => PathExpr::Relative(x.into_owned()),
        }
    }
}

impl_nom_parser! {
    fn PathExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        alt((
            // Match "//" RelativePathExpr
            preceded(tag("//"), RelativePathExpr::nom.map(Self::Descendant)),
            // Match "/" [RelativePathExpr]
            preceded(tag("/"), opt(RelativePathExpr::nom).map(Self::Root)),
            // Match RelativePathExpr
            RelativePathExpr::nom.map(Self::Relative),
        ))
        .parse(rest)
    }
}

impl Display for PathExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Root(None) => write!(f, "/"),
            Self::Root(Some(expr)) => write!(f, "/{}", expr),
            Self::Descendant(expr) => write!(f, "//{}", expr),
            Self::Relative(expr) => write!(f, "{}", expr),
        }
    }
}

/// Implementation of [`RelativePathExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-RelativePathExpr).
///
/// ```
/// [37] RelativePathExpr ::= StepExpr (("/" | "//") StepExpr)*
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RelativePathExpr<'a> {
    pub head: StepExpr<'a>,
    pub tail: Vec<(RelativePathExprOp, StepExpr<'a>)>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RelativePathExprOp {
    Child,
    Descendant,
}

impl RelativePathExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> RelativePathExpr<'static> {
        RelativePathExpr {
            head: self.head.into_owned(),
            tail: self
                .tail
                .into_iter()
                .map(|(op, expr)| (op, expr.into_owned()))
                .collect(),
        }
    }
}

impl_nom_parser! {
    fn RelativePathExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        let (rest, head) = StepExpr::nom.parse(rest)?;
        let (rest, tail) = many0((
            RelativePathExprOp::nom,
            StepExpr::nom,
        )).parse(rest)?;

        Ok((rest, Self {
            head,
            tail,
        }))
    }
}

impl_nom_parser! {
    fn RelativePathExprOp::nom(input: &[u8]) -> IResult<&[u8], Self> {
        let rest = input.trim_ascii();

        alt((
            tag("//").map(|_| Self::Descendant),
            tag("/").map(|_| Self::Child),
        ))
        .parse(rest)
    }
}

impl Display for RelativePathExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { head, tail } = self;

        head.fmt(f)?;

        for (op, step) in tail {
            write!(f, "{op}{step}")?;
        }

        Ok(())
    }
}

impl Display for RelativePathExprOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Child => write!(f, "/"),
            Self::Descendant => write!(f, "//"),
        }
    }
}

/// Implementation of [`StepExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-StepExpr).
///
/// ```
/// [38] StepExpr ::= PostfixExpr | AxisStep
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StepExpr<'a> {
    Axis(AxisStep<'a>),
    Postfix(PostfixExpr<'a>),
}

impl StepExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> StepExpr<'static> {
        match self {
            Self::Axis(x) => StepExpr::Axis(x.into_owned()),
            Self::Postfix(x) => StepExpr::Postfix(x.into_owned()),
        }
    }
}

impl_nom_parser! {
    fn StepExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        enum Preview {
            Unknown,
            Axis,
            Postfix
        }

        let rest = input.trim_ascii();

        let preview = alt((
            // ".." => Axis
            (tag("..")).map(|_| Preview::Axis),

            // "." AnyChar => Postfix
            (tag("."), verify(take(1usize), |b: &[u8]| b != b".")).map(|_| Preview::Postfix),

            // QName "[" => Axis
            // QName "/" => Axis
            // QName "(" => Postfix
            preceded((QName::nom, multispace0), alt((
                tag("[").map(|_| Preview::Axis),
                tag("/").map(|_| Preview::Axis),
                tag("(").map(|_| Preview::Postfix),
            ))),
        )).parse(rest).map(|(_, p)| p).unwrap_or(Preview::Unknown);

        match preview {
            Preview::Axis => AxisStep::nom.map(Self::Axis).parse(rest),
            Preview::Postfix => PostfixExpr::nom.map(Self::Postfix).parse(rest),
            Preview::Unknown => {
                let ret_axis = AxisStep::nom.map(Self::Axis).parse(rest);
                let ret_postfix = PostfixExpr::nom.map(Self::Postfix).parse(rest);

                match (ret_axis, ret_postfix) {
                    (Ok((rest_axis, axis)), Ok((rest_postfix, postfix))) => {
                        if rest_axis.len() < rest_postfix.len() {
                            Ok((rest_axis, axis))
                        } else {
                            Ok((rest_postfix, postfix))
                        }
                    },
                    (Ok((rest_axis, axis)), Err(_)) => Ok((rest_axis, axis)),
                    (Err(_), Ok((rest_postfix, postfix))) => Ok((rest_postfix, postfix)),
                    (Err(_), Err(_)) => Err(Err::Error(Error::new(rest, ErrorKind::Alt))),
                }
            }
        }
    }
}

impl Display for StepExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            StepExpr::Axis(expr) => expr.fmt(f),
            StepExpr::Postfix(expr) => expr.fmt(f),
        }
    }
}

/// Implementation of [`AxisStep`](https://www.w3.org/TR/xpath-31/#prod-xpath31-AxisStep).
///
/// ```
/// [39] AxisStep          ::= (ReverseStep | ForwardStep) PredicateList
/// [40] ForwardStep       ::= (ForwardAxis NodeTest) | AbbrevForwardStep
/// [41] ForwardAxis       ::= ("child" "::")
///                                | ("descendant" "::")
///                                | ("attribute" "::")
///                                | ("self" "::")
///                                | ("descendant-or-self" "::")
///                                | ("following-sibling" "::")
///                                | ("following" "::")
///                                | ("namespace" "::")
/// [42] AbbrevForwardStep ::= "@"? NodeTest
/// [43] ReverseStep       ::= (ReverseAxis NodeTest) | AbbrevReverseStep
/// [44] ReverseAxis       ::= ("parent" "::")
///                                | ("ancestor" "::")
///                                | ("preceding-sibling" "::")
///                                | ("preceding" "::")
///                                | ("ancestor-or-self" "::")
/// [45] AbbrevReverseStep ::= ".."
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AxisStep<'a> {
    pub op: AxisStepOp,
    pub test: NodeTest<'a>,
    pub predicates: PredicateList<'a>,
}

/// Helper type for [`AxisStep`].
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AxisStepOp {
    /* Forward */
    Child,
    Descendant,
    Attribute,
    Self_,
    DescendantOrSelf,
    FollowingSibling,
    Following,
    Namespace,

    /* Backward */
    Parent,
    Ancestor,
    PrecedingSibling,
    Preceding,
    AncestorOrSelf,
}

impl AxisStep<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> AxisStep<'static> {
        AxisStep {
            op: self.op,
            test: self.test.into_owned(),
            predicates: self.predicates.into_owned(),
        }
    }
}

impl_nom_parser! {
    fn AxisStep<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        fn test(
            s: &str,
            op: AxisStepOp,
        ) -> impl Parser<&[u8], Output = AxisStepOp, Error = Error<&[u8]>> {
            (multispace0, tag(s), multispace0, tag("::"), multispace0).map(move |_| op)
        }

        let rest = input.trim_ascii();

        #[rustfmt::skip]
        let (rest, (op, test)) = alt((
            /* ForwardAxis */
            (test("self", AxisStepOp::Self_), NodeTest::nom),
            (test("child", AxisStepOp::Child), NodeTest::nom),
            (test("namespace", AxisStepOp::Namespace), NodeTest::nom),
            (test("attribute", AxisStepOp::Attribute), NodeTest::nom),
            (test("descendant", AxisStepOp::Descendant), NodeTest::nom),
            (test("descendant-or-self", AxisStepOp::DescendantOrSelf), NodeTest::nom),
            (test("following", AxisStepOp::Following), NodeTest::nom),
            (test("following-sibling", AxisStepOp::FollowingSibling), NodeTest::nom),

            /* AbbrevForwardStep */
            (
                opt((multispace0, tag("@"))).map(|op| {
                    if op.is_some() {
                        AxisStepOp::Attribute
                    } else {
                        AxisStepOp::Child
                    }
                }),
                NodeTest::nom,
            ),

            /* ReverseAxis */
            (test("parent", AxisStepOp::Parent), NodeTest::nom),
            (test("ancestor", AxisStepOp::Ancestor), NodeTest::nom),
            (test("ancestor-or-self", AxisStepOp::AncestorOrSelf), NodeTest::nom),
            (test("preceding", AxisStepOp::Preceding), NodeTest::nom),
            (test("preceding-sibling", AxisStepOp::PrecedingSibling), NodeTest::nom),

            /* AbbrevReverseStep */
            tag("..").map(|_| (AxisStepOp::Parent, NodeTest::node())),
        ))
        .parse(rest)?;
        let (rest, predicates) = PredicateList::nom(rest)?;

        let ret = Self {
            op,
            test,
            predicates,
        };

        Ok((rest, ret))
    }
}

impl Display for AxisStep<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self {
            op,
            test,
            predicates,
        } = self;

        match op {
            // Abbreviated reverse
            AxisStepOp::Parent
                if matches!(test, NodeTest::Kind(KindTest::AnyKind)) && predicates.is_empty() =>
            {
                write!(f, "..")
            }

            // Abbreviated forward
            AxisStepOp::Attribute if matches!(test, NodeTest::Name(NameTest::EQName(_))) => {
                write!(f, "@{test}{predicates}")
            }
            AxisStepOp::Child if matches!(self.test, NodeTest::Name(_)) => {
                write!(f, "{test}{predicates}")
            }

            // Full form
            op => write!(f, "{op}::{test}{predicates}"),
        }
    }
}

impl Display for AxisStepOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Self_ => write!(f, "self"),
            Self::Child => write!(f, "child"),
            Self::Attribute => write!(f, "attribute"),
            Self::Descendant => write!(f, "descendant"),
            Self::DescendantOrSelf => write!(f, "descendant-or-self"),
            Self::Following => write!(f, "following"),
            Self::FollowingSibling => write!(f, "following-sibling"),
            Self::Namespace => write!(f, "namespace"),
            Self::Parent => write!(f, "parent"),
            Self::Ancestor => write!(f, "ancestor"),
            Self::Preceding => write!(f, "preceding"),
            Self::PrecedingSibling => write!(f, "preceding-sibling"),
            Self::AncestorOrSelf => write!(f, "ancestor-or-self"),
        }
    }
}

/// Implementation of [`NodeTest`](https://www.w3.org/TR/xpath-31/#prod-xpath31-NodeTest).
///
/// ```
/// [46] NodeTest ::= KindTest | NameTest
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NodeTest<'a> {
    Kind(KindTest<'a>),
    Name(NameTest<'a>),
}

impl NodeTest<'_> {
    #[must_use]
    pub fn node() -> Self {
        Self::Kind(KindTest::AnyKind)
    }

    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> NodeTest<'static> {
        match self {
            Self::Kind(x) => NodeTest::Kind(x.into_owned()),
            Self::Name(x) => NodeTest::Name(x.into_owned()),
        }
    }
}

impl_nom_parser! {
    fn NodeTest<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        alt((KindTest::nom.map(Self::Kind), NameTest::nom.map(Self::Name))).parse(rest)
    }
}

impl Display for NodeTest<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Kind(x) => x.fmt(f),
            Self::Name(x) => x.fmt(f),
        }
    }
}

/// Implementation of [`NameTest`](https://www.w3.org/TR/xpath-31/#prod-xpath31-NameTest).
///
/// ```
/// [47] NameTest ::= EQName | Wildcard
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NameTest<'a> {
    EQName(EQName<'a>),
    Wildcard(Wildcard<'a>),
}

impl NameTest<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> NameTest<'static> {
        match self {
            Self::EQName(x) => NameTest::EQName(x.into_owned()),
            Self::Wildcard(x) => NameTest::Wildcard(x.into_owned()),
        }
    }
}

impl_nom_parser! {
    fn NameTest<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        alt((
            Wildcard::nom.map(Self::Wildcard),
            EQName::nom.map(Self::EQName),
        ))
        .parse(rest)
    }
}

impl Display for NameTest<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            NameTest::EQName(x) => x.fmt(f),
            NameTest::Wildcard(x) => x.fmt(f),
        }
    }
}

/// Implementation of [`Wildcard`](https://www.w3.org/TR/xpath-31/#prod-xpath31-Wildcard).
///
/// ```
/// [48] Wildcard ::= "*" | (NCName ":*") | ("*:" NCName) | (BracedURILiteral "*")
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Wildcard<'a> {
    /// Any: `*`
    Any,

    /// Suffix: `NCName:*`
    Suffix(NCName<'a>),

    /// Prefix: `*:NCName`
    Prefix(NCName<'a>),

    /// URI qualified `Q{uri}*`
    UriQualified(BracedURILiteral<'a>),
}

impl Wildcard<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> Wildcard<'static> {
        match self {
            Self::Any => Wildcard::Any,
            Self::Suffix(x) => Wildcard::Suffix(x.into_owned()),
            Self::Prefix(x) => Wildcard::Prefix(x.into_owned()),
            Self::UriQualified(x) => Wildcard::UriQualified(x.into_owned()),
        }
    }
}

impl_nom_parser! {
    fn Wildcard<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        alt((
            // BracedURILiteral "*"
            terminated(BracedURILiteral::nom, tag("*")).map(Self::UriQualified),
            // NCName:*
            terminated(NCName::nom, tag(":*")).map(Self::Suffix),
            // *:NCName
            preceded(tag("*:"), NCName::nom).map(Self::Prefix),
            // *
            tag("*").map(|_| Self::Any),
        ))
        .parse(rest)
    }
}

impl Display for Wildcard<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Any => write!(f, "*"),
            Self::Suffix(x) => write!(f, "{x}:*"),
            Self::Prefix(x) => write!(f, "*:{x}"),
            Self::UriQualified(x) => write!(f, "{x}*"),
        }
    }
}

/// Implementation of [`PostfixExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-PostfixExpr).
///
/// ```
/// [49] PostfixExpr ::= PrimaryExpr ( Predicate | ArgumentList | Lookup )*
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PostfixExpr<'a> {
    pub primary: PrimaryExpr<'a>,
    pub suffixes: Vec<PostfixExprOp<'a>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PostfixExprOp<'a> {
    Lookup(Lookup<'a>),
    Predicate(Predicate<'a>),
    ArgumentList(ArgumentList<'a>),
}

impl PostfixExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> PostfixExpr<'static> {
        PostfixExpr {
            primary: self.primary.into_owned(),
            suffixes: self
                .suffixes
                .into_iter()
                .map(PostfixExprOp::into_owned)
                .collect(),
        }
    }
}

impl_nom_parser! {
    fn PostfixExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        let (rest, primary) = PrimaryExpr::nom(rest)?;
        let (rest, suffixes) = many0(alt((
            Predicate::nom.map(PostfixExprOp::Predicate),
            ArgumentList::nom.map(PostfixExprOp::ArgumentList),
            Lookup::nom.map(PostfixExprOp::Lookup),
        )))
        .parse(rest)?;

        Ok((rest, Self { primary, suffixes }))
    }
}

impl Display for PostfixExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.primary.fmt(f)?;

        for suffix in &self.suffixes {
            match suffix {
                PostfixExprOp::Predicate(x) => x.fmt(f)?,
                PostfixExprOp::ArgumentList(x) => x.fmt(f)?,
                PostfixExprOp::Lookup(x) => x.fmt(f)?,
            }
        }

        Ok(())
    }
}

impl PostfixExprOp<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> PostfixExprOp<'static> {
        match self {
            Self::Lookup(x) => PostfixExprOp::Lookup(x.into_owned()),
            Self::Predicate(x) => PostfixExprOp::Predicate(x.into_owned()),
            Self::ArgumentList(x) => PostfixExprOp::ArgumentList(x.into_owned()),
        }
    }
}

/// Implementation of [`ArgumentList`](https://www.w3.org/TR/xpath-31/#prod-xpath31-ArgumentList).
///
/// ```
/// [50] ArgumentList ::= "(" (Argument ("," Argument)*)? ")"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArgumentList<'a>(pub Vec<Argument<'a>>);

impl ArgumentList<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> ArgumentList<'static> {
        ArgumentList(self.0.into_iter().map(Argument::into_owned).collect())
    }
}

impl_deref_mut!(ArgumentList<'a>, Vec<Argument<'a>>);

impl_nom_parser! {
    fn ArgumentList<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        delimited(
            trim0(tag("(")),
            separated_list0(trim0(tag(",")), Argument::nom),
            trim0(tag(")")),
        )
        .map(Self)
        .parse(rest)
    }
}

impl Display for ArgumentList<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "(")?;

        for (i, arg) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", {}", arg)?;
            } else {
                write!(f, "{}", arg)?;
            }
        }

        write!(f, ")")?;

        Ok(())
    }
}

/// Implementation of [`PredicateList`](https://www.w3.org/TR/xpath-31/#prod-xpath31-PredicateList).
///
/// ```
/// [51] PredicateList ::= Predicate*
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PredicateList<'a>(pub Vec<Predicate<'a>>);

impl PredicateList<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> PredicateList<'static> {
        PredicateList(self.0.into_iter().map(Predicate::into_owned).collect())
    }
}

impl_deref_mut!(PredicateList<'a>, Vec<Predicate<'a>>);

impl_nom_parser! {
    fn PredicateList<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        many0(Predicate::nom).map(Self).parse(rest)
    }
}

impl Display for PredicateList<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for pred in &self.0 {
            write!(f, "{pred}")?;
        }
        Ok(())
    }
}

/// Implementation of [`Predicate`](https://www.w3.org/TR/xpath-31/#prod-xpath31-Predicate).
///
/// ```
/// [52] Predicate ::= "[" Expr "]"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Predicate<'a>(pub Expr<'a>);

impl Predicate<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> Predicate<'static> {
        Predicate(self.0.into_owned())
    }
}

impl_deref_mut!(Predicate<'a>, Expr<'a>);

impl_nom_parser! {
    fn Predicate<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        delimited(trim0(tag("[")), Expr::nom, trim0(tag("]")))
            .map(Self)
            .parse(rest)
    }
}

impl Display for Predicate<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "[{}]", self.0)
    }
}

/// Implementation of [`Lookup`](https://www.w3.org/TR/xpath-31/#prod-xpath31-Lookup).
///
/// ```
/// [53] Lookup ::= "?" KeySpecifier
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Lookup<'a>(pub KeySpecifier<'a>);

impl Lookup<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> Lookup<'static> {
        Lookup(self.0.into_owned())
    }
}

impl_deref_mut!(Lookup<'a>, KeySpecifier<'a>);

impl_nom_parser! {
    fn Lookup<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        preceded(tag("?"), KeySpecifier::nom).map(Self).parse(rest)
    }
}

impl Display for Lookup<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "?{}", self.0)
    }
}

/// Implementation of [`KeySpecifier`](https://www.w3.org/TR/xpath-31/#prod-xpath31-KeySpecifier).
///
/// ```
/// [54] KeySpecifier ::= NCName | IntegerLiteral | ParenthesizedExpr | "*"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum KeySpecifier<'a> {
    Wildcard,
    NCName(NCName<'a>),
    Integer(IntegerLiteral),
    Parens(ParenthesizedExpr<'a>),
}

impl KeySpecifier<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> KeySpecifier<'static> {
        match self {
            Self::Wildcard => KeySpecifier::Wildcard,
            Self::NCName(x) => KeySpecifier::NCName(x.into_owned()),
            Self::Integer(x) => KeySpecifier::Integer(x),
            Self::Parens(x) => KeySpecifier::Parens(x.into_owned()),
        }
    }
}

impl_nom_parser! {
    fn KeySpecifier<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        alt((
            tag("*").map(|_| Self::Wildcard),
            ParenthesizedExpr::nom.map(Self::Parens),
            IntegerLiteral::nom.map(Self::Integer),
            NCName::nom.map(Self::NCName),
        ))
        .parse(rest)
    }
}

impl Display for KeySpecifier<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NCName(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
            Self::Parens(x) => x.fmt(f),
            Self::Wildcard => write!(f, "*"),
        }
    }
}

/// Implementation of [`ArrowFunctionSpecifier`](https://www.w3.org/TR/xpath-31/#prod-xpath31-ArrowFunctionSpecifier).
///
/// ```
/// [55] ArrowFunctionSpecifier ::= EQName | VarRef | ParenthesizedExpr
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ArrowFunctionSpecifier<'a> {
    EQName(EQName<'a>),
    VarRef(VarRef<'a>),
    Parens(ParenthesizedExpr<'a>),
}

impl ArrowFunctionSpecifier<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> ArrowFunctionSpecifier<'static> {
        match self {
            Self::EQName(x) => ArrowFunctionSpecifier::EQName(x.into_owned()),
            Self::VarRef(x) => ArrowFunctionSpecifier::VarRef(x.into_owned()),
            Self::Parens(x) => ArrowFunctionSpecifier::Parens(x.into_owned()),
        }
    }
}

impl_nom_parser! {
    fn ArrowFunctionSpecifier<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        alt((
            VarRef::nom.map(Self::VarRef),
            ParenthesizedExpr::nom.map(Self::Parens),
            EQName::nom.map(Self::EQName),
        ))
        .parse(rest)
    }
}

impl Display for ArrowFunctionSpecifier<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::EQName(x) => x.fmt(f),
            Self::VarRef(x) => x.fmt(f),
            Self::Parens(x) => x.fmt(f),
        }
    }
}

/// Implementation of [`PrimaryExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-PrimaryExpr).
///
/// ```
/// [56] PrimaryExpr     ::= Literal
///                          | VarRef
///                          | ParenthesizedExpr
///                          | ContextItemExpr
///                          | FunctionCall
///                          | FunctionItemExpr
///                          | MapConstructor
///                          | ArrayConstructor
///                          | UnaryLookup`
/// [62] ContextItemExpr ::= "."
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PrimaryExpr<'a> {
    Literal(Literal<'a>),
    VarRef(VarRef<'a>),
    Parens(ParenthesizedExpr<'a>),
    FunctionCall(FunctionCall<'a>),
    FunctionItem(FunctionItemExpr<'a>),
    MapConstructor(MapConstructor<'a>),
    ArrayConstructor(ArrayConstructor<'a>),
    UnaryLookup(UnaryLookup<'a>),
    ContextItem,
}

impl PrimaryExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> PrimaryExpr<'static> {
        match self {
            Self::Literal(x) => PrimaryExpr::Literal(x.into_owned()),
            Self::VarRef(x) => PrimaryExpr::VarRef(x.into_owned()),
            Self::Parens(x) => PrimaryExpr::Parens(x.into_owned()),
            Self::FunctionCall(x) => PrimaryExpr::FunctionCall(x.into_owned()),
            Self::FunctionItem(x) => PrimaryExpr::FunctionItem(x.into_owned()),
            Self::MapConstructor(x) => PrimaryExpr::MapConstructor(x.into_owned()),
            Self::ArrayConstructor(x) => PrimaryExpr::ArrayConstructor(x.into_owned()),
            Self::UnaryLookup(x) => PrimaryExpr::UnaryLookup(x.into_owned()),
            Self::ContextItem => PrimaryExpr::ContextItem,
        }
    }
}

impl_nom_parser! {
    fn PrimaryExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        alt((
            Literal::nom.map(Self::Literal),
            VarRef::nom.map(Self::VarRef),
            ParenthesizedExpr::nom.map(Self::Parens),
            FunctionCall::nom.map(Self::FunctionCall),
            FunctionItemExpr::nom.map(Self::FunctionItem),
            MapConstructor::nom.map(Self::MapConstructor),
            ArrayConstructor::nom.map(Self::ArrayConstructor),
            UnaryLookup::nom.map(Self::UnaryLookup),
            tag(".").map(|_| Self::ContextItem),
        ))
        .parse(rest)
    }
}

impl Display for PrimaryExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Literal(x) => x.fmt(f),
            Self::VarRef(x) => x.fmt(f),
            Self::Parens(x) => x.fmt(f),
            Self::FunctionCall(x) => x.fmt(f),
            Self::FunctionItem(x) => x.fmt(f),
            Self::MapConstructor(x) => x.fmt(f),
            Self::ArrayConstructor(x) => x.fmt(f),
            Self::UnaryLookup(x) => x.fmt(f),
            Self::ContextItem => write!(f, "."),
        }
    }
}

/// Implementation of [`Literal`](https://www.w3.org/TR/xpath-31/#prod-xpath31-Literal).
///
/// ```
/// [57] Literal ::= NumericLiteral | StringLiteral
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Literal<'a> {
    String(StringLiteral<'a>),
    Numeric(NumericLiteral),
}

impl Literal<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> Literal<'static> {
        match self {
            Self::String(x) => Literal::String(x.into_owned()),
            Self::Numeric(x) => Literal::Numeric(x),
        }
    }
}

impl_nom_parser! {
    fn Literal<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        alt((
            StringLiteral::nom.map(Self::String),
            NumericLiteral::nom.map(Self::Numeric),
        ))
        .parse(rest)
    }
}

impl Display for Literal<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::String(s) => s.fmt(f),
            Self::Numeric(n) => n.fmt(f),
        }
    }
}

/// Implementation of [`NumericLiteral`](https://www.w3.org/TR/xpath-31/#prod-xpath31-NumericLiteral).
///
/// ```
/// [58] NumericLiteral ::= IntegerLiteral | DecimalLiteral | DoubleLiteral
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NumericLiteral {
    Double(DoubleLiteral),
    Decimal(DecimalLiteral),
    Integer(IntegerLiteral),
}

impl_nom_parser! {
    fn NumericLiteral::nom(input: &[u8]) -> IResult<&[u8], Self> {
        let rest = input.trim_ascii();

        alt((
            DoubleLiteral::nom.map(Self::Double),
            DecimalLiteral::nom.map(Self::Decimal),
            IntegerLiteral::nom.map(Self::Integer),
        ))
        .parse(rest)
    }
}

impl Display for NumericLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Integer(i) => i.fmt(f),
            Self::Decimal(d) => d.fmt(f),
            Self::Double(x) => x.fmt(f),
        }
    }
}

/// Implementation of [`VarRef`](https://www.w3.org/TR/xpath-31/#prod-xpath31-VarRef).
///
/// ```
/// [59] VarRef ::= "$" VarName
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VarRef<'a>(pub VarName<'a>);

impl VarRef<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> VarRef<'static> {
        VarRef(self.0.into_owned())
    }
}

impl_deref_mut!(VarRef<'a>, VarName<'a>);

impl_nom_parser! {
    fn VarRef<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        preceded(tag("$"), VarName::nom).map(Self).parse(rest)
    }
}

impl Display for VarRef<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "${}", self.0)
    }
}

/// Implementation of [`VarName`](https://www.w3.org/TR/xpath-31/#prod-xpath31-VarName).
///
/// ```
/// [60] VarName ::= EQName
/// ```
pub type VarName<'a> = EQName<'a>;

/// Implementation of [`ParenthesizedExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-ParenthesizedExpr).
///
/// ```
/// [61] ParenthesizedExpr ::= "(" Expr? ")"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParenthesizedExpr<'a>(pub Option<Expr<'a>>);

impl ParenthesizedExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> ParenthesizedExpr<'static> {
        ParenthesizedExpr(self.0.map(Expr::into_owned))
    }
}

impl_deref_mut!(ParenthesizedExpr<'a>, Option<Expr<'a>>);

impl_nom_parser! {
    fn ParenthesizedExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        delimited(trim0(tag("(")), opt(Expr::nom), trim0(tag(")")))
            .map(Self)
            .parse(rest)
    }
}

impl Display for ParenthesizedExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.0 {
            Some(expr) => write!(f, "({})", expr),
            None => write!(f, "()"),
        }
    }
}

/// Implementation of [`FunctionCall`](https://www.w3.org/TR/xpath-31/#prod-xpath31-FunctionCall).
///
/// ```
/// [63] FunctionCall ::= EQName ArgumentList
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionCall<'a> {
    pub name: EQName<'a>,
    pub arguments: ArgumentList<'a>,
}

impl FunctionCall<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> FunctionCall<'static> {
        FunctionCall {
            name: self.name.into_owned(),
            arguments: self.arguments.into_owned(),
        }
    }
}

impl_nom_parser! {
    fn FunctionCall<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        (EQName::nom, ArgumentList::nom)
            .map(|(name, arguments)| Self { name, arguments })
            .parse(rest)
    }
}

impl Display for FunctionCall<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}{}", self.name, self.arguments)
    }
}

/// Implementation of [`Argument`](https://www.w3.org/TR/xpath-31/#prod-xpath31-Argument).
///
/// ```
/// [64] Argument ::= ExprSingle | ArgumentPlaceholder
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Argument<'a> {
    Expr(ExprSingle<'a>),
    Placeholder,
}

impl Argument<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> Argument<'static> {
        match self {
            Self::Expr(x) => Argument::Expr(x.into_owned()),
            Self::Placeholder => Argument::Placeholder,
        }
    }
}

impl_nom_parser! {
    fn Argument<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        alt((
            tag("?").map(|_| Self::Placeholder),
            ExprSingle::nom.map(Self::Expr),
        ))
        .parse(rest)
    }
}

impl Display for Argument<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Expr(e) => e.fmt(f),
            Self::Placeholder => write!(f, "?"),
        }
    }
}

/// Implementation of [`FunctionItemExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-FunctionItemExpr).
///
/// ```
/// [66] FunctionItemExpr ::= NamedFunctionRef | InlineFunctionExpr
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FunctionItemExpr<'a> {
    Named(NamedFunctionRef<'a>),
    Inline(InlineFunctionExpr<'a>),
}

impl FunctionItemExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> FunctionItemExpr<'static> {
        match self {
            Self::Named(x) => FunctionItemExpr::Named(x.into_owned()),
            Self::Inline(x) => FunctionItemExpr::Inline(x.into_owned()),
        }
    }
}

impl_nom_parser! {
    fn FunctionItemExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        alt((
            NamedFunctionRef::nom.map(Self::Named),
            InlineFunctionExpr::nom.map(Self::Inline),
        ))
        .parse(rest)
    }
}

impl Display for FunctionItemExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Named(n) => n.fmt(f),
            Self::Inline(i) => i.fmt(f),
        }
    }
}

/// Implementation of [`NamedFunctionRef`](https://www.w3.org/TR/xpath-31/#prod-xpath31-NamedFunctionRef).
///
/// ```
/// [67] NamedFunctionRef ::= EQName "#" IntegerLiteral
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NamedFunctionRef<'a> {
    pub name: EQName<'a>,
    pub arity: IntegerLiteral,
}

impl NamedFunctionRef<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> NamedFunctionRef<'static> {
        NamedFunctionRef {
            name: self.name.into_owned(),
            arity: self.arity,
        }
    }
}

impl_nom_parser! {
    fn NamedFunctionRef<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        (EQName::nom, tag("#"), IntegerLiteral::nom)
            .map(|(name, _, arity)| Self { name, arity })
            .parse(rest)
    }
}

impl Display for NamedFunctionRef<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}#{}", self.name, self.arity)
    }
}

/// Implementation of [`InlineFunctionExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-InlineFunctionExpr).
///
/// ```
/// [68] InlineFunctionExpr ::= "function" "(" ParamList? ")" ("as" SequenceType)? FunctionBody
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InlineFunctionExpr<'a> {
    pub parameters: Option<ParamList<'a>>,
    pub return_type: Option<SequenceType<'a>>,
    pub body: FunctionBody<'a>,
}

impl InlineFunctionExpr<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> InlineFunctionExpr<'static> {
        InlineFunctionExpr {
            parameters: self.parameters.map(ParamList::into_owned),
            return_type: self.return_type.map(SequenceType::into_owned),
            body: self.body.into_owned(),
        }
    }
}

impl_nom_parser! {
    fn InlineFunctionExpr<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        (
            tag_no_case("function"),
            trim0(tag("(")),
            opt(ParamList::nom),
            trim0(tag(")")),
            opt((trim0(tag_no_case("as")), SequenceType::nom).map(|(_, t)| t)),
            FunctionBody::nom,
        )
            .map(|(_, _, parameters, _, return_type, body)| Self {
                parameters,
                return_type,
                body,
            })
            .parse(rest)
    }
}

impl Display for InlineFunctionExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "function(")?;

        if let Some(params) = &self.parameters {
            write!(f, "{params}")?;
        }

        write!(f, ")")?;

        if let Some(ty) = &self.return_type {
            write!(f, " as {ty}")?;
        }

        write!(f, " {}", self.body)?;

        Ok(())
    }
}

/// Implementation of [`MapConstructor`](https://www.w3.org/TR/xpath-31/#prod-xpath31-MapConstructor).
///
/// ```
/// [69] MapConstructor ::= "map" "{" (MapConstructorEntry ("," MapConstructorEntry)*)? "}"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MapConstructor<'a>(pub Vec<MapConstructorEntry<'a>>);

impl MapConstructor<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> MapConstructor<'static> {
        MapConstructor(
            self.0
                .into_iter()
                .map(MapConstructorEntry::into_owned)
                .collect(),
        )
    }
}

impl_deref_mut!(MapConstructor<'a>, Vec<MapConstructorEntry<'a>>);

impl_nom_parser! {
    fn MapConstructor<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        (
            trim0(tag_no_case("map")),
            trim0(tag("{")),
            separated_list0(trim0(tag(",")), MapConstructorEntry::nom),
            trim0(tag("}")),
        )
            .map(|(_, _, entries, _)| Self(entries))
            .parse(rest)
    }
}

impl Display for MapConstructor<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "map{{")?;
        for (i, entry) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", {}", entry)?;
            } else {
                write!(f, "{}", entry)?;
            }
        }
        write!(f, "}}")
    }
}

/// Implementation of [`MapConstructorEntry`](https://www.w3.org/TR/xpath-31/#prod-xpath31-MapConstructorEntry).
///
/// ```
/// [70] MapConstructorEntry ::= MapKeyExpr ":" MapValueExpr
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MapConstructorEntry<'a> {
    pub key: MapKeyExpr<'a>,
    pub value: MapValueExpr<'a>,
}

impl MapConstructorEntry<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> MapConstructorEntry<'static> {
        MapConstructorEntry {
            key: self.key.into_owned(),
            value: self.value.into_owned(),
        }
    }
}

impl_nom_parser! {
    fn MapConstructorEntry<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        (MapKeyExpr::nom, trim0(tag(":")), MapValueExpr::nom)
            .map(|(key, _, value)| Self { key, value })
            .parse(rest)
    }
}

impl Display for MapConstructorEntry<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}: {}", self.key, self.value)
    }
}

/// Implementation of [`MapKeyExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-MapKeyExpr).
///
/// ```
/// [71] MapKeyExpr ::= ExprSingle
/// ```
pub type MapKeyExpr<'a> = ExprSingle<'a>;

/// Implementation of [`MapValueExpr`](https://www.w3.org/TR/xpath-31/#prod-xpath31-MapValueExpr).
///
/// ```
/// [72] MapValueExpr ::= ExprSingle
/// ```
pub type MapValueExpr<'a> = ExprSingle<'a>;

/// Implementation of [`ArrayConstructor`](https://www.w3.org/TR/xpath-31/#prod-xpath31-ArrayConstructor).
///
/// ```
/// [73] ArrayConstructor ::= SquareArrayConstructor | CurlyArrayConstructor
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ArrayConstructor<'a> {
    Square(SquareArrayConstructor<'a>),
    Curly(CurlyArrayConstructor<'a>),
}

impl<'a> ArrayConstructor<'a> {
    #[must_use]
    pub fn items(&self) -> Option<&Vec<ExprSingle<'a>>> {
        match self {
            Self::Curly(x) => x.0 .0.as_ref().map(|x| &x.0),
            Self::Square(x) => Some(&x.0),
        }
    }

    #[must_use]
    pub fn items_mut(&mut self) -> Option<&mut Vec<ExprSingle<'a>>> {
        match self {
            Self::Curly(x) => x.0 .0.as_mut().map(|x| &mut x.0),
            Self::Square(x) => Some(&mut x.0),
        }
    }

    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> ArrayConstructor<'static> {
        match self {
            Self::Square(x) => ArrayConstructor::Square(x.into_owned()),
            Self::Curly(x) => ArrayConstructor::Curly(x.into_owned()),
        }
    }
}

impl_nom_parser! {
    fn ArrayConstructor<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        alt((
            SquareArrayConstructor::nom.map(Self::Square),
            CurlyArrayConstructor::nom.map(Self::Curly),
        ))
        .parse(rest)
    }
}

impl Display for ArrayConstructor<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Square(a) => a.fmt(f),
            Self::Curly(a) => a.fmt(f),
        }
    }
}

/// Implementation of [`SquareArrayConstructor`](https://www.w3.org/TR/xpath-31/#prod-xpath31-SquareArrayConstructor).
///
/// ```
/// [74] SquareArrayConstructor ::= "[" (ExprSingle ("," ExprSingle)*)? "]"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SquareArrayConstructor<'a>(pub Vec<ExprSingle<'a>>);

impl SquareArrayConstructor<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> SquareArrayConstructor<'static> {
        SquareArrayConstructor(self.0.into_iter().map(ExprSingle::into_owned).collect())
    }
}

impl_deref_mut!(SquareArrayConstructor<'a>, Vec<ExprSingle<'a>>);

impl_nom_parser! {
    fn SquareArrayConstructor<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        delimited(
            trim0(tag("[")),
            separated_list0(trim0(tag(",")), ExprSingle::nom),
            trim0(tag("]")),
        )
        .map(Self)
        .parse(rest)
    }
}

impl Display for SquareArrayConstructor<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "[")?;
        for (i, expr) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", {}", expr)?;
            } else {
                write!(f, "{}", expr)?;
            }
        }
        write!(f, "]")
    }
}

/// Implementation of [`CurlyArrayConstructor`](https://www.w3.org/TR/xpath-31/#prod-xpath31-CurlyArrayConstructor).
///
/// ```
/// [75] CurlyArrayConstructor ::= "array" EnclosedExpr
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CurlyArrayConstructor<'a>(pub EnclosedExpr<'a>);

impl CurlyArrayConstructor<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> CurlyArrayConstructor<'static> {
        CurlyArrayConstructor(self.0.into_owned())
    }
}

impl_deref_mut!(CurlyArrayConstructor<'a>, EnclosedExpr<'a>);

impl_nom_parser! {
    fn CurlyArrayConstructor<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        preceded(tag_no_case("array"), EnclosedExpr::nom)
            .map(Self)
            .parse(rest)
    }
}

impl Display for CurlyArrayConstructor<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "array{}", self.0)
    }
}

/// Implementation of [`UnaryLookup`](https://www.w3.org/TR/xpath-31/#prod-xpath31-UnaryLookup).
///
/// ```
/// [76] UnaryLookup ::= "?" KeySpecifier
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnaryLookup<'a>(pub KeySpecifier<'a>);

impl UnaryLookup<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> UnaryLookup<'static> {
        UnaryLookup(self.0.into_owned())
    }
}

impl_deref_mut!(UnaryLookup<'a>, KeySpecifier<'a>);

impl_nom_parser! {
    fn UnaryLookup<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        preceded(tag("?"), KeySpecifier::nom).map(Self).parse(rest)
    }
}

impl Display for UnaryLookup<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "?{}", self.0)
    }
}

/// Implementation of [`SingleType`](https://www.w3.org/TR/xpath-31/#prod-xpath31-SingleType).
///
/// ```
/// [77] SingleType ::= SimpleTypeName "?"?
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SingleType<'a> {
    pub name: SimpleTypeName<'a>,
    pub optional: bool,
}

impl SingleType<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> SingleType<'static> {
        SingleType {
            name: self.name.into_owned(),
            optional: self.optional,
        }
    }
}

impl_nom_parser! {
    fn SingleType<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        (SimpleTypeName::nom, opt(tag("?")))
            .map(|(name, opt)| Self {
                name,
                optional: opt.is_some(),
            })
            .parse(rest)
    }
}

impl Display for SingleType<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.optional {
            write!(f, "{}?", self.name)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

/// Implementation of [`TypeDeclaration`](https://www.w3.org/TR/xpath-31/#prod-xpath31-TypeDeclaration).
///
/// ```
/// [78] TypeDeclaration ::= "as" SequenceType
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypeDeclaration<'a>(pub SequenceType<'a>);

impl TypeDeclaration<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> TypeDeclaration<'static> {
        TypeDeclaration(self.0.into_owned())
    }
}

impl_deref_mut!(TypeDeclaration<'a>, SequenceType<'a>);

impl_nom_parser! {
    fn TypeDeclaration<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        preceded(tag_no_case("as"), SequenceType::nom)
            .map(Self)
            .parse(rest)
    }
}

impl Display for TypeDeclaration<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "as {}", self.0)
    }
}

/// Implementation of [`SequenceType`](https://www.w3.org/TR/xpath-31/#prod-xpath31-SequenceType).
///
/// ```
/// [79] SequenceType ::= ("empty-sequence" "(" ")") | (ItemType OccurrenceIndicator?)
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SequenceType<'a> {
    Empty,
    Typed {
        item_type: ItemType<'a>,
        occurrence: Option<OccurrenceIndicator>,
    },
}

impl SequenceType<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> SequenceType<'static> {
        match self {
            Self::Empty => SequenceType::Empty,
            Self::Typed {
                item_type,
                occurrence,
            } => SequenceType::Typed {
                item_type: item_type.into_owned(),
                occurrence,
            },
        }
    }
}

impl_nom_parser! {
    fn SequenceType<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        alt((
            (
                trim0(tag_no_case("empty-sequence")),
                trim0(tag("(")),
                trim0(tag(")")),
            )
                .map(|_| Self::Empty),
            (ItemType::nom, opt(OccurrenceIndicator::nom)).map(|(item_type, occurrence)| {
                Self::Typed {
                    item_type,
                    occurrence,
                }
            }),
        ))
        .parse(rest)
    }
}

impl Display for SequenceType<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Empty => write!(f, "empty-sequence()"),
            Self::Typed {
                item_type,
                occurrence,
            } => {
                write!(f, "{}", item_type)?;
                if let Some(ind) = occurrence {
                    write!(f, "{}", ind)?;
                }
                Ok(())
            }
        }
    }
}

/// Implementation of [`OccurrenceIndicator`](https://www.w3.org/TR/xpath-31/#prod-xpath31-OccurrenceIndicator).
///
/// ```
/// [80] OccurrenceIndicator ::= "?" | "*" | "+"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OccurrenceIndicator {
    Optional,   // ?
    ZeroOrMore, // *
    OneOrMore,  // +
}

impl_nom_parser! {
    fn OccurrenceIndicator::nom(input: &[u8]) -> IResult<&[u8], Self> {
        let rest = input.trim_ascii();

        alt((
            tag("?").map(|_| Self::Optional),
            tag("*").map(|_| Self::ZeroOrMore),
            tag("+").map(|_| Self::OneOrMore),
        ))
        .parse(rest)
    }
}

impl Display for OccurrenceIndicator {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Optional => write!(f, "?"),
            Self::ZeroOrMore => write!(f, "*"),
            Self::OneOrMore => write!(f, "+"),
        }
    }
}

/// Implementation of [`ItemType`](https://www.w3.org/TR/xpath-31/#prod-xpath31-ItemType).
///
/// ```
/// [81] ItemType ::= KindTest
///                | "item" "(" ")"
///                | FunctionTest
///                | MapTest
///                | ArrayTest
///                | AtomicOrUnionType
///                | ParenthesizedItemType
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ItemType<'a> {
    AnyItem,
    KindTest(Box<KindTest<'a>>),
    Function(Box<FunctionTest<'a>>),
    Map(Box<MapTest<'a>>),
    Array(Box<ArrayTest<'a>>),
    AtomicOrUnion(Box<AtomicOrUnionType<'a>>),
    Parens(Box<ParenthesizedItemType<'a>>),
}

impl ItemType<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> ItemType<'static> {
        match self {
            Self::AnyItem => ItemType::AnyItem,
            Self::KindTest(x) => ItemType::KindTest(Box::new(x.into_owned())),
            Self::Function(x) => ItemType::Function(Box::new(x.into_owned())),
            Self::Map(x) => ItemType::Map(Box::new(x.into_owned())),
            Self::Array(x) => ItemType::Array(Box::new(x.into_owned())),
            Self::AtomicOrUnion(x) => ItemType::AtomicOrUnion(Box::new(x.into_owned())),
            Self::Parens(x) => ItemType::Parens(Box::new(x.into_owned())),
        }
    }
}

impl_nom_parser! {
    fn ItemType<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        alt((
            (trim0(tag_no_case("item")), trim0(tag("(")), trim0(tag(")"))).map(|_| Self::AnyItem),
            KindTest::nom.map(Box::new).map(Self::KindTest),
            FunctionTest::nom.map(Box::new).map(Self::Function),
            MapTest::nom.map(Box::new).map(Self::Map),
            ArrayTest::nom.map(Box::new).map(Self::Array),
            AtomicOrUnionType::nom
                .map(Box::new)
                .map(Self::AtomicOrUnion),
            ParenthesizedItemType::nom.map(Box::new).map(Self::Parens),
        ))
        .parse(rest)
    }
}

impl Display for ItemType<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::AnyItem => write!(f, "item()"),
            Self::KindTest(k) => k.fmt(f),
            Self::Function(ft) => ft.fmt(f),
            Self::Map(mt) => mt.fmt(f),
            Self::Array(at) => at.fmt(f),
            Self::AtomicOrUnion(a) => a.fmt(f),
            Self::Parens(inner) => write!(f, "({})", inner),
        }
    }
}

/// Implementation of [`AtomicOrUnionType`](https://www.w3.org/TR/xpath-31/#prod-xpath31-AtomicOrUnionType).
///
/// ```
/// [82] AtomicOrUnionType ::= EQName
/// ```
pub type AtomicOrUnionType<'a> = EQName<'a>;

/// Implementation of [`KindTest`](https://www.w3.org/TR/xpath-31/#prod-xpath31-KindTest).
///
/// ```
/// [83] KindTest          ::= DocumentTest
///                              | ElementTest
///                              | AttributeTest
///                              | SchemaElementTest
///                              | SchemaAttributeTest
///                              | PITest
///                              | CommentTest
///                              | TextTest
///                              | NamespaceNodeTest
///                              | AnyKindTest
/// [84] AnyKindTest       ::= "node" "(" ")"
/// [86] TextTest          ::= "text" "(" ")"
/// [87] CommentTest       ::= "comment" "(" ")"
/// [88] NamespaceNodeTest ::= "namespace-node" "(" ")"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum KindTest<'a> {
    Document(DocumentTest<'a>),
    Element(ElementTest<'a>),
    Attribute(AttributeTest<'a>),
    SchemaElement(SchemaElementTest<'a>),
    SchemaAttribute(SchemaAttributeTest<'a>),
    PI(PITest<'a>),
    AnyKind,
    NamespaceNode,
    Comment,
    Text,
}

impl KindTest<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> KindTest<'static> {
        match self {
            Self::Document(x) => KindTest::Document(x.into_owned()),
            Self::Element(x) => KindTest::Element(x.into_owned()),
            Self::Attribute(x) => KindTest::Attribute(x.into_owned()),
            Self::SchemaElement(x) => KindTest::SchemaElement(x.into_owned()),
            Self::SchemaAttribute(x) => KindTest::SchemaAttribute(x.into_owned()),
            Self::PI(x) => KindTest::PI(x.into_owned()),
            Self::AnyKind => KindTest::AnyKind,
            Self::NamespaceNode => KindTest::NamespaceNode,
            Self::Comment => KindTest::Comment,
            Self::Text => KindTest::Text,
        }
    }
}

impl_nom_parser! {
    fn KindTest<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        fn test(s: &str) -> impl Parser<&[u8], Output = (), Error = Error<&[u8]>> {
            (
                multispace0,
                tag_no_case(s),
                multispace0,
                tag("("),
                multispace0,
                tag(")"),
                multispace0,
            )
                .map(|_| ())
        }

        let rest = input.trim_ascii();

        alt((
            DocumentTest::nom.map(Self::Document),
            ElementTest::nom.map(Self::Element),
            AttributeTest::nom.map(Self::Attribute),
            SchemaElementTest::nom.map(Self::SchemaElement),
            SchemaAttributeTest::nom.map(Self::SchemaAttribute),
            PITest::nom.map(Self::PI),
            test("namespace-node").map(|()| Self::NamespaceNode),
            test("comment").map(|()| Self::Comment),
            test("text").map(|()| Self::Text),
            test("node").map(|()| Self::AnyKind),
        ))
        .parse(rest)
    }
}

impl Display for KindTest<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Document(k) => k.fmt(f),
            Self::Element(k) => k.fmt(f),
            Self::Attribute(k) => k.fmt(f),
            Self::SchemaElement(k) => k.fmt(f),
            Self::SchemaAttribute(k) => k.fmt(f),
            Self::PI(k) => k.fmt(f),
            Self::NamespaceNode => write!(f, "namespace-node()"),
            Self::AnyKind => write!(f, "node()"),
            Self::Comment => write!(f, "comment()"),
            Self::Text => write!(f, "text()"),
        }
    }
}

/// ```
/// [85] DocumentTest ::= "document-node" "(" (ElementTest | SchemaElementTest)? ")"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DocumentTest<'a>(pub Option<DocumentTestEntry<'a>>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DocumentTestEntry<'a> {
    Element(ElementTest<'a>),
    SchemaElement(SchemaElementTest<'a>),
}

impl DocumentTest<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> DocumentTest<'static> {
        DocumentTest(self.0.map(DocumentTestEntry::into_owned))
    }
}

impl_deref_mut!(DocumentTest<'a>, Option<DocumentTestEntry<'a>>);

impl_nom_parser! {
    fn DocumentTest<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        (
            trim0(tag_no_case("document-node")),
            trim0(tag("(")),
            opt(alt((
                ElementTest::nom.map(DocumentTestEntry::Element),
                SchemaElementTest::nom.map(DocumentTestEntry::SchemaElement),
            ))),
            trim0(tag(")")),
        )
            .map(|(_, _, entry, _)| Self(entry))
            .parse(rest)
    }
}

impl Display for DocumentTest<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "document-node(")?;
        if let Some(entry) = &self.0 {
            write!(f, "{entry}")?;
        }
        write!(f, ")")
    }
}

impl DocumentTestEntry<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> DocumentTestEntry<'static> {
        match self {
            Self::Element(x) => DocumentTestEntry::Element(x.into_owned()),
            Self::SchemaElement(x) => DocumentTestEntry::SchemaElement(x.into_owned()),
        }
    }
}

impl Display for DocumentTestEntry<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Element(e) => e.fmt(f),
            Self::SchemaElement(s) => s.fmt(f),
        }
    }
}

/// Implementation of [`PITest`](https://www.w3.org/TR/xpath-31/#prod-xpath31-PITest).
///
/// ```
/// [89] PITest ::= "processing-instruction" "(" (NCName | StringLiteral)? ")"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PITest<'a> {
    Simple,
    NCName(NCName<'a>),
    String(StringLiteral<'a>),
}

impl PITest<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> PITest<'static> {
        match self {
            Self::Simple => PITest::Simple,
            Self::NCName(x) => PITest::NCName(x.into_owned()),
            Self::String(x) => PITest::String(x.into_owned()),
        }
    }
}

impl_nom_parser! {
    fn PITest<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        delimited(
            (
                trim0(tag_no_case("processing-instruction")),
                trim0(tag("(")),
            ),
            opt(alt((
                NCName::nom.map(Self::NCName),
                StringLiteral::nom.map(Self::String),
            )))
            .map(|o| o.unwrap_or(Self::Simple)),
            trim0(tag(")")),
        )
        .parse(rest)
    }
}

impl Display for PITest<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "processing-instruction(")?;

        match self {
            Self::Simple => (),
            Self::NCName(x) => x.fmt(f)?,
            Self::String(x) => x.fmt(f)?,
        }

        write!(f, ")")?;

        Ok(())
    }
}

/// Implementation of [`AttributeTest`](https://www.w3.org/TR/xpath-31/#prod-xpath31-AttributeTest).
///
/// ```
/// [90] AttributeTest ::= "attribute" "(" (AttribNameOrWildcard ("," TypeName)?)? ")"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AttributeTest<'a> {
    pub name: Option<AttribNameOrWildcard<'a>>,
    pub type_name: Option<TypeName<'a>>,
}

impl AttributeTest<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> AttributeTest<'static> {
        AttributeTest {
            name: self.name.map(AttribNameOrWildcard::into_owned),
            type_name: self.type_name.map(TypeName::into_owned),
        }
    }
}

impl_nom_parser! {
    fn AttributeTest<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        (
            trim0(tag_no_case("attribute")),
            trim0(tag("(")),
            opt((
                AttribNameOrWildcard::nom,
                opt(preceded(tag(","), TypeName::nom)),
            )),
            trim0(tag(")")),
        )
            .map(|(_, _, inner, _)| {
                if let Some((name, type_name)) = inner {
                    Self {
                        name: Some(name),
                        type_name,
                    }
                } else {
                    Self {
                        name: None,
                        type_name: None,
                    }
                }
            })
            .parse(rest)
    }
}

impl Display for AttributeTest<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "attribute(")?;

        if let Some(name) = &self.name {
            name.fmt(f)?;

            if let Some(ty) = &self.type_name {
                write!(f, ", {}", ty)?;
            }
        }

        write!(f, ")")?;

        Ok(())
    }
}

/// Implementation of [`AttribNameOrWildcard`](https://www.w3.org/TR/xpath-31/#prod-xpath31-AttribNameOrWildcard).
///
/// ```
/// [91] AttribNameOrWildcard ::= AttributeName | "*"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AttribNameOrWildcard<'a> {
    Name(AttributeName<'a>),
    Wildcard,
}

impl AttribNameOrWildcard<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> AttribNameOrWildcard<'static> {
        match self {
            Self::Name(x) => AttribNameOrWildcard::Name(x.into_owned()),
            Self::Wildcard => AttribNameOrWildcard::Wildcard,
        }
    }
}

impl_nom_parser! {
    fn AttribNameOrWildcard<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        alt((
            tag("*").map(|_| Self::Wildcard),
            AttributeName::nom.map(Self::Name),
        ))
        .parse(rest)
    }
}

impl Display for AttribNameOrWildcard<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Name(n) => n.fmt(f),
            Self::Wildcard => write!(f, "*"),
        }
    }
}

/// Implementation of [`SchemaAttributeTest`](https://www.w3.org/TR/xpath-31/#prod-xpath31-SchemaAttributeTest).
///
/// ```
/// [92] SchemaAttributeTest ::= "schema-attribute" "(" AttributeDeclaration ")"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SchemaAttributeTest<'a>(pub AttributeDeclaration<'a>);

impl SchemaAttributeTest<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> SchemaAttributeTest<'static> {
        SchemaAttributeTest(self.0.into_owned())
    }
}

impl_deref_mut!(SchemaAttributeTest<'a>, AttributeDeclaration<'a>);

impl_nom_parser! {
    fn SchemaAttributeTest<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        (
            trim0(tag_no_case("schema-attribute")),
            trim0(tag("(")),
            AttributeDeclaration::nom,
            trim0(tag(")")),
        )
            .map(|(_, _, decl, _)| Self(decl))
            .parse(rest)
    }
}

impl Display for SchemaAttributeTest<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "schema-attribute({})", self.0)
    }
}

/// Implementation of [`AttributeDeclaration`](https://www.w3.org/TR/xpath-31/#prod-xpath31-AttributeDeclaration).
///
/// ```
/// [93] AttributeDeclaration ::= AttributeName
/// ```
pub type AttributeDeclaration<'a> = AttributeName<'a>;

/// Implementation of [`ElementTest`](https://www.w3.org/TR/xpath-31/#prod-xpath31-ElementTest).
///
/// ```
/// [94] ElementTest ::= "element" "(" (ElementNameOrWildcard ("," TypeName "?"?)?)? ")"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ElementTest<'a> {
    pub name: Option<ElementNameOrWildcard<'a>>,
    pub type_name: Option<(TypeName<'a>, bool)>, // bool = is_optional
}

impl ElementTest<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> ElementTest<'static> {
        ElementTest {
            name: self.name.map(ElementNameOrWildcard::into_owned),
            type_name: self.type_name.map(|(x, b)| (x.into_owned(), b)),
        }
    }
}

impl_nom_parser! {
    fn ElementTest<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        (
            trim0(tag_no_case("element")),
            trim0(tag("(")),
            opt((
                ElementNameOrWildcard::nom,
                opt((tag(","), TypeName::nom, opt(tag("?")))
                    .map(|(_, t, opt_q)| (t, opt_q.is_some()))),
            )),
            trim0(tag(")")),
        )
            .map(|(_, _, inner, _)| {
                if let Some((name, type_info)) = inner {
                    Self {
                        name: Some(name),
                        type_name: type_info,
                    }
                } else {
                    Self {
                        name: None,
                        type_name: None,
                    }
                }
            })
            .parse(rest)
    }
}

impl Display for ElementTest<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "element(")?;

        if let Some(name) = &self.name {
            name.fmt(f)?;

            if let Some((ty, opt)) = &self.type_name {
                write!(f, ", {}", ty)?;

                if *opt {
                    write!(f, "?")?;
                }
            }
        }

        write!(f, ")")?;

        Ok(())
    }
}

/// Implementation of [`ElementNameOrWildcard`](https://www.w3.org/TR/xpath-31/#prod-xpath31-ElementNameOrWildcard).
///
/// ```
/// [95] ElementNameOrWildcard ::= ElementName | "*"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ElementNameOrWildcard<'a> {
    Name(ElementName<'a>),
    Wildcard,
}

impl ElementNameOrWildcard<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> ElementNameOrWildcard<'static> {
        match self {
            Self::Name(x) => ElementNameOrWildcard::Name(x.into_owned()),
            Self::Wildcard => ElementNameOrWildcard::Wildcard,
        }
    }
}

impl_nom_parser! {
    fn ElementNameOrWildcard<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        alt((
            tag("*").map(|_| Self::Wildcard),
            ElementName::nom.map(Self::Name),
        ))
        .parse(rest)
    }
}

impl Display for ElementNameOrWildcard<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Name(n) => n.fmt(f),
            Self::Wildcard => write!(f, "*"),
        }
    }
}

/// Implementation of [`SchemaElementTest`](https://www.w3.org/TR/xpath-31/#prod-xpath31-SchemaElementTest).
///
/// ```
/// [96] SchemaElementTest ::= "schema-element" "(" ElementDeclaration ")"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SchemaElementTest<'a>(pub ElementDeclaration<'a>);

impl SchemaElementTest<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> SchemaElementTest<'static> {
        SchemaElementTest(self.0.into_owned())
    }
}

impl_deref_mut!(SchemaElementTest<'a>, ElementDeclaration<'a>);

impl_nom_parser! {
    fn SchemaElementTest<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        (
            trim0(tag_no_case("schema-element")),
            trim0(tag("(")),
            ElementDeclaration::nom,
            trim0(tag(")")),
        )
            .map(|(_, _, decl, _)| Self(decl))
            .parse(rest)
    }
}

impl Display for SchemaElementTest<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "schema-element({})", self.0)
    }
}

/// Implementation of [`ElementDeclaration`](https://www.w3.org/TR/xpath-31/#prod-xpath31-ElementDeclaration).
///
/// ```
/// [97] ElementDeclaration ::= ElementName
/// ```
pub type ElementDeclaration<'a> = ElementName<'a>;

/// Implementation of [`AttributeName`](https://www.w3.org/TR/xpath-31/#prod-xpath31-AttributeName).
///
/// ```
/// [98] AttributeName ::= EQName
/// ```
pub type AttributeName<'a> = EQName<'a>;

/// Implementation of [`ElementName`](https://www.w3.org/TR/xpath-31/#prod-xpath31-ElementName).
///
/// ```
/// [99] ElementName ::= EQName
/// ```
pub type ElementName<'a> = EQName<'a>;

/// Implementation of [`SimpleTypeName`](https://www.w3.org/TR/xpath-31/#prod-xpath31-SimpleTypeName).
///
/// ```
/// [100] SimpleTypeName ::= TypeName
/// ```
pub type SimpleTypeName<'a> = TypeName<'a>;

/// Implementation of [`TypeName`](https://www.w3.org/TR/xpath-31/#prod-xpath31-TypeName).
///
/// ```
/// [101] TypeName ::= EQName
/// ```
pub type TypeName<'a> = EQName<'a>;

/// Implementation of [`FunctionTest`](https://www.w3.org/TR/xpath-31/#prod-xpath31-FunctionTest).
///
/// ```
/// [102] FunctionTest    ::= AnyFunctionTest | TypedFunctionTest
/// [103] AnyFunctionTest ::= "function" "(" "*" ")"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FunctionTest<'a> {
    Any,
    Typed(TypedFunctionTest<'a>),
}

impl FunctionTest<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> FunctionTest<'static> {
        match self {
            Self::Any => FunctionTest::Any,
            Self::Typed(x) => FunctionTest::Typed(x.into_owned()),
        }
    }
}

impl_nom_parser! {
    fn FunctionTest<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        alt((
            (
                multispace0,
                tag_no_case("function"),
                multispace0,
                tag("("),
                multispace0,
                tag("*"),
                multispace0,
                tag(")"),
                multispace0,
            )
                .map(|_| Self::Any),
            TypedFunctionTest::nom.map(Self::Typed),
        ))
        .parse(rest)
    }
}

impl Display for FunctionTest<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Any => write!(f, "function(*)"),
            Self::Typed(t) => t.fmt(f),
        }
    }
}

/// Implementation of [`TypedFunctionTest`](https://www.w3.org/TR/xpath-31/#prod-xpath31-TypedFunctionTest).
///
/// ```
/// [104] TypedFunctionTest ::= "function" "(" (SequenceType ("," SequenceType)*)? ")" "as" SequenceType
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypedFunctionTest<'a> {
    pub param_types: Vec<SequenceType<'a>>,
    pub return_type: SequenceType<'a>,
}

impl TypedFunctionTest<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> TypedFunctionTest<'static> {
        TypedFunctionTest {
            param_types: self
                .param_types
                .into_iter()
                .map(SequenceType::into_owned)
                .collect(),
            return_type: self.return_type.into_owned(),
        }
    }
}

impl_nom_parser! {
    fn TypedFunctionTest<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        (
            trim0(tag_no_case("function")),
            trim0(tag("(")),
            separated_list0(trim0(tag(",")), SequenceType::nom),
            trim0(tag(")")),
            trim0(tag_no_case("as")),
            SequenceType::nom,
        )
            .map(|(_, _, param_types, _, _, return_type)| Self {
                param_types,
                return_type,
            })
            .parse(rest)
    }
}

impl Display for TypedFunctionTest<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "function(")?;
        for (i, ty) in self.param_types.iter().enumerate() {
            if i > 0 {
                write!(f, ", {}", ty)?;
            } else {
                write!(f, "{}", ty)?;
            }
        }
        write!(f, ") as {}", self.return_type)
    }
}

/// Implementation of [`MapTest`](https://www.w3.org/TR/xpath-31/#prod-xpath31-MapTest).
///
/// ```
/// [105] MapTest    ::= AnyMapTest | TypedMapTest
/// [106] AnyMapTest ::= "map" "(" "*" ")"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MapTest<'a> {
    Any,
    Typed(TypedMapTest<'a>),
}

impl MapTest<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> MapTest<'static> {
        match self {
            Self::Any => MapTest::Any,
            Self::Typed(x) => MapTest::Typed(x.into_owned()),
        }
    }
}

impl_nom_parser! {
    fn MapTest<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        alt((
            (
                multispace0,
                tag_no_case("map"),
                multispace0,
                tag("("),
                multispace0,
                tag("*"),
                multispace0,
                tag(")"),
                multispace0,
            )
                .map(|_| Self::Any),
            TypedMapTest::nom.map(Self::Typed),
        ))
        .parse(rest)
    }
}

impl Display for MapTest<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Any => write!(f, "map(*)"),
            Self::Typed(t) => t.fmt(f),
        }
    }
}

/// Implementation of [`TypedMapTest`](https://www.w3.org/TR/xpath-31/#prod-xpath31-TypedMapTest).
///
/// ```
/// [107] TypedMapTest ::= "map" "(" AtomicOrUnionType "," SequenceType ")"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypedMapTest<'a> {
    pub key_type: AtomicOrUnionType<'a>,
    pub value_type: SequenceType<'a>,
}

impl TypedMapTest<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> TypedMapTest<'static> {
        TypedMapTest {
            key_type: self.key_type.into_owned(),
            value_type: self.value_type.into_owned(),
        }
    }
}

impl_nom_parser! {
    fn TypedMapTest<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        (
            trim0(tag_no_case("map")),
            trim0(tag("(")),
            AtomicOrUnionType::nom,
            trim0(tag(",")),
            SequenceType::nom,
            trim0(tag(")")),
        )
            .map(|(_, _, key_type, _, value_type, _)| Self {
                key_type,
                value_type,
            })
            .parse(rest)
    }
}

impl Display for TypedMapTest<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "map({}, {})", self.key_type, self.value_type)
    }
}

/// Implementation of [`ArrayTest`](https://www.w3.org/TR/xpath-31/#prod-xpath31-ArrayTest).
///
/// ```
/// [108] ArrayTest    ::= AnyArrayTest | TypedArrayTest
/// [109] AnyArrayTest ::= "array" "(" "*" ")"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ArrayTest<'a> {
    Any,
    Typed(TypedArrayTest<'a>),
}

impl ArrayTest<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> ArrayTest<'static> {
        match self {
            Self::Any => ArrayTest::Any,
            Self::Typed(x) => ArrayTest::Typed(x.into_owned()),
        }
    }
}

impl_nom_parser! {
    fn ArrayTest<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        alt((
            (
                multispace0,
                tag_no_case("array"),
                multispace0,
                tag("("),
                multispace0,
                tag("*"),
                multispace0,
                tag(")"),
                multispace0,
            )
                .map(|_| Self::Any),
            TypedArrayTest::nom.map(Self::Typed),
        ))
        .parse(rest)
    }
}

impl Display for ArrayTest<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Any => write!(f, "array(*)"),
            Self::Typed(t) => t.fmt(f),
        }
    }
}

/// Implementation of [`TypedArrayTest`](https://www.w3.org/TR/xpath-31/#prod-xpath31-TypedArrayTest).
///
/// ```
/// [110] TypedArrayTest ::= "array" "(" SequenceType ")"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypedArrayTest<'a>(pub SequenceType<'a>);

impl TypedArrayTest<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> TypedArrayTest<'static> {
        TypedArrayTest(self.0.into_owned())
    }
}

impl_deref_mut!(TypedArrayTest<'a>, SequenceType<'a>);

impl_nom_parser! {
    fn TypedArrayTest<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        (
            trim0(tag_no_case("array")),
            trim0(tag("(")),
            SequenceType::nom,
            trim0(tag(")")),
        )
            .map(|(_, _, ty, _)| Self(ty))
            .parse(rest)
    }
}

impl Display for TypedArrayTest<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "array({})", self.0)
    }
}

/// Implementation of [`ParenthesizedItemType`](https://www.w3.org/TR/xpath-31/#prod-xpath31-ParenthesizedItemType).
///
/// ```
/// [111] ParenthesizedItemType ::= "(" ItemType ")"
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParenthesizedItemType<'a>(pub ItemType<'a>);

impl ParenthesizedItemType<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> ParenthesizedItemType<'static> {
        ParenthesizedItemType(self.0.into_owned())
    }
}

impl_deref_mut!(ParenthesizedItemType<'a>, ItemType<'a>);

impl_nom_parser! {
    fn ParenthesizedItemType<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        delimited(trim0(tag("(")), ItemType::nom, trim0(tag(")")))
            .map(Self)
            .parse(rest)
    }
}

impl Display for ParenthesizedItemType<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({})", self.0)
    }
}

/// Implementation of [`EQName`](https://www.w3.org/TR/xpath-31/#prod-xpath31-EQName).
///
/// ```
/// [112] EQName ::= QName | URIQualifiedName
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EQName<'a> {
    QName(QName<'a>),
    URIQualifiedName(URIQualifiedName<'a>),
}

impl EQName<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> EQName<'static> {
        match self {
            Self::QName(x) => EQName::QName(x.into_owned()),
            Self::URIQualifiedName(x) => EQName::URIQualifiedName(x.into_owned()),
        }
    }
}

impl_nom_parser! {
    fn EQName<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        alt((
            URIQualifiedName::nom.map(Self::URIQualifiedName),
            QName::nom.map(Self::QName),
        ))
        .parse(rest)
    }
}

impl Display for EQName<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::QName(q) => q.fmt(f),
            Self::URIQualifiedName(u) => u.fmt(f),
        }
    }
}

/// Implementation of [`IntegerLiteral`](https://www.w3.org/TR/xpath-31/#prod-xpath31-IntegerLiteral).
///
/// ```
/// [113] IntegerLiteral ::= Digits
/// [125] Digits         ::= [0-9]+
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IntegerLiteral(pub u128);

impl_deref_mut!(IntegerLiteral, u128);

impl_nom_parser! {
    fn IntegerLiteral::nom(input: &[u8]) -> IResult<&[u8], Self> {
        let rest = input.trim_ascii();

        recognize(digit1)
            .map_res(|b: &[u8]| {
                let s = from_utf8(b).map_err(|_| ())?;
                let v = s.parse::<u128>().map_err(|_| ())?;

                Ok::<Self, ()>(Self(v))
            })
            .parse(rest)
    }
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

/// Implementation of [`DecimalLiteral`](https://www.w3.org/TR/xpath-31/#prod-xpath31-DecimalLiteral).
///
/// ```
/// [114] DecimalLiteral ::= ("." Digits) | (Digits "." [0-9]*)
/// [125] Digits         ::= [0-9]+
/// ```
#[derive(Debug, Clone)]
pub struct DecimalLiteral(pub f64);

impl_deref_mut!(DecimalLiteral, f64);

impl_nom_parser! {
    fn DecimalLiteral::nom(input: &[u8]) -> IResult<&[u8], Self> {
        let rest = input.trim_ascii();

        recognize(alt((
            (tag("."), digit1).map(|_| ()),
            (digit1, tag("."), digit0).map(|_| ()),
        )))
        .map_res(|b: &[u8]| {
            let s = from_utf8(b).map_err(|_| ())?;
            let v = s.parse::<f64>().map_err(|_| ())?;

            Ok::<Self, ()>(Self(v))
        })
        .parse(rest)
    }
}

impl Eq for DecimalLiteral {}

impl PartialEq for DecimalLiteral {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Display for DecimalLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:}", self.0)
    }
}

/// Implementation of [`DoubleLiteral`](https://www.w3.org/TR/xpath-31/#prod-xpath31-DoubleLiteral).
///
/// ```
/// [115] DoubleLiteral ::= (("." Digits) | (Digits ("." [0-9]*)?)) [eE] [+-]? Digits
/// [125] Digits        ::= [0-9]+
/// ```
#[derive(Debug, Clone)]
pub struct DoubleLiteral(pub f64);

impl_deref_mut!(DoubleLiteral, f64);

impl_nom_parser! {
    fn DoubleLiteral::nom(input: &[u8]) -> IResult<&[u8], Self> {
        let rest = input.trim_ascii();

        recognize((
            alt((
                (tag("."), digit1).map(|_| ()),
                (digit1, opt((tag("."), digit0))).map(|_| ()),
            )),
            alt((tag("e"), tag("E"))),
            opt(alt((tag("+"), tag("-")))),
            digit1,
        ))
        .map_res(|b: &[u8]| {
            let s = from_utf8(b).map_err(|_| ())?;
            let v = s.parse::<f64>().map_err(|_| ())?;

            Ok::<Self, ()>(Self(v))
        })
        .parse(rest)
    }
}

impl Eq for DoubleLiteral {}

impl PartialEq for DoubleLiteral {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Display for DoubleLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:e}", self.0)
    }
}

/// Implementation of [`StringLiteral`](https://www.w3.org/TR/xpath-31/#prod-xpath31-StringLiteral).
///
/// ```
/// [116] StringLiteral ::= ('"' (EscapeQuot | [^"])* '"') | ("'" (EscapeApos | [^'])* "'")
/// [119] EscapeQuot ::= '""'
/// [120] EscapeApos ::= "''"
/// ```
#[derive(Clone, Eq, PartialEq)]
pub struct StringLiteral<'a>(Cow<'a, [u8]>, char);

impl StringLiteral<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> StringLiteral<'static> {
        StringLiteral(Cow::Owned(self.0.into_owned()), self.1)
    }
}

impl_deref!(StringLiteral<'a>, Cow<'a, [u8]>);

impl_nom_parser! {
    fn StringLiteral<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        fn not_quote(quote: u8) -> impl Fn(u8) -> bool {
            move |c| c != quote
        }

        let rest = input.trim_ascii();

        alt((
            delimited(tag("\""), escaped(take_while(not_quote(b'"')), '"', one_of("\"")), tag("\""))
                .map(|b: &[u8]| Self(Cow::Borrowed(b), '"')),
            delimited(tag("'"), escaped(take_while(not_quote(b'\'')), '\'', one_of("'")), tag("'"))
                .map(|b: &[u8]| Self(Cow::Borrowed(b), '\'')),
        ))
        .parse(rest)
    }
}

impl Display for StringLiteral<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self(x, s) = self;

        write!(f, "{s}")?;
        format_utf8_slice(x, f)?;
        write!(f, "{s}")?;

        Ok(())
    }
}

impl std::fmt::Debug for StringLiteral<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "StringLiteral(\"")?;
        format_utf8_slice(&self.0, f)?;
        write!(f, "\")")?;

        Ok(())
    }
}

/// Implementation of [`URIQualifiedName`](https://www.w3.org/TR/xpath-31/#prod-xpath31-URIQualifiedName).
///
/// ```
/// [117] URIQualifiedName ::= BracedURILiteral NCName
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct URIQualifiedName<'a> {
    pub namespace_uri: BracedURILiteral<'a>,
    pub local_name: NCName<'a>,
}

impl URIQualifiedName<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> URIQualifiedName<'static> {
        URIQualifiedName {
            namespace_uri: self.namespace_uri.into_owned(),
            local_name: self.local_name.into_owned(),
        }
    }
}

impl_nom_parser! {
    fn URIQualifiedName<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        (BracedURILiteral::nom, NCName::nom)
            .map(|(namespace_uri, local_name)| Self {
                namespace_uri,
                local_name,
            })
            .parse(rest)
    }
}

impl Display for URIQualifiedName<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self {
            namespace_uri,
            local_name,
        } = self;

        write!(f, "{namespace_uri}{local_name}")
    }
}

/// Implementation of [`BracedURILiteral`](https://www.w3.org/TR/xpath-31/#prod-xpath31-BracedURILiteral).
///
/// ```
/// [118] BracedURILiteral ::= "Q" "{" [^{}]* "}"
/// ```
#[derive(Clone, Eq, PartialEq)]
pub struct BracedURILiteral<'a>(pub Cow<'a, [u8]>);

impl BracedURILiteral<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> BracedURILiteral<'static> {
        BracedURILiteral(Cow::Owned(self.0.into_owned()))
    }
}

impl_deref_mut!(BracedURILiteral<'a>, Cow<'a, [u8]>);

impl_nom_parser! {
    fn BracedURILiteral<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        delimited(trim0(tag("Q{")), take_until("}"), trim0(tag("}")))
            .map(|b: &[u8]| Self(Cow::Borrowed(b.trim_ascii())))
            .parse(rest)
    }
}

impl Display for BracedURILiteral<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Q{{")?;
        format_utf8_slice(&self.0, f)?;
        write!(f, "}}")?;

        Ok(())
    }
}

impl std::fmt::Debug for BracedURILiteral<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "BracedURILiteral(\"")?;
        format_utf8_slice(&self.0, f)?;
        write!(f, "\")")?;

        Ok(())
    }
}

/// Implementation of [`Comment`](https://www.w3.org/TR/xpath-31/#prod-xpath31-Comment).
///
/// ```
/// [121] Comment         ::= "(:" (CommentContents | Comment)* ":)"
/// [124] Char            ::= [http://www.w3.org/TR/REC-xml#NT-Char]XML
/// [126] CommentContents ::= (Char+ - (Char* ('(:' | ':)') Char*))
/// ```
#[derive(Clone, Eq, PartialEq)]
pub struct Comment<'a>(pub Cow<'a, [u8]>);

impl Comment<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> Comment<'static> {
        Comment(Cow::Owned(self.0.into_owned()))
    }
}

impl_deref_mut!(Comment<'a>, Cow<'a, [u8]>);

impl_nom_parser! {
    fn Comment<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        recognize(delimited(tag("(:"), take_until(":)"), tag(":)")))
            .map(|s: &[u8]| Self(Cow::Borrowed(s)))
            .parse(rest)
    }
}

impl Display for Comment<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "(:")?;
        format_utf8_slice(&self.0, f)?;
        write!(f, ":)")?;

        Ok(())
    }
}

impl std::fmt::Debug for Comment<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Comment(\"")?;
        format_utf8_slice(&self.0, f)?;
        write!(f, "\")")?;

        Ok(())
    }
}

/// Implementation of [`QName`](https://www.w3.org/TR/xpath-31/#prod-xpath31-QName).
///
/// ```
/// [122] QName ::= Names (per [XML Namespaces spec](http://www.w3.org/TR/REC-xml-names/#NT-QName))
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QName<'a> {
    pub prefix: Option<NCName<'a>>,
    pub local: NCName<'a>,
}

impl QName<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> QName<'static> {
        QName {
            prefix: self.prefix.map(NCName::into_owned),
            local: self.local.into_owned(),
        }
    }
}

impl_nom_parser! {
    fn QName<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let rest = input.trim_ascii();

        let (rest, first) = NCName::nom(rest)?;
        let (rest, second) = opt(preceded(tag(":"), NCName::nom)).parse(rest)?;

        let ret = if let Some(local) = second {
            Self {
                prefix: Some(first),
                local,
            }
        } else {
            Self {
                prefix: None,
                local: first,
            }
        };

        Ok((rest, ret))
    }
}

impl Display for QName<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { prefix, local } = self;

        match prefix {
            Some(prefix) => write!(f, "{prefix}:{local}"),
            None => local.fmt(f),
        }
    }
}

/// Implementation of [`NCName`](https://www.w3.org/TR/xpath-31/#prod-xpath31-NCName).
///
/// ```
/// [123] NCName ::= Names  (per [XML Namespaces spec](http://www.w3.org/TR/REC-xml-names/#NT-NCName))
/// ```
#[derive(Clone, Eq, PartialEq)]
pub struct NCName<'a>(pub Cow<'a, [u8]>);

impl NCName<'_> {
    /// Converts the object to an owned version of the same type with a `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> NCName<'static> {
        NCName(Cow::Owned(self.0.into_owned()))
    }
}

impl_deref_mut!(NCName<'a>, Cow<'a, [u8]>);

impl_nom_parser! {
    fn NCName<'a>::nom(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        fn is_ncname_start(b: u8) -> bool {
            b.is_ascii_alphabetic() || b == b'_'
        }

        fn is_ncname_char(b: u8) -> bool {
            is_ncname_start(b) || b.is_ascii_digit() || b == b'-' || b == b'.'
        }

        let rest = input.trim_ascii();

        recognize((take_while1(is_ncname_start), take_while(is_ncname_char)))
            .map(|b| Self(Cow::Borrowed(b)))
            .parse(rest)
    }
}

impl Display for NCName<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        format_utf8_slice(&self.0, f)
    }
}

impl std::fmt::Debug for NCName<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "NCName(\"")?;
        format_utf8_slice(&self.0, f)?;
        write!(f, "\")")?;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum FromStrError {
    #[error("Unable to parse xpath type (expr={expr}, code={code:?})")]
    Error {
        expr: String,
        code: ErrorKind,
        rest: Option<String>,
    },
    #[error("Parsing of xpath type was incomplete (expr={expr}, rest={rest:?})")]
    Incomplete { expr: String, rest: Option<String> },
}

fn trim0<I, E, F>(parser: F) -> impl Parser<I, Output = F::Output, Error = E>
where
    I: Input,
    I::Item: AsChar,
    E: ParseError<I>,
    F: Parser<I, Error = E>,
{
    delimited(multispace0, parser, multispace0)
}

#[cfg(feature = "xpath-debug")]
mod debug {
    use std::cell::RefCell;
    use std::fmt::{Debug, Display, Formatter, Result as FmtResult, Write};

    use nom::{Err, IResult};

    #[allow(unused_must_use, clippy::needless_raw_string_hashes)]
    pub(super) fn enter(ty: &str, input: &[u8]) {
        DEBUG_CONTEXT.with_borrow_mut(|ctx| {
            let s = &mut ctx.output;
            let indent = "    ".repeat(2 * ctx.stack.len());

            if let Some(first) = ctx.stack.last_mut() {
                if !std::mem::take(first) {
                    write!(s, ",");
                }
            }

            writeln!(s, "\n{indent}{{");
            writeln!(s, r#"{indent}    "name": "{ty}","#);
            writeln!(s, r#"{indent}    "input": {},"#, JsonStr(input));
            write!(s, r#"{indent}    "items": ["#);

            ctx.stack.push(true);
        });
    }

    #[allow(unused_must_use, clippy::needless_raw_string_hashes)]
    pub(super) fn leave<T: Debug>(ret: IResult<&[u8], T>) -> IResult<&[u8], T> {
        use std::fmt::Write;

        DEBUG_CONTEXT.with_borrow_mut(|ctx| {
            ctx.stack.pop();
            let indent = "    ".repeat(2 * ctx.stack.len());

            writeln!(ctx.output, "\n{indent}    ],");

            let s = &mut ctx.output;

            match &ret {
                Ok((rest, value)) => {
                    writeln!(s, r#"{indent}    "ok": {{"#);
                    writeln!(s, r#"{indent}        "rest": {},"#, JsonStr(rest));
                    writeln!(
                        s,
                        r#"{indent}        "value": {}"#,
                        JsonStr(format!("{value:?}").as_bytes())
                    );
                    writeln!(s, r#"{indent}    }}"#);
                    write!(s, r#"{indent}}}"#);
                }
                Err(Err::Error(error) | Err::Failure(error)) => {
                    writeln!(s, r#"{indent}    "error": {{"#);
                    writeln!(s, r#"{indent}        "rest": {},"#, JsonStr(error.input));
                    writeln!(s, r#"{indent}        "code": "{:?}""#, error.code);
                    writeln!(s, r#"{indent}    }}"#);
                    write!(s, r#"{indent}}}"#);
                }
                Err(Err::Incomplete(_)) => unreachable!(),
            }
        });

        ret
    }

    #[cfg(test)]
    pub(super) fn extract() -> String {
        DEBUG_CONTEXT.with_borrow_mut(|ctx| {
            ctx.stack.clear();

            std::mem::take(&mut ctx.output)
        })
    }

    #[derive(Default, Debug)]
    struct DebugContext {
        output: String,
        stack: Vec<bool>,
    }

    struct JsonStr<'a>(&'a [u8]);

    thread_local! {
        static DEBUG_CONTEXT: RefCell<DebugContext> = RefCell::default();
    }

    impl Display for JsonStr<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_char('"')?;

            for &b in self.0 {
                match b {
                    b'"' => f.write_str("\\\"")?,
                    b'\\' => f.write_str("\\\\")?,
                    b'\n' => f.write_str("\\n")?,
                    b'\r' => f.write_str("\\r")?,
                    b'\t' => f.write_str("\\t")?,
                    b'\x08' => f.write_str("\\b")?,
                    b'\x0C' => f.write_str("\\f")?,
                    0x00..=0x1F => write!(f, "\\u{:04x}", b)?,
                    _ => f.write_char(b as char)?,
                }
            }

            f.write_char('"')?;

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use nom::{error::Error, Err};

    use crate::models::RawByteStr;

    use super::XPath;

    #[test]
    #[rustfmt::skip]
    fn parse_selected() {
        check("count(ram:ActualAmount)=1");
        check("count(//ram:ApplicableHeaderTradeSettlement/ram:ApplicableTradeTax[ram:CategoryCode='E'])=0");
        check("count(//ram:SpecifiedLineTradeSettlement/ram:ApplicableTradeTax[ram:CategoryCode='E'])=0");
        check("count(//ram:CategoryTradeTax[ram:CategoryCode='E'])=0");
        check("count(//ram:ApplicableHeaderTradeSettlement/ram:ApplicableTradeTax[ram:CategoryCode='E'])=1");
        check("exists(//ram:SpecifiedLineTradeSettlement/ram:ApplicableTradeTax[ram:CategoryCode='E'])");
        check("exists(//ram:CategoryTradeTax[ram:CategoryCode='E'])");
        check(r"
               (    count(//ram:ApplicableHeaderTradeSettlement/ram:ApplicableTradeTax[ram:CategoryCode='E'])=0
                and count(//ram:SpecifiedLineTradeSettlement/ram:ApplicableTradeTax[ram:CategoryCode='E'])=0
                and count(//ram:CategoryTradeTax[ram:CategoryCode='E'])=0 )
            or (    count(//ram:ApplicableHeaderTradeSettlement/ram:ApplicableTradeTax[ram:CategoryCode='E'])=1
                and (   exists(//ram:SpecifiedLineTradeSettlement/ram:ApplicableTradeTax[ram:CategoryCode='E'])
                    or  exists(//ram:CategoryTradeTax[ram:CategoryCode='E'])))");
    }

    #[test]
    fn parse_many() {
        let xpaths = include_str!("xpaths");
        let xpaths = xpaths.split('\n').filter(|s| {
            let s = s.trim();

            !s.starts_with('#') && !s.is_empty()
        });

        for xpath in xpaths {
            check(xpath);
        }
    }

    fn check(escaped: &str) {
        check_with(|x| XPath::nom(x).map(|(rest, _)| rest), escaped);
    }

    fn check_with<F>(f: F, escaped: &str)
    where
        F: for<'a> FnOnce(&'a [u8]) -> Result<&'a [u8], Err<Error<&'a [u8]>>>,
    {
        #[cfg(feature = "xpath-debug")]
        super::debug::extract();

        let unescaped = quick_xml::escape::unescape(escaped).unwrap();
        let res = f(unescaped.as_bytes());

        #[cfg(feature = "xpath-debug")]
        let output = super::debug::extract();

        let ret = match res {
            Ok(rest) if rest.trim_ascii().is_empty() => return,
            Ok(rest) => Ok(rest),
            Err(Err::Incomplete(_)) => panic!("incomplete"),
            Err(Err::Error(error) | Err::Failure(error)) => Err(error),
        };

        println!("Escaped Test String: {escaped}");
        println!("Unescaped Test String: {unescaped}");

        #[cfg(feature = "xpath-debug")]
        {
            println!();
            println!("{output}");
        }

        match ret {
            Ok(rest) => panic!("Rest is not empty: {}", RawByteStr::from_slice(rest)),
            Err(error) => {
                let Error { code, input } = error;

                let s = RawByteStr::from_slice(input);

                panic!("Error:\n    code = {code:?}\n    input = {s}\n");
            }
        }
    }
}
