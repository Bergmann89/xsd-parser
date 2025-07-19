// https://www.w3.org/TR/xpath-31/#prod-xpath31-XPath

#![allow(missing_docs, unused_variables, clippy::missing_errors_doc)] // TODO

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::opt,
    error::{Error, ErrorKind, ParseError},
    multi::{many0, separated_list1},
    sequence::{delimited, preceded},
    AsChar, Err, IResult, Input, Parser,
};

#[derive(Debug)]
pub struct XPath(pub Vec<ExprSingle>);

impl XPath {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [1] XPath ::= Expr
        // [6] Expr  ::= ExprSingle ("," ExprSingle)*

        trim0(separated_list1(trim0(tag(",")), ExprSingle::nom))
            .map(Self)
            .parse(input)
    }
}

#[derive(Debug)]
pub enum ExprSingle {
    For(Box<ForExpr>),
    Let(LetExpr),
    Quantified(QuantifiedExpr),
    If(IfExpr),
    Or(OrExpr),
}

impl ExprSingle {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [7] ExprSingle ::= ForExpr | LetExpr | QuantifiedExpr | IfExpr | OrExpr

        alt((
            ForExpr::nom.map(Box::new).map(Self::For),
            LetExpr::nom.map(Self::Let),
            QuantifiedExpr::nom.map(Self::Quantified),
            IfExpr::nom.map(Self::If),
            OrExpr::nom.map(Self::Or),
        ))
        .parse(input)
    }
}

#[derive(Debug)]
pub struct ForExpr {
    pub clause: SimpleForClause,
    pub expr: ExprSingle,
}

pub type SimpleForClause = Vec<SimpleForBinding>;
pub type SimpleForBinding = (VarName, ExprSingle);

impl ForExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [8]  ForExpr          ::= SimpleForClause "return" ExprSingle
        // [9]  SimpleForClause  ::= "for" SimpleForBinding ("," SimpleForBinding)*
        // [10] SimpleForBinding ::= "$" VarName "in" ExprSingle

        fn simple_for_clause(input: &[u8]) -> IResult<&[u8], SimpleForClause> {
            preceded(
                (multispace0, tag("for"), multispace0),
                separated_list1((multispace0, tag(","), multispace0), simple_for_binding),
            )
            .parse(input)
        }

        fn simple_for_binding(input: &[u8]) -> IResult<&[u8], SimpleForBinding> {
            (
                multispace0,
                tag("$"),
                VarName::nom,
                multispace1,
                tag("in"),
                multispace1,
                ExprSingle::nom,
            )
                .map(|(_, _, name, _, _, _, expr)| (name, expr))
                .parse(input)
        }

        (
            simple_for_clause,
            multispace0,
            tag("return"),
            multispace0,
            ExprSingle::nom,
        )
            .map(|(clause, _, _, _, expr)| Self { clause, expr })
            .parse(input)
    }
}

#[derive(Debug)]
pub struct LetExpr;

impl LetExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [11] LetExpr ::= SimpleLetClause "return" ExprSingle

        unimplemented!()
    }
}

#[derive(Debug)]
pub struct QuantifiedExpr;

impl QuantifiedExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [14] QuantifiedExpr ::= ("some" | "every") "$" VarName "in" ExprSingle ("," "$" VarName "in" ExprSingle)* "satisfies" ExprSingle

        unimplemented!()
    }
}

#[derive(Debug)]
pub struct IfExpr;

impl IfExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [15] IfExpr ::= "if" "(" Expr ")" "then" ExprSingle "else" ExprSingle

        unimplemented!()
    }
}

#[derive(Debug)]
pub struct OrExpr(pub Vec<AndExpr>);

impl OrExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [16] OrExpr ::= AndExpr ( "or" AndExpr )*

        separated_list1(trim0(tag("or")), AndExpr::nom)
            .map(Self)
            .parse(input)
    }
}

#[derive(Debug)]
pub struct AndExpr(pub Vec<ComparisonExpr>);

impl AndExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [17] AndExpr ::= ComparisonExpr ( "and" ComparisonExpr )*

        separated_list1(trim0(tag("and")), ComparisonExpr::nom)
            .map(Self)
            .parse(input)
    }
}

#[derive(Debug)]
pub enum ComparisonExpr {
    Unary(StringConcatExpr),
    Equal(StringConcatExpr, StringConcatExpr),
    NotEqual(StringConcatExpr, StringConcatExpr),
    LowerThan(StringConcatExpr, StringConcatExpr),
    LowerEqual(StringConcatExpr, StringConcatExpr),
    GreaterThan(StringConcatExpr, StringConcatExpr),
    GreaterEqual(StringConcatExpr, StringConcatExpr),
    Is(StringConcatExpr, StringConcatExpr),
    Precedes(StringConcatExpr, StringConcatExpr),
    Follows(StringConcatExpr, StringConcatExpr),
}

impl ComparisonExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [18] ComparisonExpr ::= StringConcatExpr ( ( ValueComp | GeneralComp | NodeComp ) StringConcatExpr )?
        // [32] GeneralComp    ::= "=" | "!=" | "<" | "<=" | ">" | ">="
        // [33] ValueComp      ::= "eq" | "ne" | "lt" | "le" | "gt" | "ge"
        // [34] NodeComp       ::= "is" | "<<" | ">>"

        let (input, first) = StringConcatExpr::nom(input)?;
        let (input, second) = opt(alt((
            (trim0(tag("=")), StringConcatExpr::nom),
            (trim0(tag("!=")), StringConcatExpr::nom),
            (trim0(tag("<")), StringConcatExpr::nom),
            (trim0(tag("<=")), StringConcatExpr::nom),
            (trim0(tag(">")), StringConcatExpr::nom),
            (trim0(tag(">=")), StringConcatExpr::nom),
            (trim0(tag("eq")), StringConcatExpr::nom),
            (trim0(tag("ne")), StringConcatExpr::nom),
            (trim0(tag("lt")), StringConcatExpr::nom),
            (trim0(tag("le")), StringConcatExpr::nom),
            (trim0(tag("gt")), StringConcatExpr::nom),
            (trim0(tag("ge")), StringConcatExpr::nom),
            (trim0(tag("is")), StringConcatExpr::nom),
            (trim0(tag("<<")), StringConcatExpr::nom),
            (trim0(tag(">>")), StringConcatExpr::nom),
        )))
        .parse(input)?;

        let ret = match second {
            None => Self::Unary(first),
            Some((b"=" | b"eq", second)) => Self::Equal(first, second),
            Some((b"!=" | b"ne", second)) => Self::NotEqual(first, second),
            Some((b"<" | b"lt", second)) => Self::LowerThan(first, second),
            Some((b"<=" | b"le", second)) => Self::LowerEqual(first, second),
            Some((b">" | b"gt", second)) => Self::GreaterThan(first, second),
            Some((b">=" | b"ge", second)) => Self::GreaterEqual(first, second),
            Some((b"is", second)) => Self::Is(first, second),
            Some((b"<<", second)) => Self::Precedes(first, second),
            Some((b">>", second)) => Self::Precedes(first, second),
            Some((_, _)) => Err(Err::Failure(Error::new(input, ErrorKind::Tag)))?,
        };

        Ok((input, ret))
    }
}

#[derive(Debug)]
pub struct StringConcatExpr(pub Vec<RangeExpr>);

impl StringConcatExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [19] StringConcatExpr ::= RangeExpr ( "||" RangeExpr )*

        separated_list1(trim0(tag("||")), RangeExpr::nom)
            .map(Self)
            .parse(input)
    }
}

#[derive(Debug)]
pub enum RangeExpr {
    Single(AdditiveExpr),
    FromTo(AdditiveExpr, AdditiveExpr),
}

impl RangeExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [20] RangeExpr ::= AdditiveExpr ( "to" AdditiveExpr )?

        let (input, first) = AdditiveExpr::nom(input)?;
        let (input, second) = opt(preceded(trim0(tag("to")), AdditiveExpr::nom)).parse(input)?;

        let ret = if let Some(second) = second {
            Self::FromTo(first, second)
        } else {
            Self::Single(first)
        };

        Ok((input, ret))
    }
}

#[derive(Debug)]
pub enum AdditiveExpr {
    Single(MultiplicativeExpr),
    Plus(MultiplicativeExpr, MultiplicativeExpr),
    Minus(MultiplicativeExpr, MultiplicativeExpr),
}

impl AdditiveExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [21] AdditiveExpr ::= MultiplicativeExpr ( ( "+" | "-" ) MultiplicativeExpr )*

        let (input, first) = MultiplicativeExpr::nom(input)?;
        let (input, second) = opt(alt((
            (trim0(tag("+")), MultiplicativeExpr::nom),
            (trim0(tag("-")), MultiplicativeExpr::nom),
        )))
        .parse(input)?;

        let ret = match second {
            None => Self::Single(first),
            Some((b"+", second)) => Self::Plus(first, second),
            Some((b"-", second)) => Self::Minus(first, second),
            Some((_, _)) => Err(Err::Failure(Error::new(input, ErrorKind::Tag)))?,
        };

        Ok((input, ret))
    }
}

#[derive(Debug)]
pub enum MultiplicativeExpr {
    Single(UnionExpr),
    Mul(UnionExpr, UnionExpr),
    Div(UnionExpr, UnionExpr),
    IDiv(UnionExpr, UnionExpr),
    Mod(UnionExpr, UnionExpr),
}

impl MultiplicativeExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [22] MultiplicativeExpr ::= UnionExpr ( ( "*" | "div" | "idiv" | "mod" ) UnionExpr )*

        let (input, first) = UnionExpr::nom(input)?;
        let (input, second) = opt(alt((
            (trim0(tag("*")), UnionExpr::nom),
            (trim0(tag("div")), UnionExpr::nom),
            (trim0(tag("idiv")), UnionExpr::nom),
            (trim0(tag("mod")), UnionExpr::nom),
        )))
        .parse(input)?;

        let ret = match second {
            None => Self::Single(first),
            Some((b"*", second)) => Self::Mul(first, second),
            Some((b"div", second)) => Self::Div(first, second),
            Some((b"idiv", second)) => Self::IDiv(first, second),
            Some((b"Mod", second)) => Self::Mod(first, second),
            Some((_, _)) => Err(Err::Failure(Error::new(input, ErrorKind::Tag)))?,
        };

        Ok((input, ret))
    }
}

#[derive(Debug)]
pub struct UnionExpr(pub Vec<IntersectExceptExpr>);

impl UnionExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [23] UnionExpr ::= IntersectExceptExpr ( ( "union" | "|" ) IntersectExceptExpr )*

        trim0(separated_list1(
            trim0(alt((tag("union"), tag("|")))),
            IntersectExceptExpr::nom,
        ))
        .map(Self)
        .parse(input)
    }
}

#[derive(Debug)]
pub struct IntersectExceptExpr {
    pub head: InstanceofExpr,
    pub list: Vec<IntersectExceptExprOp>,
}

#[derive(Debug)]
pub enum IntersectExceptExprOp {
    Except(InstanceofExpr),
    Intersect(InstanceofExpr),
}

impl IntersectExceptExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [24] IntersectExceptExpr ::= InstanceofExpr ( ( "intersect" | "except" ) InstanceofExpr )*

        let (input, head) = InstanceofExpr::nom(input)?;
        let (input, list) = many0(alt((
            preceded(trim0(tag("except")), InstanceofExpr::nom).map(IntersectExceptExprOp::Except),
            preceded(trim0(tag("intersect")), InstanceofExpr::nom)
                .map(IntersectExceptExprOp::Intersect),
        )))
        .parse(input)?;

        let ret = Self { head, list };

        Ok((input, ret))
    }
}

#[derive(Debug)]
pub enum InstanceofExpr {
    Single(TreatExpr),
    InstanceOf(TreatExpr, SequenceType),
}

impl InstanceofExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [25] InstanceofExpr ::= TreatExpr ( "instance" "of" SequenceType )?

        let (input, first) = TreatExpr::nom(input)?;
        let (input, second) = opt((
            multispace0,
            tag("instance"),
            multispace1,
            tag("of"),
            multispace1,
            SequenceType::nom,
        ))
        .parse(input)?;

        let ret = if let Some((_, _, _, _, _, second)) = second {
            Self::InstanceOf(first, second)
        } else {
            Self::Single(first)
        };

        Ok((input, ret))
    }
}

#[derive(Debug)]
pub enum TreatExpr {
    Single(CastableExpr),
    TreatAs(CastableExpr, SequenceType),
}

impl TreatExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [26] TreatExpr ::= CastableExpr ( "treat" "as" SequenceType )?

        let (input, first) = CastableExpr::nom(input)?;
        let (input, second) = opt((
            multispace0,
            tag("treat"),
            multispace1,
            tag("as"),
            multispace1,
            SequenceType::nom,
        ))
        .parse(input)?;

        let ret = if let Some((_, _, _, _, _, second)) = second {
            Self::TreatAs(first, second)
        } else {
            Self::Single(first)
        };

        Ok((input, ret))
    }
}

#[derive(Debug)]
pub enum CastableExpr {
    Single(CastExpr),
    CastableAs(CastExpr, SingleType),
}

impl CastableExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [27] CastableExpr ::= CastExpr ( "castable" "as" SingleType )?

        let (input, first) = CastExpr::nom(input)?;
        let (input, second) = opt((
            multispace0,
            tag("castable"),
            multispace1,
            tag("as"),
            multispace1,
            SingleType::nom,
        ))
        .parse(input)?;

        let ret = if let Some((_, _, _, _, _, second)) = second {
            Self::CastableAs(first, second)
        } else {
            Self::Single(first)
        };

        Ok((input, ret))
    }
}

#[derive(Debug)]
pub enum CastExpr {
    Single(ArrowExpr),
    CastAs(ArrowExpr, SingleType),
}

impl CastExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [28] CastExpr ::= ArrowExpr ( "cast" "as" SingleType )?

        let (input, first) = ArrowExpr::nom(input)?;
        let (input, second) = opt((
            multispace0,
            tag("cast"),
            multispace1,
            tag("as"),
            multispace1,
            SingleType::nom,
        ))
        .parse(input)?;

        let ret = if let Some((_, _, _, _, _, second)) = second {
            Self::CastAs(first, second)
        } else {
            Self::Single(first)
        };

        Ok((input, ret))
    }
}

#[derive(Debug)]
pub struct ArrowExpr {
    pub expr: UnaryExpr,
    pub functions: Vec<(ArrowFunctionSpecifier, ArgumentList)>,
}

impl ArrowExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [29] ArrowExpr ::= UnaryExpr ( "=>" ArrowFunctionSpecifier ArgumentList )*

        let (input, expr) = UnaryExpr::nom(input)?;
        let (input, functions) = many0(preceded(
            trim0(tag("=>")),
            (ArrowFunctionSpecifier::nom, trim0(ArgumentList::nom)),
        ))
        .parse(input)?;

        let ret = Self { expr, functions };

        Ok((input, ret))
    }
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub signs: Vec<UnaryExprSign>,
    pub value: ValueExpr,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryExprSign {
    Plus,
    Minus,
}

impl UnaryExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [30] UnaryExpr ::= ("-" | "+")* ValueExpr

        let (input, signs) = many0(alt((
            tag("+").map(|_| UnaryExprSign::Plus),
            tag("-").map(|_| UnaryExprSign::Minus),
        )))
        .parse(input)?;
        let (input, value) = ValueExpr::nom(input)?;

        let ret = Self { signs, value };

        Ok((input, ret))
    }
}

#[derive(Debug)]
pub struct ValueExpr(pub Vec<PathExpr>);

impl ValueExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [31] ValueExpr     ::= SimpleMapExpr
        // [35] SimpleMapExpr ::= PathExpr ("!" PathExpr)*

        separated_list1(trim0(tag("!")), PathExpr::nom)
            .map(Self)
            .parse(input)
    }
}

#[derive(Debug)]
pub enum PathExpr {
    Root(Option<RelativePathExpr>),
    Anywhere(RelativePathExpr),
    Relative(RelativePathExpr),
}

impl PathExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [36] PathExpr ::= ("/" RelativePathExpr?) | ("//" RelativePathExpr) | RelativePathExpr

        alt((
            preceded(tag("/"), opt(RelativePathExpr::nom)).map(Self::Root),
            preceded(tag("//"), RelativePathExpr::nom).map(Self::Anywhere),
            RelativePathExpr::nom.map(Self::Anywhere),
        ))
        .parse(input)
    }
}

#[derive(Debug)]
pub struct RelativePathExpr {
    pub head: StepExpr,
    pub list: Vec<RelativePathExprOp>,
}

#[derive(Debug)]
pub enum RelativePathExprOp {
    Child(StepExpr),
    Descendant(StepExpr),
}

impl RelativePathExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [37] RelativePathExpr ::= StepExpr (("/" | "//") StepExpr)*

        let (input, head) = StepExpr::nom(input)?;
        let (input, list) = many0(alt((
            preceded(trim0(tag("/")), StepExpr::nom).map(RelativePathExprOp::Child),
            preceded(trim0(tag("//")), StepExpr::nom).map(RelativePathExprOp::Descendant),
        )))
        .parse(input)?;

        let ret = Self { head, list };

        Ok((input, ret))
    }
}

#[derive(Debug)]
pub struct StepExpr;

impl StepExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [38] StepExpr ::= PostfixExpr | AxisStep

        unimplemented!()
    }
}

#[derive(Debug)]
pub struct PostfixExpr;

impl PostfixExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [49] PostfixExpr ::= PrimaryExpr ( Predicate | ArgumentList | Lookup )*

        unimplemented!()
    }
}

#[derive(Debug)]
pub struct AxisStep {
    pub op: AxisStepOp,
    pub test: NodeTest,
    pub predicates: Vec<Predicate>,
}

#[derive(Debug, Clone, Copy)]
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

impl AxisStep {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [39] AxisStep          ::= (ReverseStep | ForwardStep) PredicateList
        // [40] ForwardStep       ::= (ForwardAxis NodeTest) | AbbrevForwardStep
        // [41] ForwardAxis       ::= ("child" "::")
        //                                | ("descendant" "::")
        //                                | ("attribute" "::")
        //                                | ("self" "::")
        //                                | ("descendant-or-self" "::")
        //                                | ("following-sibling" "::")
        //                                | ("following" "::")
        //                                | ("namespace" "::")
        // [42] AbbrevForwardStep ::= "@"? NodeTest
        // [43] ReverseStep       ::= (ReverseAxis NodeTest) | AbbrevReverseStep
        // [44] ReverseAxis       ::= ("parent" "::")
        //                                | ("ancestor" "::")
        //                                | ("preceding-sibling" "::")
        //                                | ("preceding" "::")
        //                                | ("ancestor-or-self" "::")
        // [45] AbbrevReverseStep ::= ".."
        // [51] PredicateList     ::= Predicate*

        fn test(
            s: &str,
            op: AxisStepOp,
        ) -> impl Parser<&[u8], Output = AxisStepOp, Error = Error<&[u8]>> {
            (multispace0, tag(s), multispace0, tag("::"), multispace0).map(move |_| op)
        }

        #[rustfmt::skip]
        let (input, (op, test)) = alt((
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
        .parse(input)?;
        let (input, predicates) = many0(Predicate::nom).parse(input)?;

        let ret = Self {
            op,
            test,
            predicates,
        };

        Ok((input, ret))
    }
}

#[derive(Debug)]
pub enum NodeTest {
    Kind(KindTest),
    EQName(EQName),
    Wildcard(Wildcard),
}

impl NodeTest {
    #[must_use]
    pub fn node() -> Self {
        Self::Kind(KindTest::AnyKind)
    }

    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [46] NodeTest ::= KindTest | NameTest
        // [47] NameTest ::= EQName | Wildcard

        alt((
            KindTest::nom.map(Self::Kind),
            EQName::nom.map(Self::EQName),
            Wildcard::nom.map(Self::Wildcard),
        ))
        .parse(input)
    }
}

#[derive(Debug)]
pub enum KindTest {
    Document,
    DocumentElement(ElementTest),
    DocumentSchemaElement(SchemaElementTest),
    Element(ElementTest),
    Attribute(AttributeTest),
    SchemaElement(SchemaElementTest),
    SchemaAttribute(SchemaAttributeTest),
    PI(PITest),
    Comment,
    Text,
    NamespaceNode,
    AnyKind,
}

#[derive(Debug)]
pub enum TypenameTest {
    Empty,
    Named(EQName, TypenameOpt),
    Wildcard(TypenameOpt),
}

#[derive(Debug)]
pub enum PITest {
    Empty,
    NCName(NCName),
    StringLiteral(StringLiteral),
}

pub type ElementTest = TypenameTest;
pub type AttributeTest = TypenameTest;
pub type ElementTestTypename = TypenameOpt;
pub type TypenameOpt = Option<(EQName, bool)>;

pub type SchemaElementTest = ElementDeclaration;
pub type ElementDeclaration = ElementName;
pub type ElementName = EQName;

pub type SchemaAttributeTest = AttributeDeclaration;
pub type AttributeDeclaration = AttributeName;
pub type AttributeName = EQName;

impl KindTest {
    #[allow(clippy::too_many_lines)]
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [83]  KindTest              ::= DocumentTest
        //                                     | ElementTest
        //                                     | AttributeTest
        //                                     | SchemaElementTest
        //                                     | SchemaAttributeTest
        //                                     | PITest
        //                                     | CommentTest
        //                                     | TextTest
        //                                     | NamespaceNodeTest
        //                                     | AnyKindTest
        // [84]  AnyKindTest           ::= "node" "(" ")"
        // [86]  TextTest              ::= "text" "(" ")"
        // [85]  DocumentTest          ::= "document-node" "(" (ElementTest | SchemaElementTest)? ")"
        // [88]  NamespaceNodeTest     ::= "namespace-node" "(" ")"
        // [90]  AttributeTest         ::= "attribute" "(" (AttribNameOrWildcard ("," TypeName)?)? ")"
        // [87]  CommentTest           ::= "comment" "(" ")"
        // [89]  PITest                ::= "processing-instruction" "(" (NCName | StringLiteral)? ")"
        // [91]  AttribNameOrWildcard  ::= AttributeName | "*"
        // [92]  SchemaAttributeTest   ::= "schema-attribute" "(" AttributeDeclaration ")"
        // [93]  AttributeDeclaration  ::= AttributeName
        // [94]  ElementTest           ::= "element" "(" (ElementNameOrWildcard ("," TypeName "?"?)?)? ")"
        // [95]  ElementNameOrWildcard ::= ElementName | "*"
        // [96]  SchemaElementTest     ::= "schema-element" "(" ElementDeclaration ")"
        // [97]  ElementDeclaration    ::= ElementName
        // [98]  AttributeName         ::= EQName
        // [99]  ElementName           ::= EQName
        // [101] TypeName              ::= EQName

        fn document_test(input: &[u8]) -> IResult<&[u8], KindTest> {
            delimited(
                (
                    multispace0,
                    tag("document-node"),
                    multispace0,
                    tag("("),
                    multispace0,
                ),
                opt(alt((
                    element_test.map(KindTest::DocumentElement),
                    schema_element_test.map(KindTest::DocumentSchemaElement),
                )))
                .map(|x| x.unwrap_or(KindTest::Document)),
                (multispace0, tag(")"), multispace0),
            )
            .parse(input)
        }

        fn element_test(input: &[u8]) -> IResult<&[u8], ElementTest> {
            typename_test("element").parse(input)
        }

        fn attribute_test(input: &[u8]) -> IResult<&[u8], AttributeTest> {
            typename_test("attribute").parse(input)
        }

        fn schema_element_test(input: &[u8]) -> IResult<&[u8], SchemaElementTest> {
            declaration_test("schema-element").parse(input)
        }

        fn schema_attribute_test(input: &[u8]) -> IResult<&[u8], SchemaAttributeTest> {
            declaration_test("schema-attribute").parse(input)
        }

        fn pi_test(input: &[u8]) -> IResult<&[u8], PITest> {
            delimited(
                (
                    multispace0,
                    tag("processing-instruction"),
                    multispace0,
                    tag("("),
                    multispace0,
                ),
                opt(alt((
                    NCName::nom.map(PITest::NCName),
                    StringLiteral::nom.map(PITest::StringLiteral),
                )))
                .map(|x| x.unwrap_or(PITest::Empty)),
                (multispace0, tag(")"), multispace0),
            )
            .parse(input)
        }

        fn typename_test(
            name: &str,
        ) -> impl Parser<&[u8], Output = TypenameTest, Error = Error<&[u8]>> {
            delimited(
                (multispace0, tag(name), multispace0, tag("("), multispace0),
                opt(alt((
                    (tag("*"), typename_opt).map(|(_, ty)| TypenameTest::Wildcard(ty)),
                    (EQName::nom, typename_opt).map(|(name, ty)| TypenameTest::Named(name, ty)),
                )))
                .map(|x| x.unwrap_or(TypenameTest::Empty)),
                (multispace0, tag(")"), multispace0),
            )
        }

        fn declaration_test(
            name: &str,
        ) -> impl Parser<&[u8], Output = EQName, Error = Error<&[u8]>> {
            delimited(
                (multispace0, tag(name), multispace0, tag("("), multispace0),
                EQName::nom,
                (multispace0, tag(")"), multispace0),
            )
        }

        fn simple_test(name: &str) -> impl Parser<&[u8], Output = (), Error = Error<&[u8]>> {
            (
                multispace0,
                tag(name),
                multispace0,
                tag("("),
                multispace0,
                tag(")"),
                multispace0,
            )
                .map(|_| ())
        }

        fn typename_opt(input: &[u8]) -> IResult<&[u8], ElementTestTypename> {
            opt(preceded(
                (multispace0, tag(","), multispace0),
                (
                    EQName::nom,
                    opt((multispace0, tag("?"), multispace0)).map(|x| x.is_some()),
                ),
            ))
            .parse(input)
        }

        alt((
            document_test,
            element_test.map(KindTest::Element),
            attribute_test.map(KindTest::Attribute),
            schema_element_test.map(KindTest::SchemaElement),
            schema_attribute_test.map(KindTest::SchemaAttribute),
            pi_test.map(KindTest::PI),
            simple_test("comment").map(|()| KindTest::Comment),
            simple_test("text").map(|()| KindTest::Text),
            simple_test("namespace-node").map(|()| KindTest::NamespaceNode),
            simple_test("node").map(|()| KindTest::AnyKind),
        ))
        .parse(input)
    }
}

#[derive(Debug)]
pub struct Predicate;

impl Predicate {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct EQName;

impl EQName {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct NCName;

impl NCName {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct StringLiteral;

impl StringLiteral {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct Wildcard;

impl Wildcard {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct SequenceType;

impl SequenceType {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        // [79] SequenceType        ::= ( "empty-sequence" "(" ")" ) | ( ItemType OccurrenceIndicator? )
        // [80] OccurrenceIndicator ::= "?" | "*" | "+"

        unimplemented!()
    }
}

#[derive(Debug)]
pub struct SingleType;

impl SingleType {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct ArrowFunctionSpecifier;

impl ArrowFunctionSpecifier {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct ArgumentList;

impl ArgumentList {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        unimplemented!()
    }
}

pub type VarName = EQName;

fn trim0<I, E, F>(parser: F) -> impl Parser<I, Output = F::Output, Error = E>
where
    I: Input,
    I::Item: AsChar,
    E: ParseError<I>,
    F: Parser<I, Error = E>,
{
    delimited(multispace0, parser, multispace0)
}

// [2]      ParamList               ::=     Param ("," Param)*
// [3]      Param                   ::=     "$" EQName TypeDeclaration?
// [4]      FunctionBody            ::=     EnclosedExpr
// [5]      EnclosedExpr            ::=     "{" Expr? "}"
// [12]     SimpleLetClause         ::=     let" SimpleLetBinding ("," SimpleLetBinding)*
// [13]     SimpleLetBinding        ::=     "$" VarName ":=" ExprSingle
// [48]     Wildcard                ::=     "*"
//                                              | (NCName ":*")
//                                              | ("*:" NCName)
//                                              | (BracedURILiteral "*")
// [50]     ArgumentList            ::=     "(" (Argument ("," Argument)*)? ")"
// [52]     Predicate               ::=     "[" Expr "]"
// [53]     Lookup                  ::=     "?" KeySpecifier
// [54]     KeySpecifier            ::=     NCName | IntegerLiteral | ParenthesizedExpr | "*"
// [55]     ArrowFunctionSpecifier  ::=     EQName | VarRef | ParenthesizedExpr
// [56]     PrimaryExpr             ::=     Literal
//                                              | VarRef
//                                              | ParenthesizedExpr
//                                              | ContextItemExpr
//                                              | FunctionCall
//                                              | FunctionItemExpr
//                                              | MapConstructor
//                                              | ArrayConstructor
//                                              | UnaryLookup
// [57]     Literal                 ::=     NumericLiteral | StringLiteral
// [58]     NumericLiteral          ::=     IntegerLiteral | DecimalLiteral | DoubleLiteral
// [59]     VarRef                  ::=     "$" VarName
// [61]     ParenthesizedExpr       ::=     "(" Expr? ")"
// [62]     ContextItemExpr         ::=     "."
// [63]     FunctionCall            ::=     EQName ArgumentList
// [64]     Argument                ::=     ExprSingle | ArgumentPlaceholder
// [65]     ArgumentPlaceholder     ::=     "?"
// [66]     FunctionItemExpr        ::=     NamedFunctionRef | InlineFunctionExpr
// [67]     NamedFunctionRef        ::=     EQName "#" IntegerLiteral
// [68]     InlineFunctionExpr      ::=     "function" "(" ParamList? ")" ("as" SequenceType)? FunctionBody
// [69]     MapConstructor          ::=     "map" "{" (MapConstructorEntry ("," MapConstructorEntry)*)? "}"
// [70]     MapConstructorEntry     ::=     MapKeyExpr ":" MapValueExpr
// [71]     MapKeyExpr              ::=     ExprSingle
// [72]     MapValueExpr            ::=     ExprSingle
// [73]     ArrayConstructor        ::=     SquareArrayConstructor | CurlyArrayConstructor
// [74]     SquareArrayConstructor  ::=     "[" (ExprSingle ("," ExprSingle)*)? "]"
// [75]     CurlyArrayConstructor   ::=     "array" EnclosedExpr
// [76]     UnaryLookup             ::=     "?" KeySpecifier
// [77]     SingleType              ::=     SimpleTypeName "?"?
// [78]     TypeDeclaration         ::=     "as" SequenceType
// [81]     ItemType                ::=     KindTest | ("item" "(" ")") | FunctionTest | MapTest | ArrayTest | AtomicOrUnionType | ParenthesizedItemType
// [82]     AtomicOrUnionType       ::=     EQName
// [100]    SimpleTypeName          ::=     TypeName
// [102]    FunctionTest            ::=     AnyFunctionTest
//                                              | TypedFunctionTest
// [103]    AnyFunctionTest         ::=     "function" "(" "*" ")"
// [104]    TypedFunctionTest       ::=     "function" "(" (SequenceType ("," SequenceType)*)? ")" "as" SequenceType
// [105]    MapTest                 ::=     AnyMapTest | TypedMapTest
// [106]    AnyMapTest              ::=     "map" "(" "*" ")"
// [107]    TypedMapTest            ::=     "map" "(" AtomicOrUnionType "," SequenceType ")"
// [108]    ArrayTest               ::=     AnyArrayTest | TypedArrayTest
// [109]    AnyArrayTest            ::=     "array" "(" "*" ")"
// [110]    TypedArrayTest          ::=     "array" "(" SequenceType ")"
// [111]    ParenthesizedItemType   ::=     "(" ItemType ")"
// [112]    EQName                  ::=     QName | URIQualifiedName
// [113]    IntegerLiteral          ::=     Digits
// [114]    DecimalLiteral          ::=     ("." Digits) | (Digits "." [0-9]*)
// [115]    DoubleLiteral           ::=     (("." Digits) | (Digits ("." [0-9]*)?)) [eE] [+-]? Digits
// [116]    StringLiteral           ::=     ('"' (EscapeQuot | [^"])* '"') | ("'" (EscapeApos | [^'])* "'")
// [117]    URIQualifiedName        ::=     BracedURILiteral NCName
// [118]    BracedURILiteral        ::=     "Q" "{" [^{}]* "}"
// [119]    EscapeQuot              ::=     '""'
// [120]    EscapeApos              ::=     "''"
// [121]    Comment                 ::=     "(:" (CommentContents | Comment)* ":)"
// [122]    QName                   ::=     [http://www.w3.org/TR/REC-xml-names/#NT-QName]Names
// [123]    NCName                  ::=     [http://www.w3.org/TR/REC-xml-names/#NT-NCName]Names
// [124]    Char                    ::=     [http://www.w3.org/TR/REC-xml#NT-Char]XML
// [125]    Digits                  ::=     [0-9]+
// [126]    CommentContents         ::=     (Char+ - (Char* ('(:' | ':)') Char*))

#[cfg(test)]
mod tests {
    use super::XPath;

    #[test]
    fn parse() {
        XPath::nom(b"count(ram:ActualAmount)=1").unwrap();
    }
}
