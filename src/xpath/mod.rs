#![allow(missing_docs)]

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::opt,
    error::{Error, ErrorKind},
    multi::{many1, separated_list1},
    sequence::delimited,
    Err, IResult, Parser,
};

#[derive(Debug)]
pub struct XPath(pub Vec<Expr>);

impl XPath {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        many1(Expr::nom).map(Self).parse(input)
    }
}

#[derive(Debug)]
pub enum Expr {
    For(ForExpr),
    Let(LetExpr),
    Quantified(QuantifiedExpr),
    If(IfExpr),
    Or(OrExpr),
}

impl Expr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        alt((
            ForExpr::nom.map(Self::For),
            LetExpr::nom.map(Self::Let),
            QuantifiedExpr::nom.map(Self::Quantified),
            IfExpr::nom.map(Self::If),
            OrExpr::nom.map(Self::Or),
        ))
        .parse(input)
    }
}

#[derive(Debug)]
pub struct ForExpr;

impl ForExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct LetExpr;

impl LetExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct QuantifiedExpr;

impl QuantifiedExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct IfExpr;

impl IfExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct OrExpr(pub Vec<AndExpr>);

impl OrExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        fn sep(input: &[u8]) -> IResult<&[u8], ()> {
            delimited(multispace0, tag("or"), multispace0)
                .map(|_| ())
                .parse(input)
        }

        separated_list1(sep, AndExpr::nom).map(Self).parse(input)
    }
}

#[derive(Debug)]
pub struct AndExpr(pub Vec<ComparisonExpr>);

impl AndExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        fn sep(input: &[u8]) -> IResult<&[u8], ()> {
            delimited(multispace0, tag("and"), multispace0)
                .map(|_| ())
                .parse(input)
        }

        separated_list1(sep, ComparisonExpr::nom)
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
        fn op(
            op: &str,
        ) -> impl Parser<&[u8], Output = (&[u8], StringConcatExpr), Error = Error<&[u8]>> {
            (
                delimited(multispace0, tag(op), multispace0),
                StringConcatExpr::nom,
            )
        }

        let (input, a) = StringConcatExpr::nom(input)?;
        let (input, b) = opt(alt((
            op("="),
            op("!="),
            op("<"),
            op("<="),
            op(">"),
            op(">="),
            op("eq"),
            op("ne"),
            op("lt"),
            op("le"),
            op("gt"),
            op("ge"),
            op("is"),
            op("<<"),
            op(">>"),
        )))
        .parse(input)?;

        let ret = match (a, b) {
            (a, None) => Self::Unary(a),
            (a, Some((b"=" | b"eq", b))) => Self::Equal(a, b),
            (a, Some((b"!=" | b"ne", b))) => Self::NotEqual(a, b),
            (a, Some((b"<" | b"lt", b))) => Self::LowerThan(a, b),
            (a, Some((b"<=" | b"le", b))) => Self::LowerEqual(a, b),
            (a, Some((b">" | b"gt", b))) => Self::GreaterThan(a, b),
            (a, Some((b">=" | b"ge", b))) => Self::GreaterEqual(a, b),
            (a, Some((b"is", b))) => Self::Is(a, b),
            (a, Some((b"<<", b))) => Self::Precedes(a, b),
            (a, Some((b">>", b))) => Self::Precedes(a, b),
            (_, Some((_, _))) => Err(Err::Failure(Error::new(input, ErrorKind::Tag)))?,
        };

        Ok((input, ret))
    }
}

#[derive(Debug)]
pub enum StringConcatExpr {
    Unary(),
}

impl StringConcatExpr {
    pub fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        unimplemented!()
    }
}

// https://www.w3.org/TR/xpath-31/#prod-xpath31-XPath
// [1]      XPath                   ::=     Expr
// [2]      ParamList               ::=     Param ("," Param)*
// [3]      Param                   ::=     "$" EQName TypeDeclaration?
// [4]      FunctionBody            ::=     EnclosedExpr
// [5]      EnclosedExpr            ::=     "{" Expr? "}"
// [6]      Expr                    ::=     ExprSingle ("," ExprSingle)*
// [7]      ExprSingle              ::=     ForExpr
//                                              | LetExpr
//                                              | QuantifiedExpr
//                                              | IfExpr
//                                              | OrExpr
// [8]      ForExpr                 ::=     SimpleForClause "return" ExprSingle
// [9]      SimpleForClause         ::=     for" SimpleForBinding ("," SimpleForBinding)*
// [10]     SimpleForBinding        ::=     "$" VarName "in" ExprSingle
// [11]     LetExpr                 ::=     SimpleLetClause "return" ExprSingle
// [12]     SimpleLetClause         ::=     let" SimpleLetBinding ("," SimpleLetBinding)*
// [13]     SimpleLetBinding        ::=     "$" VarName ":=" ExprSingle
// [14]     QuantifiedExpr          ::=     ("some" | "every") "$" VarName "in" ExprSingle ("," "$" VarName "in" ExprSingle)* "satisfies" ExprSingle
// [15]     IfExpr                  ::=     if" "(" Expr ")" "then" ExprSingle "else" ExprSingle
// [16]     OrExpr                  ::=     AndExpr ( "or" AndExpr )*
// [17]     AndExpr                 ::=     ComparisonExpr ( "and" ComparisonExpr )*
// [18]     ComparisonExpr          ::=     StringConcatExpr ( (ValueComp
//                                              | GeneralComp
//                                              | NodeComp) StringConcatExpr )?
// [19]     StringConcatExpr        ::=     RangeExpr ( "||" RangeExpr )*
// [20]     RangeExpr               ::=     AdditiveExpr ( "to" AdditiveExpr )?
// [21]     AdditiveExpr            ::=     MultiplicativeExpr ( ("+" | "-") MultiplicativeExpr )*
// [22]     MultiplicativeExpr      ::=     UnionExpr ( ("*" | "div" | "idiv" | "mod") UnionExpr )*
// [23]     UnionExpr               ::=     IntersectExceptExpr ( ("union" | "|") IntersectExceptExpr )*
// [24]     IntersectExceptExpr     ::=     InstanceofExpr ( ("intersect" | "except") InstanceofExpr )*
// [25]     InstanceofExpr          ::=     TreatExpr ( "instance" "of" SequenceType )?
// [26]     TreatExpr               ::=     CastableExpr ( "treat" "as" SequenceType )?
// [27]     CastableExpr            ::=     CastExpr ( "castable" "as" SingleType )?
// [28]     CastExpr                ::=     ArrowExpr ( "cast" "as" SingleType )?
// [29]     ArrowExpr               ::=     UnaryExpr ( "=>" ArrowFunctionSpecifier ArgumentList )*
// [30]     UnaryExpr               ::=     ("-" | "+")* ValueExpr
// [31]     ValueExpr               ::=     SimpleMapExpr
// [32]     GeneralComp             ::=     "=" | "!=" | "<" | "<=" | ">" | ">="
// [33]     ValueComp               ::=     "eq" | "ne" | "lt" | "le" | "gt" | "ge"
// [34]     NodeComp                ::=     "is" | "<<" | ">>"
// [35]     SimpleMapExpr           ::=     PathExpr ("!" PathExpr)*
// [36]     PathExpr                ::=     ("/" RelativePathExpr?)
//                                              | ("//" RelativePathExpr)
//                                              | RelativePathExpr
// [37]     RelativePathExpr        ::=     StepExpr (("/" | "//") StepExpr)*
// [38]     StepExpr                ::=     PostfixExpr | AxisStep
// [39]     AxisStep                ::=     (ReverseStep | ForwardStep) PredicateList
// [40]     ForwardStep             ::=     (ForwardAxis NodeTest) | AbbrevForwardStep
// [41]     ForwardAxis             ::=     ("child" "::")
//                                              | ("descendant" "::")
//                                              | ("attribute" "::")
//                                              | ("self" "::")
//                                              | ("descendant-or-self" "::")
//                                              | ("following-sibling" "::")
//                                              | ("following" "::")
//                                              | ("namespace" "::")
// [42]     AbbrevForwardStep       ::=     "@"? NodeTest
// [43]     ReverseStep             ::=     (ReverseAxis NodeTest) | AbbrevReverseStep
// [44]     ReverseAxis             ::=     ("parent" "::")
//                                              | ("ancestor" "::")
//                                              | ("preceding-sibling" "::")
//                                              | ("preceding" "::")
//                                              | ("ancestor-or-self" "::")
// [45]     AbbrevReverseStep       ::=     ".."
// [46]     NodeTest                ::=     KindTest | NameTest
// [47]     NameTest                ::=     EQName | Wildcard
// [48]     Wildcard                ::=     "*"
//                                              | (NCName ":*")
//                                              | ("*:" NCName)
//                                              | (BracedURILiteral "*")
// [49]     PostfixExpr             ::=     PrimaryExpr (Predicate | ArgumentList | Lookup)*
// [50]     ArgumentList            ::=     "(" (Argument ("," Argument)*)? ")"
// [51]     PredicateList           ::=     Predicate*
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
// [60]     VarName                 ::=     EQName
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
// [79]     SequenceType            ::=     ("empty-sequence" "(" ")") | (ItemType OccurrenceIndicator?)
// [80]     OccurrenceIndicator     ::=     "?" | "*" | "+"
// [81]     ItemType                ::=     KindTest | ("item" "(" ")") | FunctionTest | MapTest | ArrayTest | AtomicOrUnionType | ParenthesizedItemType
// [82]     AtomicOrUnionType       ::=     EQName
// [83]     KindTest                ::=     DocumentTest
//                                              | ElementTest
//                                              | AttributeTest
//                                              | SchemaElementTest
//                                              | SchemaAttributeTest
//                                              | PITest
//                                              | CommentTest
//                                              | TextTest
//                                              | NamespaceNodeTest
//                                              | AnyKindTest
// [84]     AnyKindTest             ::=     "node" "(" ")"
// [85]     DocumentTest            ::=     "document-node" "(" (ElementTest | SchemaElementTest)? ")"
// [86]     TextTest                ::=     "text" "(" ")"
// [87]     CommentTest             ::=     "comment" "(" ")"
// [88]     NamespaceNodeTest       ::=     "namespace-node" "(" ")"
// [89]     PITest                  ::=     "processing-instruction" "(" (NCName | StringLiteral)? ")"
// [90]     AttributeTest           ::=     "attribute" "(" (AttribNameOrWildcard ("," TypeName)?)? ")"
// [91]     AttribNameOrWildcard    ::=     AttributeName | "*"
// [92]     SchemaAttributeTest     ::=     "schema-attribute" "(" AttributeDeclaration ")"
// [93]     AttributeDeclaration    ::=     AttributeName
// [94]     ElementTest             ::=     "element" "(" (ElementNameOrWildcard ("," TypeName "?"?)?)? ")"
// [95]     ElementNameOrWildcard   ::=     ElementName | "*"
// [96]     SchemaElementTest       ::=     "schema-element" "(" ElementDeclaration ")"
// [97]     ElementDeclaration      ::=     ElementName
// [98]     AttributeName           ::=     EQName
// [99]     ElementName             ::=     EQName
// [100]    SimpleTypeName          ::=     TypeName
// [101]    TypeName                ::=     EQName
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
