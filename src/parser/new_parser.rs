use crate::srule;
use std::fmt::Display;

use super::*;

use ParseResult::*;
use unicode_xid::UnicodeXID;

/// 
/// A parse position the parser.
/// 
#[derive(Clone, Copy)]
pub struct PPos {
    /// The byte index in the `str`
    pub index: usize,
    /// The character column (starts at 1).
    pub column: usize,
    /// The character line (starts at 1).
    pub line: usize,
}

impl PPos {
    /// Creates and returns a new `PPos`.
    pub fn new() -> Self {
        Self {
            index: 0,
            column: 1,
            line: 1,
        }
    }
}

impl std::fmt::Debug for PPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for PPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.line, f)?;
        f.write_str(":")?;
        Display::fmt(&self.column, f)?;
        f.write_str(":")?;
        Display::fmt(&self.index, f)
    }
}

impl ParsePos for PPos {
    type Key = usize;
    fn key(&self) -> usize {
        self.index
    }
}

impl ParseStore<PPos, char> for str {
    fn value_at(&self, pos: &mut PPos) -> Option<char> {
        if let Some(ch) = self.value_at(&mut pos.index) {
            if ch == '\n' {
                pos.line += 1;
                pos.column = 1;
            } else {
                pos.column += 1;
            }
            Some(ch)
        } else {
            None
        }
    }
}

impl ParseStore<PPos, char> for &str {
    fn value_at(&self, pos: &mut PPos) -> Option<char> {
        (**self).value_at(pos)
    }
}


#[derive(Debug, Clone)]
pub struct RMacroDef {
    pub span: Span<PPos>,
    pub ident: Span<PPos>,
    pub rules_def: RMacroRulesDef,
}

#[derive(Debug, Clone)]
pub struct RMacroRulesDef {
    pub span: Span<PPos>,
    pub rules: RMacroRules
}

#[derive(Debug, Clone)]
pub struct RMacroRules {
    pub span: Span<PPos>,
    pub rules: Vec<RMacroRule>,
}

#[derive(Debug, Clone)]
pub struct RMacroRule {
    pub span: Span<PPos>,
    pub macro_match: RMacroMatch,
    pub macro_transcriber: RMacroTranscriber,
}

#[derive(Debug, Clone)]
pub struct RMacroMatcher {
    pub span: Span<PPos>,
    pub matches: Vec<RMacroMatch>,
}

#[derive(Debug, Clone)]
pub enum RMacroMatch {
    Token(RToken),
    Matcher(RMacroMatcher),
    Arg(RMacroArg),
    OpArg(RMacroOpArg),
}

#[derive(Debug, Clone)]
pub struct RMacroArg {
    pub span: Span<PPos>,
    pub arg: Span<PPos>,
    pub spec: RMatchSpec,
}

#[derive(Debug, Clone)]
pub enum RMatchSpec {
    Block(Span<PPos>),
    Expr(Span<PPos>),
    Ident(Span<PPos>),
    Item(Span<PPos>),
    Lifetime(Span<PPos>),
    Literal(Span<PPos>),
    Meta(Span<PPos>),
    Pat(Span<PPos>),
    PatParam(Span<PPos>),
    Path(Span<PPos>),
    Stmt(Span<PPos>),
    Tt(Span<PPos>),
    Ty(Span<PPos>),
    Vis(Span<PPos>),
}

#[derive(Debug, Clone)]
pub struct RMacroOpArg {
    pub span: Span<PPos>,
    pub matches: Vec<RMacroMatch>,
    pub sep: Option<RToken>,
    pub op: RMacroOp,
}

#[derive(Debug, Clone)]
pub struct RMacroTranscriber {
    pub span: Span<PPos>,
    pub tree: RDelimTokenTree,
}

#[derive(Debug, Clone)]
pub enum RMacroOp {
    ZeroOrMore(Span<PPos>),
    OneOrMore(Span<PPos>),
    Optional(Span<PPos>),
}

#[derive(Debug, Clone)]
pub struct RCrate {
    pub utf8bom: Option<Span<PPos>>,
    pub shebang: Option<Span<PPos>>,
    pub items: Vec<RItem>,
}

#[derive(Debug, Clone)]
pub struct ROuterAttr {
    pub span: Span<PPos>
}

#[derive(Debug, Clone)]
pub struct RInnerAttr {
    pub span: Span<PPos>
}

#[derive(Debug, Clone)]
pub struct RFn {
    span: Span<PPos>,
    is_const: bool,
    is_async: bool,
    is_unsafe: bool,
    is_extern: Option<RABI>,
    name: Span<PPos>,
    generics: RGenericParams,
    params: RFnParams,
    ret_type: Option<RType>,
    where_clause: Option<RWhereClause>,
    body: Option<RBlockExpr>,
}

#[derive(Debug, Clone)]
pub struct RFnParams {
    pub span: Span<PPos>,
    pub self_param: Option<RSelfParam>,
    pub params: Vec<RFnParam>,
}

#[derive(Debug, Clone)]
pub enum RABI {
    StrLit(RStrLit),
    RawStrLit(RRawStrLit)
}

#[derive(Debug, Clone)]
pub struct RWhereClause {
    pub span: Span<PPos>,
    pub items: Vec<RWhereClauseItem>,
}

#[derive(Debug, Clone)]
pub enum RWhereClauseItem {
    Lifetime {
        span: Span<PPos>,
        lifetime: RLifetime,
        bounds: Vec<RLifetime>,
    },
    Type {
        span: Span<PPos>,
        lifetime: Option<RForLifetimes>,
        ty: RType,
        bounds: RTypeParamBounds
    }
}

#[derive(Debug, Clone)]
pub struct RForLifetimes {
    span: Span<PPos>,
    generics: RGenericParams,
}

#[derive(Debug, Clone)]
pub struct RAssociatedItem {
    pub span: Span<PPos>
}

#[derive(Debug, Clone)]
pub enum RSelfParam {
    NotBorrowed {
        attrs: Vec<ROuterAttr>,
        span: Span<PPos>,
        mutable: bool,
    },
    Borrowed {
        attrs: Vec<ROuterAttr>,
        span: Span<PPos>,
        mutable: bool,
    },
    BorrowedWithLife {
        attrs: Vec<ROuterAttr>,
        span: Span<PPos>,
        mutable: bool,
        lifetime: RLifetime,
    },
    Longhand {
        attrs: Vec<ROuterAttr>,
        span: Span<PPos>,
        mutable: bool,
        ty: RType,
    }
}

#[derive(Debug, Clone)]
pub enum RFnParam {
    Pattern {
        span: Span<PPos>,
        attrs: Vec<ROuterAttr>,
        pattern: RFnParamPattern
    },
    Rest {
        span: Span<PPos>,
        attrs: Vec<ROuterAttr>,
    },
    Type {
        span: Span<PPos>,
        attrs: Vec<ROuterAttr>,
        ty: RType,
    },
}

#[derive(Debug, Clone)]
pub enum RFnParamPattern {
    Type {
        span: Span<PPos>,
        pat: RSubPattern,
        ty: RType,
    },
    Rest {
        span: Span<PPos>,
    },
}


#[derive(Debug, Clone)]
pub struct RGenericParam {
    pub span: Span<PPos>,
    pub segs: Vec<RGenericParamSeg>,
}

#[derive(Debug, Clone)]
pub enum RGenericParamSeg {
    Lifetime {
        span: Span<PPos>,
        attrs: Vec<ROuterAttr>,
        param: RLifetimeParam
    },
    Type {
        span: Span<PPos>,
        attrs: Vec<ROuterAttr>,
        param: RTypeParam
    },
    Const {
        span: Span<PPos>,
        attrs: Vec<ROuterAttr>,
        param: RConstParam
    },
}

#[derive(Debug, Clone)]
pub struct RLifetimeParam {
    pub span: Span<PPos>,
    pub lifetime: RLifetime,
    pub bounds: Vec<RLifetime>,
}

#[derive(Debug, Clone)]
pub struct RGenericParams {
    span: Span<PPos>,
    params: Vec<RGenericParam>,
}

#[derive(Debug, Clone)]
pub struct RMatchExpr {
    pub span: Span<PPos>,
}

#[derive(Debug, Clone)]
pub struct RGroup {
    span: Span<PPos>,
}

#[derive(Debug, Clone)]
pub struct REnum {
    pub span: Span<PPos>,
}

#[derive(Debug, Clone)]
pub struct RUnion {
    pub span: Span<PPos>,
    pub ident: Span<PPos>,
    pub generics: RGenericParams,
    pub clause: Option<RWhereClause>,
    pub fields: RStructFields,
}

#[derive(Debug, Clone)]
pub struct RTupleFields {
    pub span: Span<PPos>,
    pub fields: Vec<RTupleField>,
}

#[derive(Debug, Clone)]
pub struct RTupleField {
    pub span: Span<PPos>,
    pub attrs: Vec<ROuterAttr>,
    pub vis: Option<RVis>,
    pub ty: RType,
}


#[derive(Debug, Clone)]
pub struct RStructFields {
    pub span: Span<PPos>,
    pub fields: Vec<RStructField>,
}

#[derive(Debug, Clone)]
pub struct RStructField {
    pub span: Span<PPos>,
    pub attrs: Vec<ROuterAttr>,
    pub vis: Option<RVis>,
    pub ident: Span<PPos>,
    pub ty: RType,
}

#[derive(Debug, Clone)]
pub struct RConstItem {
    pub span: Span<PPos>,
    pub ident: Span<PPos>,
    pub ty: RType,
    pub expr: Option<RExpr>,
}

#[derive(Debug, Clone)]
pub struct RStaticItem {
    pub span: Span<PPos>,
    pub mutable: bool,
    pub ident: Span<PPos>,
    pub ty: RType,
    pub expr: Option<RExpr>,
}

#[derive(Debug, Clone)]
pub struct RTypeAlias {
    pub span: Span<PPos>,
}

#[derive(Debug, Clone)]
pub enum RStruct {
    Struct {
        span: Span<PPos>,
        ident: Span<PPos>,
        generics: RGenericParams,
        fields: RStructFields,
        clause: Option<RWhereClause>
    },
    Tuple {
        span: Span<PPos>,
        ident: Span<PPos>,
        generics: RGenericParams,
        fields: RTupleFields,
        clause: Option<RWhereClause>
    },
}

#[derive(Debug, Clone)]
pub struct RTrait {
    pub span: Span<PPos>,
}

#[derive(Debug, Clone)]
pub struct RImpl {
    pub span: Span<PPos>,
}

#[derive(Debug, Clone)]
pub struct RExternBlock {
    pub span: Span<PPos>,
    pub is_unsafe: bool,
    pub abi: Option<RABI>,
    pub attrs: Vec<RInnerAttr>,
    pub items: Vec<RExternalItem>,
}


#[derive(Debug, Clone)]
pub enum RExternalItem {
    MacroInvocation {
        span: Span<PPos>,
        attrs: Vec<ROuterAttr>,
        invoc: RMacroInvocation,
    },
    Static {
        span: Span<PPos>,
        attrs: Vec<ROuterAttr>,
        vis: Option<RVis>,
        stat: RStaticItem,
    },
    Fn {
        span: Span<PPos>,
        attrs: Vec<ROuterAttr>,
        vis: Option<RVis>,
        func: RFn,
    }
}

#[derive(Debug, Clone)]
pub enum RStatement {
    Item(RItem),
    Let(RLetStatement),
    Expr(RExpr),
    Macro(RMacroInvocation)
}

///
/// A literal that cannot have a sign.
/// 
#[derive(Debug, Clone)]
pub enum RLit {
    Char(RCharLit),
    String(RStrLit),
    RawString(RRawStrLit),
    Byte(RByteLit),
    ByteString(RByteStrLit),
    RawByteString(RRawByteStrLit),
    Integer(RIntLit),
    Float(RFloatLit),
    Bool(RBoolLit),
}

/// 
/// A literal that can be signed.
/// 
#[derive(Debug, Clone)]
pub enum RSLit {
    Char(RCharLit),
    String(RStrLit),
    RawString(RRawStrLit),
    Byte(RByteLit),
    ByteString(RByteStrLit),
    RawByteString(RRawByteStrLit),
    Integer(RSIntLit),
    Float(RSFloatLit),
    Bool(RBoolLit),
}

#[derive(Debug, Clone)]
pub enum RBoolLit {
    True {
        span: Span<PPos>
    },
    False {
        span: Span<PPos>
    }
}

#[derive(Debug, Clone)]
pub enum RPathExpr {
    Path(RPathInExpr),
    Qualified(RQualPathInExpr),
}

#[derive(Debug, Clone)]
pub enum ROpExpr {
    Borrow {
        span: Span<PPos>,
        mutable: bool,
        target: RExpr,
    },
    BorrowBorrow {
        span: Span<PPos>,
        mutable: bool,
        target: RExpr,
    },
    Deref {
        span: Span<PPos>,
        target: RExpr,
    },
    ErrorProp {
        span: Span<PPos>,
        target: RExpr,
    },
    SubNegate {
        span: Span<PPos>,
        target: RExpr,
    },
    NotNegate {
        span: Span<PPos>,
        target: RExpr,
    },

    Add {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    Sub {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    Mul {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    Div {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    Mod {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    BitAnd {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    BitOr {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    BitXOr {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    LShift {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    RShift {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },

    Eq {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    NotEq {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    LessThan {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    GreaterThan {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    LessThanOrEq {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    GreaterThanOrEq {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },

    LogicOr {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    LogicAnd {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    TypeCast {
        span: Span<PPos>,
        left: RExpr,
        ty: RType,
    },
    Assign {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },

    AddAssign {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    SubAssign {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    MulAssign {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    DivAssign {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    ModAssign {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    BitAndAssign {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    BitOrAssign {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    BitXOrAssign {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    LShiftAssign {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    RShiftAssign {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
}

#[derive(Debug, Clone)]
pub struct RGroupExpr {
    pub span: Span<PPos>,
    pub expr: RExpr,
}

#[derive(Debug, Clone)]
pub struct RArrayExpr {
    pub span: Span<PPos>,
    pub elements: RArrayElements,
}

#[derive(Debug, Clone)]
pub enum RArrayElements {
    List {
        span: Span<PPos>,
        elements: Vec<RExpr>,
    },
    Duplicate {
        span: Span<PPos>,
        duplicate: RExpr,
        num_times: RExpr,
    }
}

#[derive(Debug, Clone)]
pub struct RQualPathType {
    ty: RType,
    as_ty: Option<RTypePath>
}

#[derive(Debug, Clone)]
pub struct RTypePath {
    pub span: Span<PPos>,
    pub segs: Vec<RTypePathSegment>,
}

#[derive(Debug, Clone)]
pub struct RQualPathInExpr {
    pub span: Span<PPos>,
    pub qual_path_type: RQualPathType,
    pub segs: Vec<RPathInExprSeg>,
}

#[derive(Debug, Clone)]
pub struct RQualPathInType {
    pub span: Span<PPos>,
    pub qual_path_type: RQualPathType,
    pub segs: Vec<RTypePathSegment>,
}

#[derive(Debug, Clone)]
pub struct RTypePathSegment {
    pub span: Span<PPos>,
    pub ident: RPathInExprIdent,
    pub generic_args: Option<RGenericArgs>,
    pub type_path_fn: Option<RTypePathFn>,
}

#[derive(Debug, Clone)]
pub struct RTypePathFn {
    pub span: Span<PPos>,
    pub inputs: Option<RTypeList>,
    pub ret: Option<RType>
}

#[derive(Debug, Clone)]
pub struct RAwaitExpr {
    pub span: Span<PPos>,
    pub expr: RExpr,
}

#[derive(Debug, Clone)]
pub struct RIndexExpr {
    pub span: Span<PPos>,
    pub to_index: RExpr,
    pub index_with: RExpr,
}

#[derive(Debug, Clone)]
pub struct RTupleExpr {
    pub span: Span<PPos>,
    pub elems: Vec<RExpr>,
}

#[derive(Debug, Clone)]
pub struct RTupleIndexingExpr {
    pub span: Span<PPos>,
    pub tuple: RExpr,
    pub index: RIntLit,
}

#[derive(Debug, Clone)]
pub struct RCallExpr {
    pub span: Span<PPos>,
}

#[derive(Debug, Clone)]
pub struct RMethCallExpr {
    pub span: Span<PPos>,
}

#[derive(Debug, Clone)]
pub struct RFieldExpr {
    pub span: Span<PPos>,
}

#[derive(Debug, Clone)]
pub struct RClosureExpr {
    pub span: Span<PPos>,
}

#[derive(Debug, Clone)]
pub struct RAsyncBlockExpr {
    pub span: Span<PPos>,
    pub is_move: bool,
    pub block: RBlockExpr,
}

#[derive(Debug, Clone)]
pub struct RContinueExpr {
    pub span: Span<PPos>,
    pub label: Option<RLifetime>,
}

#[derive(Debug, Clone)]
pub struct RBreakExpr {
    pub span: Span<PPos>,
    pub lifetime: Option<RLifetime>,
    pub expr: Option<RExpr>,
}

#[derive(Debug, Clone)]
pub struct RReturnExpr {
    pub span: Span<PPos>,
    pub expr: Option<RExpr>,
}

#[derive(Debug, Clone)]
pub struct RUnderscoreExpr {
    pub span: Span<PPos>,
}

#[derive(Debug, Clone)]
pub struct RMacroInvocation {
    pub span: Span<PPos>,
    pub path: RSimplePath,
    pub trees: Vec<RTokenTree>,
}

#[derive(Debug, Clone)]
pub enum RExpr {
    Lit(Box<RLit>),
    Path(Box<RPathExpr>),
    Op(Box<ROpExpr>),
    Group(Box<RGroupExpr>),
    Array(Box<RArrayExpr>),
    Await(Box<RAwaitExpr>),
    Index(Box<RIndexExpr>),
    Tuple(Box<RTupleExpr>),
    TupleIndexing(Box<RTupleIndexingExpr>),
    Struct(Box<RStructExpr>),
    Call(Box<RCallExpr>),
    MethodCall(Box<RCallExpr>),
    Field(Box<RFieldExpr>),
    Closure(Box<RClosureExpr>),
    Async(Box<RAsyncBlockExpr>),
    Continue(Box<RContinueExpr>),
    Break(Box<RBreakExpr>),
    Range(Box<RRangeExpr>),
    Return(Box<RReturnExpr>),
    Underscore(Box<RUnderscoreExpr>),
    MacroInvocation(Box<RMacroInvocation>),
    Block(Box<RBlockExpr>),
    UnsafeBlock(Box<RUnsafeBlockExpr>),
    Loop(Box<RLoopExpr>),
    If(Box<RIfExpr>),
    MatchExpr(Box<RMatchExpr>),
}


#[derive(Debug, Clone)]
pub struct RStructExpr {
    span: Span<PPos>,
}

#[derive(Debug, Clone)]
pub struct RPattern {
    pub span: Span<PPos>,
    pub patterns: Vec<RSubPattern>,
}

#[derive(Debug, Clone)]
pub enum RSubPattern {
    Lit(Box<RSLit>),
    Ident(Box<RIdentPattern>),
    Wildcard(Box<Span<PPos>>),
    Rest(Box<Span<PPos>>),
    Ref(Box<RRefPattern>),
    Struct(Box<RStructPattern>),
    TupleStruct(Box<RTupleStructPattern>),
    Tuple(Box<RTuplePattern>),
    Grouped(Box<RGroupedPattern>),
    Slice(Box<RSlicePattern>),
    Path(Box<RPathPattern>),
    Macro(Box<RMacroInvocation>),
    Range(Box<RRangePattern>),
}

#[derive(Debug, Clone)]
pub struct RTupleStructPattern {
    pub span: Span<PPos>,
    pub path: RPathInExpr,
    pub items: Vec<RPattern>,
}

#[derive(Debug, Clone)]
pub struct RTuplePattern {
    pub span: Span<PPos>,
}


#[derive(Debug, Clone)]
pub struct RGroupedPattern {
    pub span: Span<PPos>,
    pub pattern: RPattern,
}

#[derive(Debug, Clone)]
pub struct RSlicePattern {
    pub span: Span<PPos>,
    pub contents: Vec<RPattern>,
}

#[derive(Debug, Clone)]
pub struct RPathPattern {
    pub path: RPathExpr,
}

#[derive(Debug, Clone)]
pub struct RStructPattern {
    pub span: Span<PPos>,
    pub path: RPathInExpr,
    pub elems: Option<RStructPatternElems>,
}

#[derive(Debug, Clone)]
pub struct RStructPatternElems {
    pub span: Span<PPos>,
    pub fields: Vec<RStructPatternField>,
    pub et_cetera: bool,
}

#[derive(Debug, Clone)]
pub enum RStructPatternField {
    TupleMatch {
        span: Span<PPos>,
        tuple_index: RIntLit,
        pattern: RPattern,
    },
    IdentMatch {
        span: Span<PPos>,
        ident: Span<PPos>,
        pattern: RPattern,
    },
    Ident {
        span: Span<PPos>,
        reference: bool,
        mutable: bool,
        ident: Span<PPos>,
    },
    EtCetera {
        span: Span<PPos>,
    }
}

#[derive(Debug, Clone)]
pub enum RRangePattern {
    Range {
        span: Span<PPos>,
        left: RRangePatternBound,
        right: RRangePatternBound,
    },
    RangeFrom {
        span: Span<PPos>,
        left: RRangePatternBound,
    },
    RangeTo {
        span: Span<PPos>,
        right: RRangePatternBound,
    },
    RangeFull {
        span: Span<PPos>,
    },
    RangeInclusive {
        span: Span<PPos>,
        left: RRangePatternBound,
        right: RRangePatternBound,
    },
    RangeToInclusive {
        span: Span<PPos>,
        right: RRangePatternBound,
    }
}

#[derive(Debug, Clone)]
pub struct RIdentPattern {
    pub span: Span<PPos>,
    pub reference: Option<Span<PPos>>,
    pub mutable: Option<Span<PPos>>,
    pub ident: Span<PPos>,
    pub test: Option<RSubPattern>,
}

#[derive(Debug, Clone)]
pub struct RRefPattern {
    pub span: Span<PPos>,
    pub ref1: Option<Span<PPos>>,
    pub ref2: Option<Span<PPos>>,
    pub mutable: Option<Span<PPos>>,
    pub pattern: RSubPattern
}

#[derive(Debug, Clone)]
pub struct RLetStatement {
    pub span: Span<PPos>,
    pub pattern: RSubPattern,
    pub ty: Option<RType>,
    pub right: Option<(RExpr, Option<RBlockExpr>)>,
}

#[derive(Debug, Clone)]
pub enum RType {
    ImplTrait(Box<RImplTraitType>),
    TraitObj(Box<RTraitObjType>),
    Path(Box<RTypePath>),
    Tuple(Box<RTupleType>),
    Never(Box<RNeverType>),
    Ptr(Box<RPtrType>),
    Ref(Box<RRefType>),
    Array(Box<RArrayType>),
    Slice(Box<RSliceType>),
    Inferred(Box<RInferredType>),
    QualPath(Box<RQualPathInType>),
    BareFn(Box<RBareFnType>),
    Macro(Box<RMacroInvocation>),
}

#[derive(Debug, Clone)]
pub struct RInferredType {
    pub span: Span<PPos>,
}

#[derive(Debug, Clone)]
pub struct RTypeParam {
    pub span: Span<PPos>,
    pub ident: Span<PPos>,
    pub bounds: Option<RTypeParamBounds>,
    pub ty: Option<RType>,
}

#[derive(Debug, Clone)]
pub struct RTypeParamBounds {
    pub span: Span<PPos>,
    pub bounds: Vec<RTypeParamBound>,
}

#[derive(Debug, Clone)]
pub enum RTypeParamBound {
    Lifetime(RLifetime),
    Trait(RTraitBound)
}

#[derive(Debug, Clone)]
pub struct RTraitObjType {
    pub span: Span<PPos>,
}


#[derive(Debug, Clone)]
pub struct RLifetimeBounds {
    pub span: Span<PPos>,
    pub bounds: Vec<RLifetime>,
}

#[derive(Debug, Clone)]
pub struct RImplTraitType {
    pub span: Span<PPos>,
}

#[derive(Debug, Clone)]
pub enum RConstParam {
    Decl {
        span: Span<PPos>,
        ident: Span<PPos>,
        ty: RType,
    },
    Block {
        span: Span<PPos>,
        ident: Span<PPos>,
        ty: RType,
        expr: RBlockExpr,
    },
    Id {
        span: Span<PPos>,
        ident: Span<PPos>,
        ty: RType,
        right_ident: Span<PPos>,
    },
    Lit {
        span: Span<PPos>,
        ident: Span<PPos>,
        ty: RType,
        neg: bool,
        lit: RLit,
    }
}

#[derive(Debug, Clone)]
pub struct RTupleType {
    pub span: Span<PPos>,
    pub children: Vec<RType>,
}

#[derive(Debug, Clone)]
pub struct RArrayType {
    pub span: Span<PPos>,
    pub child: RType,
    pub expr: RExpr,
}

#[derive(Debug, Clone)]
pub struct RSliceType {
    pub span: Span<PPos>,
    pub child: RType,
}

#[derive(Debug, Clone)]
pub struct RTraitBound {
    pub span: Span<PPos>,
    pub not: bool,
}

#[derive(Debug, Clone)]
pub struct RNeverType {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
}

#[derive(Debug, Clone)]
pub struct RPtrType {
    pub span: Span<PPos>,
    pub mutable: bool,
    pub ty: RType,
}

#[derive(Debug, Clone)]
pub struct RRefType {
    pub span: Span<PPos>,
    pub lifetime: Option<RLifetime>,
    pub mutable: bool,
    pub ty: RType,
}


#[derive(Debug, Clone)]
pub struct RToken {
    pub span: Span<PPos>
}

#[derive(Debug, Clone)]
pub enum RTokenTree {
    Token(RToken),
    DelimTokenTree(Box<RDelimTokenTree>),
}

#[derive(Debug, Clone)]
pub struct RDelimTokenTree {
    pub span: Span<PPos>,
    pub trees: Vec<RTokenTree>,
}

#[derive(Debug, Clone)]
pub struct RConfigAttr {
    pub span: Span<PPos>,
    pub ident: Span<PPos>,
    pub string: RString,
}

/// 
/// A configuration predicate.
/// 
#[derive(Debug, Clone)]
pub enum RConfigPred {
    /// The given config option must either exist or match the given string.
    Option {
        span: Span<PPos>,
        /// The identifier of the configuration option to check.
        ident: Span<PPos>,
        /// The string to check the config option against (if `None`, then it is a check to simply see if the config option is non-null).
        string: Option<RString>,
    },
    /// A list of predicates where all of them must be true.
    All {
        span: Span<PPos>,
        preds: Vec<Self>,
    },
    /// A list of predicates where at least one of them must be true.
    Any {
        span: Span<PPos>,
        preds: Vec<Self>,
    },
    /// A list of predicates where the given predicate must not be true.
    Not {
        span: Span<PPos>,
        pred: Box<Self>
    },
}

#[derive(Debug, Clone)]
pub struct RCharLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
    pub suffix: Option<Span<PPos>>,
}

#[derive(Debug, Clone)]
pub struct RByteStrLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
    pub suffix: Option<Span<PPos>>,
}

#[derive(Debug, Clone)]
pub struct RRawByteStrLit {
    pub span: Span<PPos>,
    pub text: Span<PPos>,
    pub suffix: Option<Span<PPos>>,
}

#[derive(Debug, Clone)]
pub struct RByteLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
    pub suffix: Option<Span<PPos>>
}

#[derive(Debug, Clone)]
pub struct RFloatLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
    pub exp: Option<Span<PPos>>,
    pub value_exp_span: Span<PPos>,
    pub suffix: Option<Span<PPos>>
}

#[derive(Debug, Clone)]
pub struct RSFloatLit {
    pub span: Span<PPos>,
    pub neg: bool,
    pub lit: RFloatLit,
}

#[derive(Debug, Clone)]
pub struct RDecLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
}

#[derive(Debug, Clone)]
pub struct RBinLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
}

#[derive(Debug, Clone)]
pub struct ROctLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
}

#[derive(Debug, Clone)]
pub struct RHexLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
}

#[derive(Debug, Clone)]
pub enum RIntLit {
    DecLit {
        span: Span<PPos>,
        lit: RDecLit,
        suffix: Option<Span<PPos>>,
    },
    BinLit {
        span: Span<PPos>,
        lit: RBinLit,
        suffix: Option<Span<PPos>>,
    },
    OctLit {
        span: Span<PPos>,
        lit: ROctLit,
        suffix: Option<Span<PPos>>,
    },
    HexLit {
        span: Span<PPos>,
        lit: RHexLit,
        suffix: Option<Span<PPos>>,
    },
}

#[derive(Debug, Clone)]
pub struct RSIntLit {
    span: Span<PPos>,
    neg: bool,
    lit: RIntLit
}

#[derive(Debug, Clone)]
pub enum RLifetime {
    Static {
        span: Span<PPos>,
        stat: Span<PPos>,
    },
    Elided {
        span: Span<PPos>,
        underscore: Span<PPos>,
    },
    Ident {
        span: Span<PPos>,
        ident: Span<PPos>,
    }
}

/// 
/// A string where escape sequences are allowed.
/// 
#[derive(Debug, Clone)]
pub struct RStrLit {
    pub span: Span<PPos>,
    pub text: Span<PPos>,
    pub suffix: Option<Span<PPos>>,
}

/// 
/// A string where no escape sequences are allowed i.e. the string is
/// exactly as it looks.
/// 
#[derive(Debug, Clone)]
pub struct RRawStrLit {
    pub span: Span<PPos>,
    pub text: Span<PPos>,
    pub suffix: Option<Span<PPos>>,
}


#[derive(Debug, Clone)]
pub struct RTypeList {
    pub span: Span<PPos>,
    pub types: Vec<RType>,
}

#[derive(Debug, Clone)]
pub struct RLoopExpr {
    span: Span<PPos>,
    label: Option<RLifetime>,
    ty: RLoop,
}

#[derive(Debug, Clone)]
pub struct RBlockExpr {
    pub span: Span<PPos>,
    statements: Vec<RStatement>
}

#[derive(Debug, Clone)]
pub struct RUnsafeBlockExpr {
    pub span: Span<PPos>,
    pub block: RBlockExpr,
}

#[derive(Debug, Clone)]
pub enum RLoop {
    Infinite {
        span: Span<PPos>,
        body: RBlockExpr,
    },
    While {
        span: Span<PPos>,
        expr: RExpr,
        body: RBlockExpr,
    },
    WhileLet {
        span: Span<PPos>,
        pattern: RPattern,
        expr: RExpr,
        body: RBlockExpr,
    },
    For {
        span: Span<PPos>,
        pattern: RPattern,
        expr: RExpr,
        body: RBlockExpr,
    },
    Expr {
        span: Span<PPos>,
        body: RBlockExpr,
    }
}


#[derive(Debug, Clone)]
pub enum RRangeExpr {
    Range {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    RangeFrom {
        span: Span<PPos>,
        left: RExpr,
    },
    RangeTo {
        span: Span<PPos>,
        right: RExpr,
    },
    RangeFull {
        span: Span<PPos>,
    },
    RangeInclusive {
        span: Span<PPos>,
        left: RExpr,
        right: RExpr,
    },
    RangeToInclusive {
        span: Span<PPos>,
        right: RExpr,
    }
}


#[derive(Debug, Clone)]
pub enum RIfExpr {
    If {
        span: Span<PPos>,
        expr: RExpr,
        body: RBlockExpr,
    },
    IfElse {
        span: Span<PPos>,
        expr: RExpr,
        body: RBlockExpr,
        else_body: Box<RIfExpr>,
    },
    IfLet {
        span: Span<PPos>,
        pattern: RPattern,
        expr: RExpr,
        body: RBlockExpr,
    },
    IfLetElse {
        span: Span<PPos>,
        pattern: RPattern,
        expr: RExpr,
        body: RBlockExpr,
        else_body: Box<RIfExpr>,
    },
    BlockExpr(RBlockExpr)
}


#[derive(Debug, Clone)]
pub enum RRangePatternBound {
    Char(RCharLit),
    Byte(RByteLit),
    Int {
        span: Span<PPos>,
        neg_sign: bool,
        int: RIntLit,
    },
    Float {
        span: Span<PPos>,
        neg_sign: bool,
        float: RFloatLit,
    },
    Path {
        path: RPathExpr,
    }
}

#[derive(Debug, Clone)]
pub struct RMod {
    pub span: Span<PPos>,
    pub is_unsafe: bool,
    pub ident: Span<PPos>,
    pub items: Vec<RItem>,
}

#[derive(Debug, Clone)]
pub struct RExternCrate {
    pub span: Span<PPos>,
    pub crate_ref: Span<PPos>,
    pub as_clause: Option<Span<PPos>>,
}

#[derive(Debug, Clone)]
pub struct RUseDecl {
    pub span: Span<PPos>,
    pub tree: RUseTree,
}

#[derive(Debug, Clone)]
pub enum RUseTree {
    All {
        span: Span<PPos>,
        path: Option<RSimplePath>,
    },
    List {
        span: Span<PPos>,
        path: Option<RSimplePath>,
        list: Vec<Self>,
    },
    As {
        span: Span<PPos>,
        path: RSimplePath,
        ident: Option<Span<PPos>>
    }
}

#[derive(Debug, Clone)]
pub struct RBareFnType {
    pub span: Span<PPos>,
}

#[derive(Debug, Clone)]
pub enum RVisItem {
    Mod {
        span: Span<PPos>,
        vis: Option<RVis>,
        val: RMod,
    },
    ExternCrate {
        span: Span<PPos>,
        vis: Option<RVis>,
        val: RExternCrate,
    },
    UseDecl {
        span: Span<PPos>,
        vis: Option<RVis>,
        val: RUseDecl,
    },
    Fn {
        span: Span<PPos>,
        vis: Option<RVis>,
        val: RFn,
    },
    TypeAlias {
        span: Span<PPos>,
        vis: Option<RVis>,
        val: RTypeAlias,
    },
    Struct {
        span: Span<PPos>,
        vis: Option<RVis>,
        val: RStruct,
    },
    Enum {
        span: Span<PPos>,
        vis: Option<RVis>,
        val: REnum,
    },
    Union {
        span: Span<PPos>,
        vis: Option<RVis>,
        val: RUnion,
    },
    Const {
        span: Span<PPos>,
        vis: Option<RVis>,
        val: RConstItem,
    },
    Static {
        span: Span<PPos>,
        vis: Option<RVis>,
        val: RStaticItem,
    },
    Trait {
        span: Span<PPos>,
        vis: Option<RVis>,
        val: RTrait,
    },
    Impl {
        span: Span<PPos>,
        vis: Option<RVis>,
        val: RImpl,
    },
    ExternBlock {
        span: Span<PPos>,
        vis: Option<RVis>,
        val: RExternBlock,
    },
}

#[derive(Debug, Clone)]
pub enum RMacroItem {
    Invocation(RMacroInvocation),
    MacroDef(RMacroDef),
}

/// 
/// An item in a crate.
/// 
#[derive(Debug, Clone)]
pub enum RItem {
    VisItem(RVisItem),
    MacroItem(RMacroItem),
}

/// 
/// A parsed Rust string.
/// 
#[derive(Debug, Clone)]
pub enum RString {
    Str(RStrLit),
    Raw(RRawStrLit),
}

#[derive(Debug, Clone)]
pub enum Type {
    /// A tuple type such as `(&str, 2, 0.0)`
    Tuple {
        span: Span<PPos>,
        children: Vec<Self>,
    },
    /// A bounded type like in an impl statement.
    Bounded {
        span: Span<PPos>,
    }
}

#[derive(Debug, Clone)]
pub struct LifeTime {
    span: Span<PPos>,
    ident: Span<PPos>,
}

#[derive(Debug, Clone)]
/// The visibility of an item.
pub enum RVis {
    VisPub {
        span: Span<PPos>,
    },
    VisCrate {
        span: Span<PPos>,
    },
    VisSelf {
        span: Span<PPos>,
    },
    VisSuper {
        span: Span<PPos>,
    },
    VisPath {
        span: Span<PPos>,
        path: RSimplePath,
    }
}

#[derive(Debug, Clone)]
pub struct RAttr {
    span: Span<PPos>,
}

/// 
/// A comment in Rust.
/// 
#[derive(Debug, Clone)]
pub enum RComment {
    Line {
        /// Span including "//"
        span: Span<PPos>,
        /// Span of just the comment text.
        text: Span<PPos>,
    },
    Block {
        /// Span including "/*" and "*/"
        span: Span<PPos>,
        /// The comment's text.
        text: Span<PPos>,
    },
    InnerLineDoc {
        /// Span including "//!".
        span: Span<PPos>,
        /// The doc comment's text.
        text: Span<PPos>,
    },
    InnerBlockDoc {
        /// Span including "/*!" and "*/".
        span: Span<PPos>,
        /// The doc comment's text.
        text: Span<PPos>,
    },
    OuterLineDoc {
        /// Span including "///".
        span: Span<PPos>,
        /// The doc comment's text.
        text: Span<PPos>,
    },
    OuterBlockDoc {
        /// Span including "/**" and "*/".
        span: Span<PPos>,
        /// The doc comment's text.
        text: Span<PPos>,
    },
}

#[derive(Debug, Clone)]
pub struct RPathInExpr {
    pub span: Span<PPos>,
    pub path: Vec<RPathInExprSeg>,
}

#[derive(Debug, Clone)]
pub struct RPathInExprSeg {
    pub span: Span<PPos>,
    pub ident: RPathInExprIdent,
    pub generics: RGenericArgs,
}

#[derive(Debug, Clone)]
pub struct RPathInExprIdent {
    pub span: Span<PPos>,
}

#[derive(Debug, Clone)]
pub struct RGenericArgs {
    pub span: Span<PPos>,
    pub args: Vec<RGenericArg>,
}

#[derive(Debug, Clone)]
pub enum RGenericArg {
    Lifetime(RLifetime),
    Type(RType),
    GenericConstArg(RGenericArgsConst),
    GenericConstArgsBinding(RGenericArgsBinding),
}

#[derive(Debug, Clone)]
pub enum RGenericArgsConst {
    Block(RBlockExpr),
    Lit(RLit),
    SLit(RSLit),
    PathSeg(Span<PPos>),
}

#[derive(Debug, Clone)]
pub struct RGenericArgsBinding {
    pub span: Span<PPos>,
    pub ident: Span<PPos>,
    pub ty: RType,
}

/// 
/// A simple path.
/// 
#[derive(Debug, Clone)]
pub struct RSimplePath {
    pub span: Span<PPos>,
    /// A number of path segments (a path segment is: identifier | "super" | "self" | "crate" | "$crate")
    pub path: Vec<Span<PPos>>,
}


/// 
/// A function that parses the given file's worth of text and returns the created parse tree.
/// 
pub fn parse_file(file: &str) -> ParseResult<RCrate, String, PPos> {

    // define function to produce "panic" uniform messages of parse
    let panic_fn = |pos: Span<PPos>, fn_name: &str, message: &str| -> String {
        format!("{}: ({}) {}", pos, fn_name, message)
    };
    let panic = &panic_fn;

    // strict keywords
    const KW_AS: &str = "as";
    const KW_BREAK: &str = "break";
    const KW_CONST: &str = "const";
    const KW_CONTINUE: &str = "continue";
    const KW_CRATE: &str = "crate";
    const KW_ELSE: &str = "else";
    const KW_ENUM: &str = "enum";
    const KW_EXTERN: &str = "extern";
    const KW_FALSE: &str = "false";
    const KW_FN: &str = "fn";
    const KW_FOR: &str = "for";
    const KW_IF: &str = "if";
    const KW_IMPL: &str = "impl";
    const KW_IN: &str = "in";
    const KW_LET: &str = "let";
    const KW_LOOP: &str = "loop";
    const KW_MATCH: &str = "match";
    const KW_MOD: &str = "mod";
    const KW_MOVE: &str = "move";
    const KW_MUT: &str = "mut";
    const KW_PUB: &str = "pub";
    const KW_REF: &str = "ref";
    const KW_RETURN: &str = "return";
    const KW_SELFVALUE: &str = "self";
    const KW_SELFTYPE: &str = "Self";
    const KW_STATIC: &str = "static";
    const KW_STRUCT: &str = "struct";
    const KW_SUPER: &str = "super";
    const KW_TRAIT: &str = "trait";
    const KW_TRUE: &str = "true";
    const KW_TYPE: &str = "type";
    const KW_UNSAFE: &str = "unsafe";
    const KW_USE: &str = "use";
    const KW_WHERE: &str = "where";
    const KW_WHILE: &str = "while";
    
    // 2018+  strict keywords
    const KW_ASYNC: &str = "async";
    const KW_AWAIT: &str = "await";
    const KW_DYN: &str = "dyn";

    let strict_keywords = OneOf(
        [KW_AS, KW_BREAK, KW_CONST, KW_CONTINUE, KW_CRATE, KW_ELSE, KW_ENUM, KW_EXTERN, KW_FALSE, KW_FN, KW_FOR, KW_IF, KW_IMPL, KW_IN, KW_LET, KW_LOOP, KW_MATCH, KW_MOD, KW_MOVE, KW_MUT, KW_PUB, KW_REF, KW_RETURN, KW_SELFVALUE, KW_SELFTYPE, KW_STATIC, KW_STRUCT, KW_SUPER, KW_TRAIT, KW_TRUE, KW_TYPE, KW_UNSAFE, KW_USE, KW_WHERE, KW_WHILE, KW_ASYNC, KW_AWAIT, KW_DYN]
    );
    let strict_keywords = strict_keywords;

    // reserved keywords
    const KW_ABSTRACT: &str = "abstract";
    const KW_BECOME: &str = "become";
    const KW_BOX: &str = "box";
    const KW_DO: &str = "do";
    const KW_FINAL: &str = "final";
    const KW_MACRO: &str = "macro";
    const KW_OVERRIDE: &str = "override";
    const KW_PRIV: &str = "priv";
    const KW_TYPEOF: &str = "typeof";
    const KW_UNSIZED: &str = "unsized";
    const KW_VIRTUAL: &str = "virtual";
    const KW_YIELD: &str = "yield";

    // 2018+ reserved keywords
    const KW_TRY: &str = "try";

    let reserved_keywords = OneOf(
        [KW_ABSTRACT, KW_BECOME, KW_BOX, KW_DO, KW_FINAL, KW_MACRO, KW_OVERRIDE, KW_PRIV, KW_TYPEOF, KW_UNSIZED, KW_VIRTUAL, KW_YIELD, KW_TRY]
    );
    let reserved_keywords: &dyn ParseNode<Span<PPos>, String, _, PPos, char> = &reserved_keywords;

    // weak keywords
//    const KW_MACRO_RULES: &str = "macro_rules";
//    const KW_UNION: &str = "union";
//    const KW_STATIC_LIFETIME: &str = "'static";

//    let weak_keywords = OneOf(
//        [KW_MACRO_RULES, KW_UNION, KW_STATIC_LIFETIME]
//    );
//    let weak_keywords: &dyn ParseNode<Span<PPos>, String, _, PPos, char> = &weak_keywords;

    let isolated_cr = SpanOf(('\r', Not('\n')));
    let isolated_cr = &isolated_cr;

    // IDENTIFIERS

    // unicode groups
    let xid_start = MapPValue(|span, ch| {
        if UnicodeXID::is_xid_start(ch) {
            Okay(span.clone(), span.end)
        } else {
            Error(format!("{}: (XID_START) expected character in the [:XID_Start:] unicode group", span.start))
        }
    });
    let xid_start = &xid_start;

    let xid_continue = MapPValue(|span, ch| {
        if UnicodeXID::is_xid_continue(ch) {
            Okay(span.clone(), span.end)
        } else {
            Error(format!("{}: (XID_CONTINUE) expected character in the [:XID_Continue:] unicode group", span.start))
        }
    });
    let xid_continue = &xid_continue;

    let identifier_or_keyword =
        SpanOf(OneOf2(
            (xid_start, ZeroOrMore(xid_continue)),
            ('_', OneOrMore(xid_continue)),
        ));
    let identifier_or_keyword = &identifier_or_keyword;

    let raw_identifier = SpanOf(Leader("r#", (Not(OneOf(["crate", "self", "super", "Self"])), identifier_or_keyword), |_, span, _| panic(span, "raw_identifier", "expected identifier after this `r#` raw identifier signiture")));
    let raw_identifier = &raw_identifier;

    let non_keyword_identifier = SpanOf((Not(OneOf2(strict_keywords, reserved_keywords)), identifier_or_keyword));
    let non_keyword_identifier = &non_keyword_identifier;

    let identifier = SpanOf(OneOf2(non_keyword_identifier, raw_identifier));
    let identifier = &identifier;

    srule!(comment, comment_rule);
    srule!(generic_arg, generic_arg_rule);
    srule!(generic_args_const, generic_args_const_rule);
    srule!(generic_args_binding, generic_args_binding_rule);
    srule!(where_clause, where_clause_rule);
    srule!(where_clause_item, where_clause_item_rule);
    srule!(lifetime_where_clause_item, lifetime_where_clause_item_rule);
    srule!(type_bound_where_clause_item, type_bound_where_clause_item_rule);
    srule!(range_pattern, range_pattern_rule);
    srule!(path_expr_segment, path_expr_segment_rule);
    srule!(deref_expression, deref_expression_rule);
    srule!(path_expression, path_expression_rule);
    srule!(path_in_expression, path_in_expression_rule);
    srule!(unsafe_block_expression, unsafe_block_expression_rule);
    srule!(async_block_expression, async_block_expression_rule);
    srule!(underscore_expression, underscore_expression_rule);
    srule!(lifetime_bounds, lifetime_bounds_rule);
    srule!(_crate, _crate_rule);
    srule!(_type, _type_rule);
    srule!(configuration_option, configuration_option_rule);
    srule!(type_no_bounds, type_no_bounds_rule);
    srule!(impl_trait_type, impl_trait_type_rule);
    srule!(trait_object_type, trait_object_type_rule);
    srule!(parenthesized_type, parenthesized_type_rule);
    srule!(impl_trait_type_one_bound, impl_trait_type_one_bound_rule);
    srule!(trait_object_type_one_bound, trait_object_type_one_bound_rule);
    srule!(type_path, type_path_rule);
    srule!(tuple_type, tuple_type_rule);
    srule!(never_type, never_type_rule);
    srule!(raw_pointer_type, raw_pointer_type_rule);
    srule!(reference_type, reference_type_rule);
    srule!(array_type, array_type_rule);
    srule!(slice_type, slice_type_rule);
    srule!(inferred_type, inferred_type_rule);
    srule!(macro_invocation, macro_invocation_rule);
    srule!(type_path_segment, type_path_segment_rule);
    srule!(type_path_fn, type_path_fn_rule);
    srule!(type_path_fn_inputs, type_path_fn_inputs_rule);
    srule!(path_ident_segment, path_ident_segment_rule);
    srule!(generic_args, generic_args_rule);
    srule!(qualified_path_in_expression, qualified_path_in_expression_rule);
    srule!(qualified_path_type, qualified_path_type_rule);
    srule!(qualified_path_in_type, qualified_path_in_type_rule);
    srule!(bare_function_type, bare_function_type_rule);
    srule!(function_type_qualifiers, function_type_qualifiers_rule);
    srule!(bare_function_return_type, bare_function_return_type_rule);
    srule!(function_parameters_maybe_named_variadic, function_parameters_maybe_named_variadic_rule);
    srule!(maybe_named_function_parameters, maybe_named_function_parameters_rule);
    srule!(maybe_named_param, maybe_named_param_rule);
    srule!(maybe_named_function_parameters_variadic, maybe_named_function_parameters_variadic_rule);
    srule!(delim_token_tree, delim_token_tree_rule, RDelimTokenTree, String, _, PPos, char);
    srule!(token_tree, token_tree_rule);
    srule!(macro_invocation_semi, macro_invocation_semi_rule);
    srule!(block_comment_or_doc, block_comment_or_doc_rule);
    srule!(line_comment, line_comment_rule);
    srule!(block_comment, block_comment_rule);
    srule!(inner_line_doc, inner_line_doc_rule);
    srule!(inner_block_doc, inner_block_doc_rule);
    srule!(outer_line_doc, outer_line_doc_rule);
    srule!(outer_block_doc, outer_block_doc_rule);
    srule!(suffix, suffix_rule);
    srule!(suffix_no_e, suffix_no_e_rule);
    srule!(integer_literal, integer_literal_rule);
    srule!(dec_literal, dec_literal_rule);
    srule!(bin_literal, bin_literal_rule);
    srule!(oct_literal, oct_literal_rule);
    srule!(hex_literal, hex_literal_rule);
    srule!(bin_digit, bin_digit_rule);
    srule!(oct_digit, oct_digit_rule);
    srule!(dec_digit, dec_digit_rule);
    srule!(hex_digit, hex_digit_rule);
    srule!(tuple_index, tuple_index_rule);
    srule!(char_literal, char_literal_rule);
    srule!(quote_escape, quote_escape_rule);
    srule!(ascii_escape, ascii_escape_rule);
    srule!(unicode_escape, unicode_escape_rule);
    srule!(float_literal, float_literal_rule);
    srule!(float_exponent, float_exponent_rule);
    srule!(reserved_number, reserved_number_rule);
    srule!(string_literal, string_literal_rule);
    srule!(string_continue, string_continue_rule);
    srule!(raw_string_literal, raw_string_literal_rule);
    srule!(raw_string_content, raw_string_content_rule);
    srule!(byte_literal, byte_literal_rule);
    srule!(ascii_for_char, ascii_for_char_rule);
    srule!(byte_escape, byte_escape_rule);
    srule!(byte_string_literal, byte_string_literal_rule);
    srule!(ascii_for_string, ascii_for_string_rule);
    srule!(raw_byte_string_literal, raw_byte_string_literal_rule);
    srule!(raw_byte_string_content, raw_byte_string_content_rule);
    srule!(ascii, ascii_rule);
    srule!(lifetime_token, lifetime_token_rule);
    srule!(lifetime_or_label, lifetime_or_label_rule);
    srule!(reserved_token_double_quote, reserved_token_double_quote_rule);
    srule!(reserved_token_single_quote, reserved_token_single_quote_rule);
    srule!(reserved_token_pound, reserved_token_pound_rule);
    srule!(simple_path, simple_path_rule);
    srule!(simple_path_segment, simple_path_segment_rule);
    srule!(macro_rules_definition, macro_rules_definition_rule);
    srule!(macro_rules_def, macro_rules_def_rule);
    srule!(macro_rules, macro_rules_rule);
    srule!(macro_rule, macro_rule_rule);
    srule!(macro_matcher, macro_matcher_rule);
    srule!(macro_frag_spec, macro_frag_spec_rule);
    srule!(macro_rep_sep, macro_rep_sep_rule);
    srule!(macro_rep_op, macro_rep_op_rule);
    srule!(macro_transcriber, macro_transcriber_rule);
    srule!(configuration_predicate, configuration_predicate_rule);
    srule!(configuration_all, configuration_all_rule);
    srule!(configuration_any, configuration_any_rule);
    srule!(configuration_not, configuration_not_rule);
    srule!(configuration_predicate_list, configuration_predicate_list_rule);
    srule!(cfg_attribute, cfg_attribute_rule);
    srule!(cfg_attr_attribute, cfg_attr_attribute_rule);
    srule!(cfg_attrs, cfg_attrs_rule);
    srule!(item, item_rule);
    srule!(vis_item, vis_item_rule);
    srule!(macro_item, macro_item_rule);
    srule!(module, module_rule);
    srule!(extern_crate, extern_crate_rule);
    srule!(crate_ref, crate_ref_rule);
    srule!(as_clause, as_clause_rule);
    srule!(use_declaration, use_declaration_rule);
    srule!(use_tree, use_tree_rule);
    srule!(function, function_rule);
    srule!(abi, abi_rule);
    srule!(function_parameters, function_parameters_rule);
    srule!(self_param, self_param_rule);
    srule!(function_param, function_param_rule);
    srule!(function_param_pattern, function_param_pattern_rule);
    srule!(function_return_type, function_return_type_rule);
    srule!(type_alias, type_alias_rule);
    srule!(_struct, _struct_rule);
    srule!(struct_struct, struct_struct_rule);
    srule!(tuple_struct, tuple_struct_rule);
    srule!(struct_fields, struct_fields_rule);
    srule!(struct_field, struct_field_rule);
    srule!(tuple_fields, tuple_fields_rule);
    srule!(tuple_field, tuple_field_rule);
    srule!(enumeration, enumeration_rule);
    srule!(enum_items, enum_items_rule);
    srule!(enum_item, enum_item_rule);
    srule!(enum_item_tuple, enum_item_tuple_rule);
    srule!(enum_item_struct, enum_item_struct_rule);
    srule!(enum_item_discriminant, enum_item_discriminant_rule);
    srule!(_union, _union_rule);
    srule!(constant_item, constant_item_rule);
    srule!(static_item, static_item_rule);
    srule!(_trait, _trait_rule);
    srule!(implementation, implementation_rule);
    srule!(inherent_impl, inherent_impl_rule);
    srule!(trait_impl, trait_impl_rule);
    srule!(extern_block, extern_block_rule);
    srule!(external_item, external_item_rule);
    srule!(generic_params, generic_params_rule);
    srule!(generic_param, generic_param_rule);
    srule!(lifetime_param, lifetime_param_rule);
    srule!(type_param, type_param_rule);
    srule!(const_param, const_param_rule);
    srule!(associated_item, associated_item_rule);
    srule!(inner_attribute, inner_attribute_rule);
    srule!(outer_attribute, outer_attribute_rule);
    srule!(attr, attr_rule);
    srule!(attr_input, attr_input_rule);
    srule!(meta_item, meta_item_rule);
    srule!(meta_seq, meta_seq_rule);
    srule!(meta_item_inner, meta_item_inner_rule);
    srule!(meta_word, meta_word_rule);
    srule!(meta_name_value_str, meta_name_value_str_rule);
    srule!(meta_list_paths, meta_list_paths_rule);
    srule!(meta_list_idents, meta_list_idents_rule);
    srule!(statement, statement_rule);
    srule!(let_statement, let_statement_rule);
    srule!(expression_statement, expression_statement_rule);
    srule!(expression, expression_rule);
    srule!(expression_without_block, expression_without_block_rule);
    srule!(expression_with_block, expression_with_block_rule);
    srule!(literal_expression, literal_expression_rule);
    srule!(block_expression, block_expression_rule);
    srule!(statements, statements_rule);
    srule!(operator_expression, operator_expression_rule);
    srule!(borrow_expression, borrow_expression_rule);
    srule!(error_propogation_expression, error_propogation_expression_rule);
    srule!(negation_expression, negation_expression_rule);
    srule!(arithmetic_or_logical_expression, arithmetic_or_logical_expression_rule);
    srule!(comparison_expression, comparison_expression_rule);
    srule!(lazy_boolean_expression, lazy_boolean_expression_rule);
    srule!(type_cast_expression, type_cast_expression_rule);
    srule!(assignment_expression, assignment_expression_rule);
    srule!(compound_assignment_expression, compound_assignment_expression_rule);
    srule!(grouped_expression, grouped_expression_rule);
    srule!(array_expression, array_expression_rule);
    srule!(array_elements, array_elements_rule);
    srule!(index_expression, index_expression_rule);
    srule!(tuple_expression, tuple_expression_rule);
    srule!(tuple_elements, tuple_elements_rule);
    srule!(tuple_indexing_expression, tuple_indexing_expression_rule);
    srule!(struct_expression, struct_expression_rule);
    srule!(struct_expr_struct, struct_expr_struct_rule);
    srule!(struct_expr_fields, struct_expr_fields_rule);
    srule!(struct_expr_field, struct_expr_field_rule);
    srule!(struct_base, struct_base_rule);
    srule!(struct_expr_tuple, struct_expr_tuple_rule);
    srule!(struct_expr_unit, struct_expr_unit_rule);
    srule!(call_expression, call_expression_rule);
    srule!(call_params, call_params_rule);
    srule!(method_call_expression, method_call_expression_rule);
    srule!(field_expression, field_expression_rule);
    srule!(closure_expression, closure_expression_rule);
    srule!(closure_parameters, closure_parameters_rule);
    srule!(closure_param, closure_param_rule);
    srule!(loop_expression, loop_expression_rule);
    srule!(infinite_loop_expression, infinite_loop_expression_rule);
    srule!(predicate_loop_expression, predicate_loop_expression_rule);
    srule!(predicate_pattern_loop_expression, predicate_pattern_loop_expression_rule);
    srule!(iterator_loop_expression, iterator_loop_expression_rule);
    srule!(loop_label, loop_label_rule);
    srule!(break_expression, break_expression_rule);
    srule!(label_block_expression, label_block_expression_rule);
    srule!(continue_expression, continue_expression_rule);
    srule!(range_expression, range_expression_rule);
    srule!(range_expr, range_expr_rule);
    srule!(range_from_expr, range_from_expr_rule);
    srule!(range_to_expr, range_to_expr_rule);
    srule!(range_full_expr, range_full_expr_rule);
    srule!(range_inclusive_expr, range_inclusive_expr_rule);
    srule!(range_to_inclusive_expr, range_to_inclusive_expr_rule);
    srule!(if_expression, if_expression_rule);
    srule!(if_let_expression, if_let_expression_rule);
    srule!(match_expression, match_expression_rule);
    srule!(scrutinee, scrutinee_rule);
    srule!(match_arms, match_arms_rule);
    srule!(match_arm, match_arm_rule);
    srule!(match_arm_gaurd, match_arm_gaurd_rule);
    srule!(return_expression, return_expression_rule);
    srule!(await_expression, await_expression_rule);
    srule!(pattern, pattern_rule);
    srule!(pattern_no_top_alt, pattern_no_top_alt_rule);
    srule!(pattern_without_range, pattern_without_range_rule);
    srule!(literal_pattern, literal_pattern_rule);
    srule!(identifier_pattern, identifier_pattern_rule);
    srule!(wildcard_pattern, wildcard_pattern_rule);
    srule!(rest_pattern, rest_pattern_rule);
    srule!(range_inclusive_pattern, range_inclusive_pattern_rule);
    srule!(range_from_pattern, range_from_pattern_rule);
    srule!(range_to_inclusive_pattern, range_to_inclusive_pattern_rule);
    srule!(obsolete_range_pattern, obsolete_range_pattern_rule);
    srule!(range_pattern_bound, range_pattern_bound_rule);
    srule!(reference_pattern, reference_pattern_rule);
    srule!(struct_pattern, struct_pattern_rule);
    srule!(struct_pattern_elements, struct_pattern_elements_rule);
    srule!(struct_pattern_fields, struct_pattern_fields_rule);
    srule!(struct_pattern_field, struct_pattern_field_rule);
    srule!(struct_pattern_et_cetera, struct_pattern_et_cetera_rule);
    srule!(tuple_struct_pattern, tuple_struct_pattern_rule);
    srule!(tuple_struct_items, tuple_struct_items_rule);
    srule!(tuple_pattern, tuple_pattern_rule);
    srule!(tuple_pattern_items, tuple_pattern_items_rule);
    srule!(grouped_pattern, grouped_pattern_rule);
    srule!(slice_pattern, slice_pattern_rule);
    srule!(slice_pattern_items, slice_pattern_items_rule);
    srule!(path_pattern, path_pattern_rule);
    srule!(type_param_bounds, type_param_bounds_rule);
    srule!(type_param_bound, type_param_bound_rule);
    srule!(trait_bound, trait_bound_rule);
    srule!(lifetime, lifetime_rule);
    srule!(for_lifetimes, for_lifetimes_rule);
    srule!(visibility, visibility_rule);
    srule!(token, token_rule);
    srule!(delimiter, delimiter_rule);
    srule!(punctuation, punctuation_rule);
    srule!(macro_match, macro_match_rule);

    // whitespace

    // a single white-space character or comment
    let wc = SpanOf(OneOf3(0..=32u32, 127u32, comment));
    // zero or more whitespace
    let w = SpanOf(ZeroOrMore(wc.clone()));
    // one or more whitespace
    let o = SpanOf(OneOrMore(wc.clone()));

    let w = &w;
    let o = &o;

    // --- Tokens ---

    punctuation_rule.set(
        SpanOf(OneOf([
            "+", "-", "*", "/", "%", "^", "!", "&", "|", "&&", "||", "<<", ">>", "+=", "-=", "*=", "/=", "%=", "^=", "&=", "|=", "<<=", ">>=", "=", "==", "!=", ">", "<", ">=", "<=", "@", "_", ".", "..", "...", "..=", ",", ";", ":", "::", "->", "=>", "#", "$", "?", "~"
        ])),
    );

    delimiter_rule.set(MapV(
        OneOf3(
            ('(', w, Join(token_tree, w), w, ')'),
            ('[', w, Join(token_tree, w), w, ']'),
            ('{', w, Join(token_tree, w), w, '}'),
        ),
        |res| {
            use AnyOf3::*;
            match res {
                Child1((_, _, trees, _, _)) => trees,
                Child2((_, _, trees, _, _)) => trees,
                Child3((_, _, trees, _, _)) => trees,
            }
        }
    ));

    token_rule.set(MapV(
        SpanOf(OneOf2(
            punctuation,
            delimiter
        )),
        |span| { RToken { span } }
    ));

    // --- TYPES ---

    _type_rule.set(MapV(
        OneOf3(
            type_no_bounds,
            impl_trait_type,
            trait_object_type
        ),
        |three| {
            use AnyOf3::*;
            match three {
                Child1(ty) => ty,
                Child2(ty) => RType::ImplTrait(Box::new(ty)),
                Child3(ty) => RType::TraitObj(Box::new(ty)),
            }
        }
    ));

    trait_object_type_rule.set(MapV(
        Spanned((Maybe("dyn"), w, type_param_bounds)),
        |(span, (_, _, _bounds))| {
            RTraitObjType { span }
        }
    ));
    trait_object_type_one_bound_rule.set(MapV(
        Spanned((Maybe("dyn"), w, trait_bound)),
        |(span, (_, _, _bounds))| {
            RTraitObjType { span }
        }
    ));

    type_no_bounds_rule.set(MapV(
        Spanned(OneOf9(
            parenthesized_type,
            impl_trait_type_one_bound, 
            trait_object_type_one_bound, 
            type_path, 
            tuple_type, 
            never_type, 
            raw_pointer_type, 
            reference_type, 
            OneOf6(
                array_type,
                slice_type,
                inferred_type,
                qualified_path_in_type,
                bare_function_type,
                macro_invocation
            )
        )),
        |(span, nine)| {
            use AnyOf9::*;
            match nine {
                Child1(ty) => ty,
                Child2(ty) => RType::ImplTrait(Box::new(ty)),
                Child3(ty) => RType::TraitObj(Box::new(ty)),
                Child4(ty) => RType::Path(Box::new(ty)),
                Child5(ty) => RType::Tuple(Box::new(ty)),
                Child6(ty) => RType::Never(Box::new(ty)),
                Child7(ty) => RType::Ptr(Box::new(ty)),
                Child8(ty) => RType::Ref(Box::new(ty)),
                Child9(AnyOf6::Child1(ty)) => RType::Array(Box::new(ty)),
                Child9(AnyOf6::Child2(ty)) => RType::Slice(Box::new(ty)),
                Child9(AnyOf6::Child3(ty)) => RType::Inferred(Box::new(ty)),
                Child9(AnyOf6::Child4(ty)) => RType::QualPath(Box::new(ty)),
                Child9(AnyOf6::Child5(ty)) => RType::BareFn(Box::new(ty)),
                Child9(AnyOf6::Child6(ty)) => RType::Macro(Box::new(ty)),
            }
        }
    ));

    parenthesized_type_rule.set(MapV(
        Surround(
            '(',
            (w, _type, w),
            ')',
            |_, _, e| e,
            |_, start_span, _, _| panic(start_span, "parenthesized_type", "missing ending parenthesis")
        ),
        |(_, (_, ty, _), _)| ty
    ));

    impl_trait_type_rule.set(MapV(
        Spanned(("impl", w, type_param_bounds)),
        |(span, (_, _, _bounds))| {
            RImplTraitType { span }
        }
    ));

    impl_trait_type_one_bound_rule.set(MapV(
        Spanned(("impl", w, trait_bound)),
        |(span, (_, _, _))| {
            RImplTraitType {
                span,
            }
        }
    ));

    // - TYPE PATH -    

    type_path_rule.set(MapV(
        Spanned((Maybe("::"), type_path_segment, ZeroOrMore(("::", type_path_segment)))),
        |(span, (_, ty_seg, segs))| {
            RTypePath {
                span,
                segs: segs.into_iter().map(|(_, v)|v).collect()
            }
        }
    ));

    type_path_segment_rule.set(MapV(
        Spanned((path_ident_segment, Maybe((Maybe("::"), OneOf2(generic_args, type_path_fn))))),
        |(span, (ident, maybe))| {
            use AnyOf2::*;
            let (generic_args, type_path_fn) = match maybe.map(|(_, two)|two) {
                Some(Child1(v)) => (Some(v), None   ),
                Some(Child2(v)) => (None   , Some(v)),
                None            => (None   , None   ),
            };
            RTypePathSegment { span, ident, generic_args, type_path_fn }
        }
    ));

    type_path_fn_rule.set(MapV(Spanned((
            Surround(
                '(',
                (w, Maybe(type_path_fn_inputs), w),
                ')',
                |_, _, e| e,
                |_, start_span, _, _| panic(start_span, "type_path_fn", "expected closed parenthesis somewhere after this open parenthesis")
            ),
            Maybe((
                w,
                "->",
                w,
                _type
            ))
        )),
        |(span, ((_, (_, inputs, _), _), ret))| { RTypePathFn { span, inputs, ret: ret.map(|(_, _, _, ty)|ty) } }
    ));

    type_path_fn_inputs_rule.set(MapV(
        Spanned((_type, ZeroOrMore((w, ',', w, _type)), Maybe((w, ',')))),
        |(span, (t1, types, _))| {
            let mut types: Vec<RType> = types.into_iter().map(|(_, _, _, ty)|ty).collect();
            types.insert(0, t1);
            RTypeList { span, types }
        }
    ));

    tuple_type_rule.set(MapV(
        Spanned(Surround(
            '(',
            (w, Join(_type, (w, ',', w)), w),
            ')',
            |_, _, e| e,
            |_, span, _, _| panic(span, "tuple_type", "tuple type expected ending `)`")
        )),
        |(span, (_, (_, types, _), _))| {
            RTupleType { span, children: types }
        }
    ));

    never_type_rule.set(MapV(
        '!',
        |span| { RNeverType { span: span.clone(), value: span } }
    ));

    raw_pointer_type_rule.set(MapV(
        Spanned(('*', w, OneOf2("mut", "const"), w, type_no_bounds)),
        |(span, (_, _, two, _, ty))| {
            RPtrType {
                span,
                mutable: match two {
                    AnyOf2::Child1(_) => true,
                    AnyOf2::Child2(_) => false
                },
                ty
            }
        }
    ));

    reference_type_rule.set(MapV(
        Spanned(('&', w, Maybe((lifetime, w)), Maybe(("mut", w)), type_no_bounds)),
        |(span, (_, _, lifetime, m, ty))| {
            RRefType {
                span,
                lifetime: lifetime.map(|(l, _)|l),
                mutable: m.is_some(),
                ty
            }
        }
    ));

    array_type_rule.set(MapV(
        Spanned(('[', w, _type, w, ';', w, expression, w, ']')),
        |(span, (_, _, child, _, _, _, expr, _, _))| { RArrayType { span, child, expr } }
    ));

    slice_type_rule.set(MapV(
        Spanned(('[', w, _type, w, ']')),
        |(span, (_, _, child, _, _))| { RSliceType { span, child } }
    ));

    inferred_type_rule.set(MapV(
        '_',
        |span| { RInferredType { span } }
    ));

    // --- PATHS ---

    // - SIMPLE PATHS -

    simple_path_rule.set(MapV(
        Spanned((Maybe("::"), simple_path_segment, ZeroOrMore(("::", simple_path_segment)))),
        |(span, (_, span1, follow_segments))| {
            let mut path: Vec<Span<PPos>> = follow_segments.into_iter().map(|(_, s)|s).collect();
            path.insert(0, span1);
            RSimplePath { span, path }
        }
    ));

    simple_path_segment_rule.set(
        SpanOf(OneOf5(identifier, "super", "self", "crate", "$crate"))
    );

    // - PATHS IN EXPRESSIONS -

    path_in_expression_rule.set(MapV(
        Spanned((Maybe("::"), path_expr_segment, ZeroOrMore(("::", path_expr_segment)))),
        |(span, (_, path1, path_cont))| {
            let mut path: Vec<RPathInExprSeg> = path_cont.into_iter().map(|(_, seg)|seg).collect();
            path.insert(0, path1);
            RPathInExpr { span, path }
        }
    ));

    path_expr_segment_rule.set(MapV(
        Spanned((path_ident_segment, Maybe(("::", generic_args)))),
        |(span, (ident, generics))| {
            RPathInExprSeg {
                span: span.clone(),
                ident,
                generics: match generics {
                    Some((_, args)) => args,
                    None => RGenericArgs { span, args: Vec::new() }
                }
            }
        }
    ));

    path_ident_segment_rule.set(MapV(
        OneOf6(identifier, "super", "self", "Self", "crate", "$crate"),
        |six| {
            use AnyOf6::*;
            match six {
                Child1(span) => RPathInExprIdent { span },
                Child2(span) => RPathInExprIdent { span },
                Child3(span) => RPathInExprIdent { span },
                Child4(span) => RPathInExprIdent { span },
                Child5(span) => RPathInExprIdent { span },
                Child6(span) => RPathInExprIdent { span },
            }
        }
    ));

    generic_args_rule.set(MapV(
        Spanned(
            Surround(
                '<',
                (w, Join(generic_arg, (w, ',', w)), w),
                '>',
                |_, _, e| e,
                |_, span, _, _| panic(span, "generic_args", "expected '>' sometime after this point")
            )
        ),
        |(span, (_, (_, args, _), _))| {
            RGenericArgs { span, args }
        }
    ));

    generic_arg_rule.set(MapV(
        OneOf4(
            lifetime,
            _type,
            generic_args_const,
            generic_args_binding
        ),
        |four| {
            use AnyOf4::*;
            match four {
                Child1(v) => RGenericArg::Lifetime(v),
                Child2(v) => RGenericArg::Type(v),
                Child3(v) => RGenericArg::GenericConstArg(v),
                Child4(v) => RGenericArg::GenericConstArgsBinding(v),
            }
        }
    ));

    generic_args_const_rule.set(Map(
        OneOf4(
            block_expression,
            literal_expression,
            Spanned(('-', w, literal_expression)),
            simple_path_segment,
        ),
        |res| {
            use ParseResult::*;
            use AnyOf4::*;
            match res {
                Okay(res, adv) => {
                    match res {
                        Child1(v) => Okay(RGenericArgsConst::Block(v), adv),
                        Child2(v) => Okay(RGenericArgsConst::Lit(v), adv),
                        Child3((span, (neg, _, lit))) => {
                            Panic(match lit {
                                RLit::Char(_) => panic(span, "generic_args_const", "`Char` cannot have a sign"),
                                RLit::String(_) => panic(span, "generic_args_const", "`String` cannot have a sign"),
                                RLit::RawString(_) => panic(span, "generic_args_const", "`RawString` cannot have a sign"),
                                RLit::Byte(_) => panic(span, "generic_args_const", "`ByteString` cannot have a sign"),
                                RLit::ByteString(_) => panic(span, "generic_args_const", "`ByteString` cannot have a sign"),
                                RLit::RawByteString(_) => panic(span, "generic_args_const", "`RawByteString` cannot have a sign"),
                                RLit::Integer(lit) => return Okay(RGenericArgsConst::SLit(RSLit::Integer(RSIntLit { span: span, neg: true, lit })), adv),
                                RLit::Float(lit) => return Okay(RGenericArgsConst::SLit(RSLit::Float(RSFloatLit { span: span, neg: true, lit })), adv),
                                RLit::Bool(_) => panic(span, "generic_args_const", "`Bool` cannot have a sign"),
                            })
                        },
                        Child4(v) => Okay(RGenericArgsConst::PathSeg(v), adv),
                    }
                },
                Error(r) => Error(r),
                Panic(r) => Panic(r),
            }
        }
    ));

    generic_args_binding_rule.set(MapV(
        Spanned((identifier, w, '=', w, _type)),
        |(span, (ident, _, _, _, ty))| { RGenericArgsBinding { span, ident, ty } }
    ));

    // - QUALIFIED PATH -

    qualified_path_in_expression_rule.set(MapV(
        Spanned((qualified_path_type, OneOrMore(("::", path_expr_segment)))),
        |(span, (ty, segs))| {
            RQualPathInExpr {
                span,
                qual_path_type: ty,
                segs: segs.into_iter().map(|(_, seg)|seg).collect()
            }
        }
    ));

    qualified_path_type_rule.set(MapV(
        Spanned(('<', w, _type, Maybe((w, "as", w, type_path)), w, '>')),
        |(span, (_, _, ty, maybe_ty, _, _))| {
            RQualPathType { ty, as_ty: maybe_ty.map(|(_, _, _, ty)|ty) }
        }
    ));

    qualified_path_in_type_rule.set(MapV(
        Spanned((qualified_path_type, OneOrMore(("::", type_path_segment)))),
        |(span, (ty, segs))| {
            RQualPathInType {
                span,
                qual_path_type: ty,
                segs: segs.into_iter().map(|(_, seg)|seg).collect()
            }
        }
    ));

    // - FUNCTION POINTER TYPES -

    bare_function_type_rule.set(MapV(
        SpanOf((Maybe((for_lifetimes, w)), Maybe((function_type_qualifiers, w)), "fn", w, '(', w, Maybe(function_parameters_maybe_named_variadic), w, ')', Maybe((w, bare_function_return_type)))),
        |span| { RBareFnType { span } }
    ));

    function_type_qualifiers_rule.set(
        OneOf2(
            ("unsafe", w, "extern", Maybe((w, abi))),
            ("extern", Maybe((w, abi))),
        )
    );

    bare_function_return_type_rule.set(MapV(
        ("->", w, type_no_bounds),
        |(_, _, ty)| ty
    ));

    function_parameters_maybe_named_variadic_rule.set(
        OneOf2(maybe_named_function_parameters, maybe_named_function_parameters_variadic)
    );

    maybe_named_function_parameters_rule.set(
        (maybe_named_param, ZeroOrMore((w, ',', w, maybe_named_param)), Maybe((w, ',')))
    );

    maybe_named_param_rule.set(
        (ZeroOrMore((outer_attribute, w)), Maybe((w, OneOf2(identifier, '_'), w, ':', w)), _type)
    );

    maybe_named_function_parameters_variadic_rule.set(
        (ZeroOrMore((maybe_named_param, w, ',')), w, maybe_named_param, w, ',', ZeroOrMore((w, outer_attribute)), w, "...")
    );

    // - MACRO INVOCATION -

    macro_invocation_rule.set(MapV(
        Spanned((simple_path, '!', Req(delim_token_tree, |_, pos, _| panic(Span::new(pos.clone(), pos), "macro_invocation", "expected macro call body")))),
        |(span, (path, _, delim_tree))| { RMacroInvocation { span, path, trees: delim_tree.trees } }
    ));

    delim_token_tree_rule.set(MapV(Spanned(OneOf3(
            Surround(
                '(', (w, Join(token_tree, w), w), ')',
                |_, _, e| e,
                |_, s, _, _| panic(s, "delim_tree_rule", "expected a matching closing parenthesis (')') sometime after this point")
            ),
            Surround(
                '[', (w, Join(token_tree, w), w), ']',
                |_, _, e| e,
                |_, s, _, _| panic(s, "delim_tree_rule", "expected a matching closing brace (']') sometime after this point")
            ),
            Surround(
                '{', (w, Join(token_tree, w), w), '}',
                |_, _, e| e,
                |_, s, _, _| panic(s, "delim_tree_rule", "expected a matching closing curly brace ('}') sometime after this point")
            ),
        )),
        |(span, three)| {
            use AnyOf3::*;
            match three {
                Child1((_, (_, trees, _), _)) => RDelimTokenTree { span, trees },
                Child2((_, (_, trees, _), _)) => RDelimTokenTree { span, trees },
                Child3((_, (_, trees, _), _)) => RDelimTokenTree { span, trees },
            }
        }
    ));

    token_tree_rule.set(MapV(
        OneOf2((Not(OneOf(['{', '[', '('])), token), delim_token_tree),
        |two| {
            use AnyOf2::*;
            match two {
                Child1((_, token)) => RTokenTree::Token(token),
                Child2(delim_tree) => RTokenTree::DelimTokenTree(Box::new(delim_tree)),
            }
        }
    ));

    macro_invocation_semi_rule.set(MapV(
        Spanned(OneOf3(
            (simple_path, '!', '(', ZeroOrMore((w, token_tree)), w, ')', ';'),
            (simple_path, '!', '[', ZeroOrMore((w, token_tree)), w, ']', ';'),
            (simple_path, '!', '{', ZeroOrMore((w, token_tree)), w, '}'),
        )),
        |(span, three)| {
            use AnyOf3::*;
            match three {
                Child1((path, _, _, trees, _, _, _)) => RMacroInvocation { span, path, trees: trees.into_iter().map(|(_, t)|t).collect() },
                Child2((path, _, _, trees, _, _, _)) => RMacroInvocation { span, path, trees: trees.into_iter().map(|(_, t)|t).collect() },
                Child3((path, _, _, trees, _, _   )) => RMacroInvocation { span, path, trees: trees.into_iter().map(|(_, t)|t).collect() },
            }
        }
    ));

    // --- COMMENTS ---

    comment_rule.set(SpanOf(OneOf7(
        line_comment,
        block_comment,
        inner_line_doc,
        inner_block_doc,
        outer_line_doc,
        outer_block_doc,
        block_comment_or_doc,
    )));

    line_comment_rule.set(MapV(
        Spanned(OneOf2(
            ("//", OneOf2(Not(OneOf(['/', '!', '\n'])), "//"), SpanOf(ZeroOrMore((Not('\n'), AnyV)))),
            "//"
        )),
        |(span, any_of_two)| {
            match any_of_two {
                AnyOf2::Child1((_, _, text)) => RComment::Line { span, text },
                AnyOf2::Child2(span)         => RComment::Line { span: span.clone(), text: span },
            }
        }
    ));

    block_comment_rule.set(MapV(
        Spanned(Surround(
            "/*",
            SpanOf((
                OneOf3(
                    Not(OneOf(['*', '!'])),
                    "**",
                    block_comment_or_doc
                ),
                ZeroOrMore(OneOf2(
                    block_comment_or_doc,
                    (Not("*/"), AnyV)
                )),
            )),
            "*/",
            |_, _, e| e,
            |_, start_span, _, _| panic(start_span, "block_comment", "expected end to this block comment"),
        )),
        |(span, (_, text, _))| {
            RComment::Block { span, text }
        }
    ));

    inner_line_doc_rule.set(MapV(
        Spanned((
            "//!",
            SpanOf(ZeroOrMore((Not(OneOf2('\n', isolated_cr)), AnyV)))
        )),
        |(span, (_, text))| {
            RComment::InnerLineDoc { span, text }
        }
    ));

    inner_block_doc_rule.set(MapV(
        Spanned(Surround(
            "/*!",
            SpanOf(ZeroOrMore(OneOf2(block_comment_or_doc, (Not(OneOf2("*/", isolated_cr)), AnyV)))),
            "*/",
            |_, _, e| e,
            |_, start_span, _, _| panic(start_span, "inner_block_doc", "expected end to this block doc comment"),
        )),
        |(span, (_, text, _))| {
            RComment::InnerBlockDoc { span, text }
        }
    ));

    outer_line_doc_rule.set(MapV(
        Spanned((
            "///",
            SpanOf(Maybe((Not('/'), ZeroOrMore(Not(OneOf2('\n', isolated_cr))))))
        )),
        |(span, (_, text))| {
            RComment::OuterLineDoc { span, text }
        }
    ));

    outer_block_doc_rule.set(MapV(
        Spanned(Surround(
            "/**",
            SpanOf((OneOf2(Not('*'), block_comment_or_doc), ZeroOrMore(OneOf2(block_comment_or_doc, (Not(OneOf2("*/", isolated_cr)), AnyV))))),
            "*/",
            |_, _, e| e,
            |_, start_span, _, _| panic(start_span, "inner_block_doc", "expected end to this block doc comment"),
        )),
        |(span, (_, text, _))| {
            RComment::OuterBlockDoc { span, text }
        }
    ));

    block_comment_or_doc_rule.set(
        MapV(
            OneOf3(block_comment, outer_block_doc, inner_block_doc),
            |any_of_three| {
                match any_of_three {
                    AnyOf3::Child1(v) => v,
                    AnyOf3::Child2(v) => v,
                    AnyOf3::Child3(v) => v,
                }
            }
        )
    );

    // - SUFFIX -

    suffix_rule.set(identifier_or_keyword);

    suffix_no_e_rule.set(SpanOf((Not(OneOf(['e', 'E'])), suffix)));

    // - INTEGER LITERALS -

    integer_literal_rule.set(MapV(Spanned((
            OneOf4(
                dec_literal,
                bin_literal,
                oct_literal,
                hex_literal
            ),
            Maybe(suffix_no_e)
        )),
        |(span, (any_of_four, suffix))| {
            use AnyOf4::*;
            match any_of_four {
                Child1(dec) => RIntLit::DecLit { span, lit: dec, suffix },
                Child2(bin) => RIntLit::BinLit { span, lit: bin, suffix },
                Child3(oct) => RIntLit::OctLit { span, lit: oct, suffix },
                Child4(hex) => RIntLit::HexLit { span, lit: hex, suffix },
            }
        }
    ));

    dec_literal_rule.set(MapV(
        SpanOf((
            dec_digit,
            ZeroOrMore(OneOf2(
                dec_digit,
                '_'
            ))
        )),
        |span| { RDecLit { span: span.clone(), value: span } }
    ));

    bin_literal_rule.set(MapV(
        Spanned(("0b", SpanOf((ZeroOrMore(OneOf2(bin_digit, '_')), bin_digit, ZeroOrMore(OneOf2(bin_digit, '_')))))),
        |(span, (_, value))| { RBinLit { span, value } }
    ));

    oct_literal_rule.set(MapV(
        Spanned(("0o", SpanOf((ZeroOrMore(OneOf2(oct_digit, '_')), oct_digit, ZeroOrMore(OneOf2(oct_digit, '_')))))),
        |(span, (_, value))| { ROctLit { span, value } }
    ));

    hex_literal_rule.set(MapV(
        Spanned(("0x", SpanOf((ZeroOrMore(OneOf2(hex_digit, '_')), hex_digit, ZeroOrMore(OneOf2(hex_digit, '_')))))),
        |(span, (_, value))| { RHexLit { span, value } }
    ));

    bin_digit_rule.set('0'..='1');

    oct_digit_rule.set('0'..='7');

    dec_digit_rule.set('0'..='9');

    hex_digit_rule.set(SpanOf(OneOf3('0'..='9', 'a'..='f', 'A'..='F')));

    tuple_index_rule.set(integer_literal);

    // - CHARACTER -

    char_literal_rule.set(MapV(
        Spanned((
            '\'',
            SpanOf(OneOf4(
                (Not(OneOf(['\'', '\\', '\n', '\r', '\t'])), AnyV),
                quote_escape,
                ascii_escape,
                unicode_escape
            )),
            '\'',
            Maybe(suffix)
        )),
        |(span, (_, value, _, suffix))| {
            RCharLit { span, value, suffix }
        }
    ));

    quote_escape_rule.set(SpanOf(OneOf(["\\'", "\\\""])));

    ascii_escape_rule.set(
        SpanOf(OneOf6(
            ("\\x", oct_digit, hex_digit),
            "\\n",
            "\\r",
            "\\t",
            "\\\\",
            "\\0"
        ))
    );

    let _d = (hex_digit, ZeroOrMore('_'));
    let _d = &_d;
    unicode_escape_rule.set(
        ("\\u{",
            _d, Maybe(_d), Maybe(_d), Maybe(_d), Maybe(_d), Maybe(_d),
        '}')
    );

    float_literal_rule.set(MapV(
        Spanned(OneOf3(
            SpanOf((dec_literal, '.', Not(OneOf3('.', '_', xid_start)))),
            (SpanOf((dec_literal, '.', dec_literal)), Maybe(suffix_no_e)),
            (Spanned((SpanOf((dec_literal, Maybe(('.', dec_literal)))), float_exponent)), Maybe(suffix)),
        )),
        |(span, three)| {
            use AnyOf3::*;
            match three {
                Child1(value) => {
                    RFloatLit { span: span.clone(), value, exp: None, value_exp_span: span, suffix: None }
                },
                Child2((value, suffix)) => {
                    RFloatLit { span: span.clone(), value, exp: None, value_exp_span: span, suffix }
                },
                Child3(((value_exp_span, (value, exp)), suffix)) => {
                    RFloatLit { span, value, exp: Some(exp), value_exp_span, suffix }
                }
            }
        }
    ));

    float_exponent_rule.set(SpanOf((
        OneOf(['e', 'E']),
        Maybe(OneOf(['+', '-'])),
        ZeroOrMore(OneOf2(dec_digit, '_')),
        dec_digit,
        ZeroOrMore(OneOf2(dec_digit, '_'))
    )));

    reserved_number_rule.set(Map(
        Spanned(OneOf8(
            (bin_literal, '2'..='9'),
            (oct_literal, '8'..='9'),
            (
                OneOf3(
                    bin_literal,
                    oct_literal,
                    hex_literal
                ),
                '.',
                Not(OneOf3('.', '_', xid_start))
            ),
            (
                OneOf2(bin_literal, oct_literal),
                OneOf(['e', 'E'])
            ),
            (
                "0b",
                ZeroOrMore('_'),
                OneOf2(
                    End(),
                    (Not(bin_digit), AnyV)
                )
            ),
            (
                "0o",
                ZeroOrMore('_'),
                OneOf2(
                    End(),
                    (Not(oct_digit), AnyV)
                )
            ),
            (
                "0x",
                ZeroOrMore('_'),
                OneOf2(
                    End(),
                    (Not(hex_digit), AnyV)
                )
            ),
            (
                dec_literal,
                Maybe(('.', dec_literal)),
                OneOf2('e', 'E'),
                Maybe(OneOf2('+', '-')),
                OneOf2(
                    End(),
                    (Not(dec_digit), AnyV)
                )
            )
        )),
        |res| {
            (match res {
                Okay((span, _), _)
                    => ParseResult::Panic(panic(span, "reserved_number", "this number is reserved and cannot be used")),
                res => res
            }).map_value(|v|())
        }
    ));

    // - STRING LITERALS -

    string_literal_rule.set(MapV(Spanned((
            '"',
            SpanOf(ZeroOrMore(OneOf5(
                (Not(OneOf3('"', '\\', isolated_cr)), AnyV),
                quote_escape,
                ascii_escape,
                unicode_escape,
                string_continue
            ))),
            '"',
            Maybe(suffix)
        )),
        |(span, (_, text, _, suffix))| {
            match suffix {
                Some(suffix) => {
                    RStrLit {
                        span,
                        text,
                        suffix: Some(suffix)
                    }
                },
                None => {
                    RStrLit {
                        span,
                        text,
                        suffix: None
                    }
                }
            }
        }
    ));

    string_continue_rule.set("\\\n");

    // - RAW STRING LITERALS -

    raw_string_literal_rule.set(MapV(
        Spanned(('r', raw_string_content, Maybe(suffix))),
        |(span, (_, text, suffix))| {
            match suffix {
                Some(suffix) => { RRawStrLit { span, text, suffix: Some(suffix) } },
                None => { RRawStrLit { span, text, suffix: None } },
            }
        }
    ));

    raw_string_content_rule.set(MapV(
        OneOf2(
            ('"', SpanOf(ZeroOrMore((Not(isolated_cr), Not('"'), AnyV))), '"'),
            ('#', raw_string_content, '#')
        ),
        |two| {
            use AnyOf2::*;
            match two {
                Child1((_, text, _)) => text,
                Child2((_, text, _)) => text 
            }
        }
    ));

    byte_literal_rule.set(MapV(
        Spanned(('b', ',', SpanOf(OneOf2(ascii_for_char, byte_escape)), '\'', Maybe(suffix))),
        |(span, (_, _, value, _, suffix))| { RByteLit { span, value, suffix } }
    ));

    ascii_for_char_rule.set((Not(OneOf(['\\', '\n', '\r', '\t'])), 0x00..=0x7F));

    byte_escape_rule.set(
        OneOf8(
            ("\\x", hex_digit, hex_digit),
            "\\n",
            "\\r",
            "\\t",
            "\\\\",
            "\\0",
            "\\'",
            "\\\"",
        )
    );

    // - BYTE STRING LITERALS -

    byte_string_literal_rule.set(MapV(
        Spanned(("b\"", SpanOf(ZeroOrMore((ascii_for_string, byte_escape, string_continue))), '"', Maybe(suffix))),
        |(span, (_, value, _, suffix))| { RByteStrLit { span, value, suffix } }
    ));

    ascii_for_string_rule.set(
        (Not(OneOf3('"', '\\', isolated_cr)), 0x00..=0x7F)
    );

    // - RAW BYTE STRING LITERAL -

    raw_byte_string_literal_rule.set(MapV(
        Spanned(("br", raw_byte_string_content, Maybe(suffix))),
        |(span, (_, text, suffix))| {
            RRawByteStrLit { span, text, suffix }
        }
    ));

    raw_byte_string_content_rule.set(MapV(
        OneOf2(
            ('"', SpanOf(ZeroOrMore((Not('"'), ascii))), '"'),
            ('#', raw_byte_string_content, '#')
        ),
        |any_of_two| {
            use AnyOf2::*;
            match any_of_two {
                Child1((_, span, _)) => span,
                Child2((_, span, _)) => span,
            }
        }
    ));

    ascii_rule.set(0x00..=0x7F);

    // --- Lifetimes and Loop Labels ---

    lifetime_token_rule.set(MapV(
        Spanned(OneOf2(
            ('\'', '_'),
            ('\'', identifier_or_keyword),
        )),
        |(span, two)| {
            use AnyOf2::*;
            match two {
                Child1((_, underscore)) => RLifetime::Elided { span, underscore },
                Child2((_, ident))      => RLifetime::Ident { span, ident },
            }
        }
    ));

    lifetime_or_label_rule.set(MapV(
        Spanned(('\'', non_keyword_identifier)),
        |(span, (_, ident))| { RLifetime::Ident { span, ident } }
    ));

    // --- Reserved Prefixes ---

    reserved_token_double_quote_rule.set(Map(
        SpanOf((
            OneOf2(
                (Not(OneOf3('b', 'r', "br")), identifier_or_keyword),
                '_'
            ),
            '"'
        )),
        |res| {
            match res {
                Okay(span, _) => {
                    ParseResult::Panic(panic(span, "reserved_token_double_quote_rule", "this double quote token is reserved by Rust and therefore can not be used"))
                },
                res => res
            }.map_value(|_|())
        }
    ));

    reserved_token_single_quote_rule.set(Map(
        SpanOf((
            OneOf2(
                (Not(OneOf2('b', '_')), identifier_or_keyword),
                '_'
            ),
            '\''
        )),
        |res| {
            match res {
                Okay(span, _) => {
                    ParseResult::Panic(panic(span, "reserved_token_single_quote_rule", "this single quote token is reserved by Rust and therefore cannot be used"))
                },
                res => res
            }.map_value(|_|())
        }
    ));

    reserved_token_pound_rule.set(Map(
        SpanOf((
            OneOf2(
                (Not(OneOf3('r', "br", '_')), identifier_or_keyword),
                '#'
            ),
            '\''
        )),
        |res| {
            match res {
                Okay(span, _) => {
                    ParseResult::Panic(panic(span, "reserved_token_pound_rule", "this pound token is reserved by Rust and therefore cannot be used"))
                },
                res => res
            }.map_value(|_|())
        }
    ));

    // --- MACRO DEFINITION ---

    macro_rules_definition_rule.set(MapV(
        Spanned(("macro_rules!", identifier, macro_rules_def)),
        |(span, (_, ident, rules_def))| { RMacroDef { span, ident, rules_def } }
    ));

    macro_rules_def_rule.set(MapV(
        Spanned(OneOf3(
            ('(', macro_rules, ')', ';'),
            ('[', macro_rules, ']', ';'),
            ('{', macro_rules, '}'),
        )),
        |(span, three)| {
            use AnyOf3::*;
            match three {
                Child1((_, rules, _, _)) => RMacroRulesDef { span, rules },
                Child2((_, rules, _, _)) => RMacroRulesDef { span, rules },
                Child3((_, rules, _   )) => RMacroRulesDef { span, rules },
            }
        }
    ));

    macro_rules_rule.set(MapV(
        Spanned((macro_rule, ZeroOrMore((w, ';', w, macro_rule)), Maybe((w, ';')))),
        |(span, (rule, rules, _))| {
            let mut rules: Vec<RMacroRule> = rules.into_iter().map(|(_, _, _, r)|r).collect();
            rules.push(rule);
            RMacroRules { span, rules, }
        }
    ));

    macro_rule_rule.set(MapV(
        Spanned((macro_match, w, "=>", w, macro_transcriber)),
        |(span, (macro_match, _, _, _, macro_transcriber))| {
            RMacroRule { span, macro_match, macro_transcriber }
        }
    ));

    macro_matcher_rule.set(MapV(
        Spanned(OneOf3(
            ('(', ZeroOrMore((w, macro_match)), w, ')'),
            ('[', ZeroOrMore((w, macro_match)), w, ']'),
            ('{', ZeroOrMore((w, macro_match)), w, '}'),
        )),
        |(span, three)| {
            use AnyOf3::*;
            match three {
                Child1((_, matches, _, _)) => RMacroMatcher { span, matches: matches.into_iter().map(|(_, m)|m).collect() },
                Child2((_, matches, _, _)) => RMacroMatcher { span, matches: matches.into_iter().map(|(_, m)|m).collect() },
                Child3((_, matches, _, _)) => RMacroMatcher { span, matches: matches.into_iter().map(|(_, m)|m).collect() },
            }
        }
    ));

    macro_match_rule.set(MapV(
        OneOf4(
            (Not(OneOf(['$', '{', '[', '('])), token),
            macro_matcher,
            Spanned((
                '$',
                SpanOf(OneOf3(
                    (Not("crate"), identifier_or_keyword),
                    raw_identifier,
                    '_'
                )),
                ':',
                macro_frag_spec
            )),
            Spanned(('$', '(', OneOrMore(macro_match), ')', Maybe(macro_rep_sep), macro_rep_op))
        ),
        |four| {
            use AnyOf4::*;
            match four {
                Child1((_, token)) => RMacroMatch::Token(token),
                Child2(matcher) => RMacroMatch::Matcher(matcher),
                Child3((span, (_, arg, _, spec))) => RMacroMatch::Arg(RMacroArg { span, arg, spec }),
                Child4((span, (_, _, matches, _, sep, op)))  => RMacroMatch::OpArg(RMacroOpArg { span, matches, sep, op }),
            }
        }
    ));

    macro_frag_spec_rule.set(MapV(
        OneOf9(
            "block",
            "expr",
            "ident",
            "item",
            "lifetime",
            "literal",
            "meta",
            "pat",
            OneOf6(
                "pat_param",
                "path",
                "stmt",
                "tt",
                "ty",
                "vis"
            )
        ),
        |nine| {
            use AnyOf9::*;
            match nine {
                Child1(span) => RMatchSpec::Block(span),
                Child2(span) => RMatchSpec::Expr(span),
                Child3(span) => RMatchSpec::Ident(span),
                Child4(span) => RMatchSpec::Item(span),
                Child5(span) => RMatchSpec::Lifetime(span),
                Child6(span) => RMatchSpec::Literal(span),
                Child7(span) => RMatchSpec::Meta(span),
                Child8(span) => RMatchSpec::Pat(span),
                Child9(AnyOf6::Child1(span)) => RMatchSpec::PatParam(span),
                Child9(AnyOf6::Child2(span)) => RMatchSpec::Path(span),
                Child9(AnyOf6::Child3(span)) => RMatchSpec::Stmt(span),
                Child9(AnyOf6::Child4(span)) => RMatchSpec::Tt(span),
                Child9(AnyOf6::Child5(span)) => RMatchSpec::Ty(span),
                Child9(AnyOf6::Child6(span)) => RMatchSpec::Vis(span),
            }
        }
    ));

    macro_rep_sep_rule.set(MapV(
        (Not(OneOf4(macro_rep_op, '(', '{', '[')), token),
        |(_, token)| token
    ));

    macro_rep_op_rule.set(MapV(
        OneOf3('*', '+', '?'),
        |three| {
            use AnyOf3::*;
            match three {
                Child1(span) => RMacroOp::ZeroOrMore(span),
                Child2(span) => RMacroOp::OneOrMore(span),
                Child3(span) => RMacroOp::Optional(span),
            }
        }
    ));

    macro_transcriber_rule.set(MapV(
        Spanned(delim_token_tree),
        |(span, tree)| { RMacroTranscriber { span, tree } }
    ));

    // --- CRATE ---

    _crate_rule.set(MapV((
            w,
            Maybe(("\\uFEFF", w)),
            Maybe((SpanOf(("#!", OneOrMore((Not('\n'), AnyV)))), w)),
            ZeroOrMore((inner_attribute, w)),
            ZeroOrMore((item, w)),
            Req(End(), |_, pos, _| format!("{}: parser failed to reach the end of the file (from this pos)", pos))
        ),
        |(_, utf8bom, shebang, _, items, _)| {
            println!("Crate!");
            RCrate {
                utf8bom: utf8bom.map(|(s, _)|s),
                shebang: shebang.map(|(s, _)|s),
                items: items.into_iter().map(|(i, _)|i).collect()
            }
        }
    ));

    // --- CONDITIONAL COMPILATION ---

    configuration_predicate_rule.set(
        MapV(
            OneOf4(
                configuration_option,
                configuration_all,
                configuration_any,
                configuration_not
            ),
            |four| {
                use AnyOf4::*;
                match four {
                    Child1(child) => child,
                    Child2(child) => child,
                    Child3(child) => child,
                    Child4(child) => child,
                }
            }
        )
    );

    configuration_option_rule.set(MapV(
        Spanned((identifier, Maybe(('=', OneOf2(string_literal, raw_string_literal))))),
        |(span, (ident, m))| {
            match m {
                Some((_, string)) => {
                    RConfigPred::Option {
                        span,
                        ident,
                        string: Some(match string {
                            AnyOf2::Child1(string_lit) => RString::Str(string_lit),
                            AnyOf2::Child2(string_lit) => RString::Raw(string_lit),
                        }),
                    }
                },
                None => {
                    RConfigPred::Option {
                        span,
                        ident,
                        string: None
                    }
                }
            }
        }
    ));

    configuration_all_rule.set(MapV(
        Spanned(("all", '(', w, Maybe(configuration_predicate_list), w, ')')),
        |(span, (_, _, _, list, _, _))| {
            match list {
                Some(list) => {
                    RConfigPred::All {
                        span,
                        preds: list
                    }
                },
                None => {
                    RConfigPred::All {
                        span,
                        preds: Vec::new()
                    }
                }
            }
        }
    ));

    configuration_any_rule.set(MapV(
        Spanned(("any", '(', w, Maybe(configuration_predicate_list), w, ')')),
        |(span, (_, _, _, list, _, _))| {
            match list {
                Some(list) => {
                    RConfigPred::Any {
                        span,
                        preds: list
                    }
                },
                None => {
                    RConfigPred::Any {
                        span,
                        preds: Vec::new()
                    }
                }
            }
        }
    ));
    configuration_not_rule.set(MapV(
        Spanned(("not", '(', w, configuration_predicate, w, ')')),
        |(span, (_, _, _, pred, _, _))| {
            RConfigPred::Not {
                span,
                pred: Box::new(pred)
            }
        }
    ));

    configuration_predicate_list_rule.set(MapV(
        (configuration_predicate, ZeroOrMore((w, ',', w, configuration_predicate)), Maybe((w, ','))),
        |(first, later, _)| {
            let mut list: Vec<RConfigPred> = later.into_iter().map(|v|v.3).collect();
            list.insert(0, first);
            list
        }
    ));

    cfg_attribute_rule.set(MapV(
        ("cfg(", w, configuration_predicate, w, ')'),
        |(_, _, pred, _, _)| pred
    ));

    cfg_attr_attribute_rule.set(MapV(
        Spanned(("cfg_attr(", w, configuration_predicate, w, ',', Maybe(cfg_attrs), w, ')')),
        |(span, (_, _, first, _, _, follow, _, _))| {
            span
        }
    ));

    cfg_attrs_rule.set(
        (attr, ZeroOrMore((w, ',', w, attr)), Maybe((w, ',')))
    );

    // --- ITEMS ---

    item_rule.set(Map(MapV((
            ZeroOrMore((outer_attribute, w)),
            OneOf2(
                vis_item,
                macro_item
            )
        ),
        |(_, two)| {
            use AnyOf2::*;
            match two {
                Child1(i) => RItem::VisItem(i),
                Child2(i) => RItem::MacroItem(i),
            }
        }
    ), |r| { println!("Item: {:?}", r); r}));

    vis_item_rule.set(MapV(Spanned((
            Maybe(visibility),
            OneOf13(
                module,
                extern_crate,
                use_declaration,
                function,
                type_alias,
                _struct,
                enumeration,
                _union,
                constant_item,
                static_item,
                _trait,
                implementation,
                extern_block
            )
        )),
        |(span, (vis, item))| {
            use AnyOf13::*;
            match item {
                Child1(val)  => RVisItem::Mod { span, vis, val },
                Child2(val)  => RVisItem::ExternCrate { span, vis, val },
                Child3(val)  => RVisItem::UseDecl { span, vis, val },
                Child4(val)  => RVisItem::Fn { span, vis, val },
                Child5(val)  => RVisItem::TypeAlias { span, vis, val },
                Child6(val)  => RVisItem::Struct { span, vis, val },
                Child7(val)  => RVisItem::Enum { span, vis, val },
                Child8(val)  => RVisItem::Union { span, vis, val },
                Child9(val)  => RVisItem::Const { span, vis, val },
                Child10(val) => RVisItem::Static { span, vis, val },
                Child11(val) => RVisItem::Trait { span, vis, val },
                Child12(val) => RVisItem::Impl { span, vis, val },
                Child13(val) => RVisItem::ExternBlock { span, vis, val },
            }
        }
    ));

    macro_item_rule.set(MapV(
        OneOf2(macro_invocation_semi, macro_rules_definition),
        |two| {
            use AnyOf2::*;
            match two {
                Child1(i) => RMacroItem::Invocation(i),
                Child2(i) => RMacroItem::MacroDef(i),
            }
        }
    ));

    // --- MODULES ---

    module_rule.set(MapV(
        Spanned(OneOf2(
            (Maybe(("unsafe", w)), "mod", w, identifier, w, ';'),
            (Maybe(("unsafe", w)), "mod", w, identifier, w, '{',
                ZeroOrMore(inner_attribute),
                ZeroOrMore(item),
            '}'),
        )),
        |(span, two)| {
            use AnyOf2::*;
            match two {
                Child1((un, _, _, ident, _, _)) => RMod { span, is_unsafe: un.is_some(), ident, items: Vec::new() },
                Child2((un, _, _, ident, _, _, _, items, _)) => RMod { span, is_unsafe: un.is_some(), ident, items },
            }
        }
    ));

    // --- EXTERN CRATE DECLARATIONS ---

    extern_crate_rule.set(MapV(
        Spanned(("extern", o, "crate", o, crate_ref, o, Maybe(as_clause), w, ';')),
        |(span, (_, _, _, _, crate_ref, _, as_clause, _, _))| {
            RExternCrate {
                span,
                crate_ref,
                as_clause: as_clause.map(|(_, _, ident)|ident)
            }
        }
    ));

    crate_ref_rule.set(
        SpanOf(OneOf2(
            "self",
            identifier,
        ))
    );

    as_clause_rule.set(
        ("as", o, SpanOf(OneOf2(identifier, '_')))
    );

    // --- USE DECLARATIONS ---

    use_declaration_rule.set(MapV(
        Spanned(("use", o, use_tree, w, ';')),
        |(span, (_, _, tree, _, _))| { RUseDecl { span, tree } }
    ));

    use_tree_rule.set(MapV(
        Spanned(OneOf3(
            (Maybe((Maybe(simple_path), "::")), '*'),
            (Maybe((Maybe(simple_path), "::")), w, '{', w, Maybe((use_tree, ZeroOrMore((w, ',', w, use_tree)), Maybe((w, ',')))), w, '}'),
            (simple_path, Maybe((o, "as", o, SpanOf(OneOf2(identifier, '_')))))
        )),
        |(span, three)| {
            use AnyOf3::*;
            match three {
                Child1((path, _)) => {
                    RUseTree::All {
                        span,
                        path: match path.map(|(p, _)|p) {
                            Some(p) => p,
                            None => None
                        }
                    }
                },
                Child2((path, _, _, _, trees, _, _)) => {
                    RUseTree::List {
                        span,
                        path: match path.map(|(p, _)|p) {
                            Some(p) => p,
                            None => None
                        },
                        list: trees.map(|(tree, trees, _)| {
                            let mut trees: Vec<RUseTree> = trees.into_iter().map(|(_, _, _, t)|t).collect();
                            trees.insert(0, tree);
                            trees
                        }).unwrap_or_else(||Vec::new())
                    }
                },
                Child3((path, ident)) => {
                    RUseTree::As {
                        span,
                        path,
                        ident: ident.map(|(_, _, _, ident)|ident)
                    }
                }
            }
        }
    ));

    // --- FUNCTIONS ---

    function_rule.set(MapV(Spanned((
            (Maybe(("const", w)), Maybe(("async", w)), Maybe(("unsafe", w)), Maybe(("extern", w, Maybe((abi, w))))),
            "fn", w,
            identifier,
            Spanned(Maybe(generic_params)),
            '(', w,
            Spanned(Maybe((function_parameters, w))),
            ')', w,
            Maybe(function_return_type), w,
            Maybe(where_clause), w,
            Map(OneOf2(block_expression, ';'), |v| { println!("{:?}", v); v })
        )),
        |(span, ((c, a, u, e), _, _, ident, (g_span, generics), _, _, (p_span, params), _, _, ret_type, _, where_clause, _, body))| {
            RFn {
                span,
                is_const: c.is_some(),
                is_async: a.is_some(),
                is_unsafe: u.is_some(),
                is_extern: {
                    match e.map(|(_, _, abi)|abi) {
                        Some(abi) => abi.map(|(abi, _)|abi),
                        None => None
                    }
                },
                name: ident,
                generics: {
                    match generics {
                        Some(g) => g,
                        None => RGenericParams { span: g_span, params: Vec::new() },
                    }
                },
                params: {
                    match params {
                        Some((p, _)) => p,
                        None => RFnParams { span: p_span, self_param: None, params: Vec::new() },
                    }
                },
                ret_type,
                where_clause,
                body: {
                    use AnyOf2::*;
                    match body {
                        Child1(body) => Some(body),
                        Child2(_) => None,
                    }
                },
            }
        }
    ));

    abi_rule.set(MapV(
        OneOf2(string_literal, raw_string_literal),
        |two| {
            use AnyOf2::*;
            match two {
                Child1(s) => RABI::StrLit(s),
                Child2(s) => RABI::RawStrLit(s),
            }
        }
    ));

    function_parameters_rule.set(MapV(
        Spanned(OneOf2(
            (Maybe((self_param, w, ',', w)), Join(function_param, (w, ',', w)), Maybe((w, ','))),
            (self_param, Maybe((w, ','))),
        )),
        |(span, two)| {
            use AnyOf2::*;
            match two {
                Child1((s, params, _)) => {
                    RFnParams {
                        span,
                        self_param: s.map(|(s, _, _, _)|s),
                        params
                    }
                },
                Child2((self_param, _)) => {
                    RFnParams {
                        span,
                        self_param: Some(self_param),
                        params: Vec::new()
                    }
                },
            }
        }
    ));

    self_param_rule.set(MapV(Spanned((
            ZeroOrMore(outer_attribute),
            OneOf2(
                (Maybe(OneOf2('&', ('&', lifetime))), Maybe("mut"), "self"),
                (Maybe("mut"), "self", ':', _type)
            )
        )),
        |(span, (attrs, two))| {
            use AnyOf2::*;
            match two {
                Child1((lifetime, m, _)) => {
                    let mutable = m.is_some();
                    match lifetime {
                        None => RSelfParam::NotBorrowed { span, attrs, mutable },
                        Some(Child1(_)) => RSelfParam::Borrowed { span, attrs, mutable },
                        Some(Child2((_, lifetime))) => RSelfParam::BorrowedWithLife { span, attrs, mutable, lifetime },
                    }
                },
                Child2((m, _, _, ty)) => {
                    RSelfParam::Longhand { span, attrs, mutable: m.is_some(), ty }
                },
            }
        }
    ));

    function_param_rule.set(MapV(Spanned((
            ZeroOrMore(outer_attribute),
            OneOf3(
                function_param_pattern,
                "...",
                _type,
            )
        )),
        |(span, (attrs, three))| {
            use AnyOf3::*;
            match three {
                Child1(pattern) => RFnParam::Pattern { span, attrs, pattern },
                Child2(_) => RFnParam::Rest { span, attrs },
                Child3(ty) => RFnParam::Type { span, attrs, ty },
            }
        }
    ));

    function_param_pattern_rule.set(MapV(Spanned((
            pattern_no_top_alt,
            w,
            ':',
            w,
            OneOf2(
                _type,
                "..."
            )
        )),
        |(span, (pat, _, _, _, two))| {
            use AnyOf2::*;
            match two {
                Child1(ty) => RFnParamPattern::Type { span, pat, ty },
                Child2(span) => RFnParamPattern::Rest { span },
            }
        }
    ));

    function_return_type_rule.set(MapV((
            "->",
            w,
            _type
        ),
        |(_, _, ty)| ty
    ));

    // --- TYPE ALIASES ---

    type_alias_rule.set(MapV(SpanOf((
            "type",
            w,
            identifier,
            Maybe(generic_params),
            w,
            Maybe((':', w, type_param_bounds, w)),
            Maybe((where_clause, w)),
            Maybe((
                '=',
                w,
                _type,
                w,
                Maybe(where_clause)
            )),
            ';'
        )),
        |span| { RTypeAlias { span } }
    ));

    // --- STRUCTS ---

    _struct_rule.set(MapV(
        OneOf2(struct_struct, tuple_struct),
        |two| {
            use AnyOf2::*;
            match two {
                Child1(t) => t,
                Child2(t) => t,
            }
        }
    ));

    struct_struct_rule.set(MapV(Spanned((
            "struct",
            w,
            identifier,
            w,
            Spanned(Maybe(generic_params)),
            Maybe((w, where_clause, w)),
            Spanned(OneOf2(
                ('{', w, Maybe(struct_fields), w, '}'),
                ';'
            ))
        )),
        |(span, (_, _, ident, _, (gspan, generics), whr, (tspan, two)))| {
            RStruct::Struct {
                span,
                ident,
                generics: {
                    match generics {
                        Some(params) => params,
                        None => RGenericParams { span: gspan, params: Vec::new() }
                    }
                },
                clause: whr.map(|(_, w, _)|w),
                fields: {
                    use AnyOf2::*;
                    match two {
                        Child1((_, _, Some(fields), _, _)) => fields,
                        _ => RStructFields { span: tspan, fields: Vec::new() }
                    }
                },
            }
        }
    ));

    tuple_struct_rule.set(MapV(Spanned((
            "struct", w, identifier, w, Spanned(Maybe((generic_params, w))), '(',
            w, Spanned(Maybe(tuple_fields)), w,
            ')', w, Maybe((where_clause, w)), ';')
        ),
        |(span, (_, _, ident, _, (gspan, generics), _, _, (fspan, fields), _, _, _, whr, _))| {
            RStruct::Tuple {
                span,
                ident,
                generics: {
                    match generics {
                        Some((params, _)) => params,
                        None => RGenericParams { span: gspan, params: Vec::new() }
                    }
                },
                clause: whr.map(|(w, _)|w),
                fields: {
                    match fields {
                        Some(fields) => fields,
                        None => RTupleFields { span: fspan, fields: Vec::new() }
                    }
                },
            }
        }
    ));

    struct_fields_rule.set(MapV(Spanned((
            Join(struct_field, (w, ',', w)),
            Maybe((w, ',', w))
        )),
        |(span, (fields, _))| {
            RStructFields { span, fields }
        }
    ));

    struct_field_rule.set(MapV(Spanned((
            ZeroOrMore((outer_attribute, w)),
            Maybe((visibility, w)),
            identifier,
            w, ':', w,
            _type
        )),
        |(span, (attrs, vis, ident, _, _, _, ty))| {
            let attrs = attrs.into_iter().map(|(a, _)|a).collect();
            let vis = vis.map(|(v, _)|v);
            RStructField { span, attrs, vis, ident, ty }
        }
    ));

    tuple_fields_rule.set(MapV(Spanned((
            Join(tuple_field, (w, ',', w)),
            Maybe((w, ','))
        )),
        |(span, (fields, _))| {
            RTupleFields { span, fields }
        }
    ));

    tuple_field_rule.set(MapV(Spanned((
            ZeroOrMore((outer_attribute, w)),
            Maybe((visibility, w)),
            _type,
        )),
        |(span, (attrs, vis, ty))| {
            let attrs = attrs.into_iter().map(|(a, _)|a).collect();
            let vis = vis.map(|(v, _)|v);
            RTupleField { span, attrs, vis, ty }
        }
    ));

    // --- ENUMERATIONS ---

    enumeration_rule.set(MapV(SpanOf((
            "enum", w, identifier, Maybe((w, generic_params)), Maybe((w, where_clause)), '{', w,
            Maybe(enum_items), w,
            '}'
        )),
        |span| { REnum { span } }
    ));

    enum_items_rule.set(SpanOf((
        Join(enum_item, (w, ',', w)),
        Maybe((w, ','))
    )));

    enum_item_rule.set(SpanOf((
        ZeroOrMore((outer_attribute, w)), Maybe((visibility, w)), identifier,
        Maybe((w, OneOf2(enum_item_tuple, enum_item_struct))),
        Maybe((w, enum_item_discriminant))
    )));

    enum_item_tuple_rule.set(SpanOf(
        ('(', w, Maybe(tuple_fields), w, ')')
    ));

    enum_item_struct_rule.set(SpanOf(
        ('{', w, Maybe(struct_fields), w, '}')
    ));

    enum_item_discriminant_rule.set(SpanOf(
        ('=', w, expression)
    ));

    // --- UNIONS ---

    _union_rule.set(MapV(Spanned((
            "union", o, identifier, w, Spanned(Maybe(generic_params)), Maybe(where_clause), w, '{', w,
            struct_fields, w,
            '}'
        )),
        |(span, (_, _, ident, _, (gspan, generics), clause, _, _, _, fields, _, _))| {
            RUnion {
                span,
                ident,
                generics: {
                    match generics {
                        Some(params) => params,
                        None => RGenericParams { span: gspan, params: Vec::new() }
                    }
                },
                clause,
                fields,
            }
        }
    ));

    // --- CONSTANT ITEMS ---
    
    constant_item_rule.set(MapV(
        Spanned(("const", w, SpanOf(OneOf2(identifier, '_')), w, ':', w, _type, w, Maybe(('=', w, expression, w)), ';')),
        |(span, (_, _, ident, _, _, _, ty, _, maybe_expr, _))| {
            RConstItem {
                span,
                ident,
                ty,
                expr: maybe_expr.map(|(_, _, expr, _)|expr)
            }
        }
    ));

    // --- STATIC ITEMS ---

    static_item_rule.set(MapV(
        Spanned(("static", w, Maybe(("mut", w)), identifier, w, ':', w, _type, Maybe(( '=', w, expression, w)), ';')),
        |(span, (_, _, m, ident, _, _, _, ty, maybe_expr, _))| {
            RStaticItem {
                span,
                mutable: m.is_some(),
                ident,
                ty,
                expr: maybe_expr.map(|(_, _, expr, _)|expr)
            }
        }
    ));

    // --- TRAITS ---

    _trait_rule.set(MapV(SpanOf((
            Maybe(("unsafe", w)), "trait", w, identifier, Maybe(generic_params), Maybe((w, ":", w, Maybe((w, type_param_bounds)))), Maybe((w, where_clause)), w, '{', w,
            ZeroOrMore(inner_attribute), w,
            ZeroOrMore(associated_item), w,
            '}'
        )),
        |span| { RTrait { span } }
    ));

    // --- IMPL ---

    implementation_rule.set(MapV(
        SpanOf(OneOf2(inherent_impl, trait_impl)),
        |span| { RImpl { span } }
    ));

    inherent_impl_rule.set((
        "impl", w, Maybe(generic_params), w, _type, w, Maybe((where_clause, w)), '{', w,
        ZeroOrMore(inner_attribute), w,
        ZeroOrMore(associated_item), w,
        '}'
    ));

    trait_impl_rule.set((
        Maybe(("unsafe", w)), "impl", w, Maybe(generic_params), Maybe('!'), type_path, w, "for", w, _type, w,
        Maybe(where_clause), w,
        (
            '{', w,
            ZeroOrMore(inner_attribute), w,
            ZeroOrMore(associated_item), w,
            '}'
        )
    ));

    // --- EXTERNAL BLOCKS ---

    extern_block_rule.set(MapV(Spanned((
            Maybe(("unsafe", w)), "extern", w, Maybe((abi, w)), w, '{', w,
            ZeroOrMore(inner_attribute), w,
            ZeroOrMore(external_item), w,
            '}'
        )),
        |(span, (u, _, _, abi, _, _, _, attrs, _, items, _, _))| {
            RExternBlock {
                span,
                is_unsafe: u.is_some(),
                abi: abi.map(|(a, _)|a),
                attrs,
                items,
            }
        }
    ));

    external_item_rule.set(MapV(Spanned((
            ZeroOrMore(outer_attribute),
            OneOf2(
                macro_invocation_semi,
                (Maybe(visibility), OneOf2(static_item, function))
            )
        )),
        |(span, (attrs, two))| {
            use AnyOf2::*;
            match two {
                Child1(invoc) => RExternalItem::MacroInvocation { span, attrs, invoc },
                Child2((vis, Child1(stat))) => RExternalItem::Static { span, attrs, vis, stat },
                Child2((vis, Child2(func))) => RExternalItem::Fn { span, attrs, vis, func },
            }
        }
    ));

    // --- GENERIC PARAMETERS ---

    generic_params_rule.set(MapV(
        Spanned(
            Surround(
                '<',
                (w, Join(generic_param, (w, ",", w)), w, Maybe((',', w))),
                '>',
                |_, _, e| e,
                |_, span, _, _| panic(span, "generic", "expected '>' after this '<'")
            )
        ),
        |(span, (_, (_, params, _, _), _))| {
            RGenericParams { span, params }
        }
    ));

    generic_param_rule.set(MapV(Spanned(
        ZeroOrMore(Spanned((
            w,
            ZeroOrMore((outer_attribute, w)),
            OneOf3(
                lifetime_param,
                type_param,
                const_param
            )
        )))),
        |(span, params)| {
            RGenericParam {
                 span,
                 segs: {
                    params.into_iter().map(|(span, (_, attrs, three))| {
                        use AnyOf3::*;
                        let attrs = attrs.into_iter().map(|(a, _)|a).collect();
                        match three {
                            Child1(param) => RGenericParamSeg::Lifetime { span, attrs, param },
                            Child2(param) => RGenericParamSeg::Type     { span, attrs, param },
                            Child3(param) => RGenericParamSeg::Const    { span, attrs, param },
                        }
                    }).collect()
                 }
            }
        }
    ));

    lifetime_param_rule.set(MapV(
        Spanned((lifetime_or_label, Maybe((w, ':', w, lifetime_bounds)))),
        |(span, (lifetime, bounds))| {
            RLifetimeParam {
                span,
                lifetime,
                bounds: {
                    match bounds {
                        Some((_, _, _, bounds)) => bounds,
                        None => Vec::new()
                    }
                }
            }
        }
    ));

    type_param_rule.set(MapV(
        Spanned((identifier, Maybe((Maybe((w, ':', w, type_param_bounds)), Maybe((w, '=', w, _type)))))),
        |(span, (ident, maybe))| {
            match maybe {
                Some((bounds, ty)) => {
                    RTypeParam { span, ident, bounds: bounds.map(|(_, _, _, b)|b), ty: ty.map(|(_, _, _, ty)|ty) }
                },
                None => {
                    RTypeParam { span, ident, bounds: None, ty: None }
                }
            }
        }
    ));

    const_param_rule.set(MapV(
        Spanned(("const", w, identifier, w, ':', w, _type, w, Maybe(OneOf3(('=', w, block_expression), identifier, (Maybe('-'), w, literal_expression))))),
        |(span, (_, _, ident, _, _a, _, ty, _, three))| {
            use AnyOf3::*;
            match three {
                Some(Child1((_, _, block)))  => RConstParam::Block { span, ident, ty, expr: block },
                Some(Child2(right_ident)) => RConstParam::Id    { span, ident, ty, right_ident },
                Some(Child3((neg, _, lit)))  => RConstParam::Lit   { span, ident, ty, neg: neg.is_some(), lit },
                None => RConstParam::Decl { span, ident, ty },
            }
        }
    ));

    // - WHERE CLAUSES -

    where_clause_rule.set(MapV(Spanned(
            Leader(
                "where",
                (w, Join(where_clause_item, (w, ',', w))),
                |_, span, _| panic(span, "where_clause", "expected one or more items after this `where`")
            )
        ),
        |(span, (_, (_, items)))| {
            RWhereClause { span, items }
        }
    ));

    where_clause_item_rule.set(MapV(
        OneOf2(
            lifetime_where_clause_item,
            type_bound_where_clause_item
        ),
        |two| {
            use AnyOf2::*;
            match two {
                Child1(c) => c,
                Child2(c) => c,
            }
        }
    ));

    lifetime_where_clause_item_rule.set(MapV(
        Spanned((lifetime, w, ':', w, lifetime_bounds)),
        |(span, (lifetime, _, _, _, bounds))| {
            RWhereClauseItem::Lifetime { span, lifetime, bounds }
        }
    ));

    type_bound_where_clause_item_rule.set(MapV(
        Spanned((Maybe((for_lifetimes, w)), _type, w, ':', w, Spanned(Maybe((w, type_param_bounds))))),
        |(span, (lifetime, ty, _, _, _, (bspan, bounds)))| {
            RWhereClauseItem::Type {
                span,
                lifetime: lifetime.map(|(l, _)|l),
                ty,
                bounds: {
                    match bounds {
                        Some((_, b)) => b,
                        None => RTypeParamBounds { span: bspan, bounds: Vec::new() }
                    }
                }
            }
        }
    ));

    // --- ASSOCIATED ITEMS ---

    associated_item_rule.set(MapV(SpanOf((
            ZeroOrMore((outer_attribute, w)),
            OneOf2(
                macro_invocation_semi,
                (Maybe((visibility, w)), OneOf3(type_alias, constant_item, function))
            )
        )),
        |span| { RAssociatedItem { span } }
    ));

    // --- ATTRIBUTES ---

    inner_attribute_rule.set(MapV(
        SpanOf(("#![", w, attr, w, ']')),
        |span| { RInnerAttr { span } }
    ));
    
    outer_attribute_rule.set(MapV(
        SpanOf(("#[", w, attr, w, ']')),
        |span| { ROuterAttr { span } }
    ));

    attr_rule.set(MapV(
        SpanOf((simple_path, Maybe((w, attr_input)))),
        |span| { RAttr { span } }
    ));

    attr_input_rule.set(
        SpanOf(OneOf2(delim_token_tree, ('=', w, expression)))
    );

    // --- META ITEM ATTRIBUTE SYNTAX ---

    meta_item_rule.set(
        SpanOf(OneOf3(
            simple_path,
            (simple_path, w, '=', w, expression),
            (simple_path, w, '(', w, Maybe(meta_seq), w, ')')
        ))
    );

    meta_seq_rule.set((
        meta_item_inner, ZeroOrMore((w, ',', w, meta_item_inner)), Maybe((w, ','))
    ));

    meta_item_inner_rule.set(OneOf2(meta_item, expression));

    meta_word_rule.set(identifier);

    meta_name_value_str_rule.set((
        identifier, w, '=', w, OneOf2(string_literal, raw_string_literal)
    ));

    meta_list_paths_rule.set(
        (identifier, w, '(', w, Maybe((simple_path, ZeroOrMore((',', simple_path)), Maybe((w, ',')))), w, ')')
    );

    meta_list_idents_rule.set(
        (identifier, w, '(', w, Maybe((identifier, ZeroOrMore((w, ',', w, identifier)), Maybe((w, ',')))), w, ')')
    );

    // --- STATEMENTS ---

    statement_rule.set(Map(MapV(
        OneOf4(
            item,
            let_statement,
            expression_statement,
            macro_invocation_semi
        ),
        |four| {
            use AnyOf4::*;
            match four {
                Child1(item) => RStatement::Item(item),
                Child2(item) => RStatement::Let(item),
                Child3(item) => RStatement::Expr(item),
                Child4(item) => RStatement::Macro(item),
            }
        }
    ), |r| { println!("Statement: {:?}", r); r}));

    let_statement_rule.set(MapV(Spanned((
            ZeroOrMore((outer_attribute, w)), "let", w, pattern_no_top_alt, w,
            Maybe((w, ':', w, _type, w)), Maybe((w, '=', w, expression, w,
            Maybe(("else", w, block_expression, w)))), ';')
        ),
        |(span, (_, _, _, pattern, _, maybe_ty, maybe_assign, _))| {
            RLetStatement {
                span,
                pattern,
                ty: maybe_ty.map(|(_, _, _, ty, _)| ty),
                right: maybe_assign.map(|(_, _, _, expr, _, maybe_else)| (expr, maybe_else.map(|(_, _, block, _)|block))),
            }
        }
    ));

    expression_statement_rule.set(MapV(
        OneOf2(
            Map((expression_without_block, w, ';'), |res| { println!("Expr No Block {:?}", res); res }),
            Map((expression_with_block, Maybe((w, ';'))), |res| { println!("Expr With Block {:?}", res); res }),
        ),
        |choice| {
            use AnyOf2::*;
            match choice {
                Child1((expr, _, _)) => expr,
                Child2((expr, _)) => expr,
            }
        }
    ));

    // --- EXPRESSIONS ---

    expression_rule.set(LRec(Map(Funnel2(
        expression_without_block,
        expression_with_block,
    ), |r| { println!("Expr: {:?}", r); r})));

    expression_without_block_rule.set(LRec(MapV((
            ZeroOrMore((outer_attribute, w)),
            Funnel21(
                Map(MapV(operator_expression,   |v| RExpr::Op(Box::new(v))), |r| { println!("Op Expr: {:?}", r); r}),
                MapV(index_expression,          |v| RExpr::Index(Box::new(v))),
                MapV(grouped_expression,        |v| RExpr::Group(Box::new(v))),
                MapV(array_expression,          |v| RExpr::Array(Box::new(v))),
                MapV(await_expression,          |v| RExpr::Await(Box::new(v))),
                MapV(tuple_expression,          |v| RExpr::Tuple(Box::new(v))),
                MapV(tuple_indexing_expression, |v| RExpr::TupleIndexing(Box::new(v))),
                MapV(struct_expression,         |v| RExpr::Struct(Box::new(v))),
                MapV(call_expression,           |v| RExpr::Call(Box::new(v))),
                MapV(method_call_expression,    |v| RExpr::MethodCall(Box::new(v))),
                MapV(field_expression,          |v| RExpr::Field(Box::new(v))),
                MapV(closure_expression,        |v| RExpr::Closure(Box::new(v))),
                MapV(async_block_expression,    |v| RExpr::Async(Box::new(v))),
                MapV(continue_expression,       |v| RExpr::Continue(Box::new(v))),
                MapV(break_expression,          |v| RExpr::Break(Box::new(v))),
                MapV(return_expression,         |v| RExpr::Return(Box::new(v))),
                MapV(underscore_expression,     |v| RExpr::Underscore(Box::new(v))),
                MapV(macro_invocation,          |v| RExpr::MacroInvocation(Box::new(v))),
                MapV(path_expression,           |v| RExpr::Path(Box::new(v))),
                MapV(range_expression,          |v| RExpr::Range(Box::new(v))),
                MapV(literal_expression,        |v| RExpr::Lit(Box::new(v))),
            )
        ),
        |(_, expr)| { expr }
    )));

    expression_with_block_rule.set(LRec(MapV((
            ZeroOrMore((outer_attribute, w)),
            Funnel6(
                MapV(block_expression,        |v| RExpr::Block(Box::new(v))),
                MapV(unsafe_block_expression, |v| RExpr::UnsafeBlock(Box::new(v))),
                MapV(loop_expression,         |v| RExpr::Loop(Box::new(v))),
                MapV(if_expression,           |v| RExpr::If(Box::new(v))),
                MapV(if_let_expression,       |v| RExpr::If(Box::new(v))),
                MapV(match_expression,        |v| RExpr::MatchExpr(Box::new(v)))
            )
        ),
        |(_, six)| { six }
    )));

    // - LITERAL EXPRESSION -

    literal_expression_rule.set(MapV(
        Map(OneOf10(
            char_literal,
            string_literal,
            raw_string_literal,
            byte_literal,
            byte_string_literal,
            raw_byte_string_literal,
            integer_literal,
            float_literal,
            "true",
            "false"
        ), |res| { println!("\nLit Expr: {:?}", res); res }),
        |any| {
            use AnyOf10::*;
            match any {
                Child1(v) => RLit::Char(v),
                Child2(v) => RLit::String(v),
                Child3(v) => RLit::RawString(v),
                Child4(v) => RLit::Byte(v),
                Child5(v) => RLit::ByteString(v),
                Child6(v) => RLit::RawByteString(v),
                Child7(int)   => RLit::Integer(int),
                Child8(float) => RLit::Float(float),
                Child9(span)  => RLit::Bool(RBoolLit::True  { span }),
                Child10(span) => RLit::Bool(RBoolLit::False { span }),
            }
        }
    ));

    path_expression_rule.set(MapV(
        OneOf2(
            path_in_expression,
            qualified_path_in_expression
        ),
        |two| {
            use AnyOf2::*;
            match two {
                Child1(path) => RPathExpr::Path(path),
                Child2(path) => RPathExpr::Qualified(path),
            }
        }
    ));

    // - BLOCK_EXPRESSIONS -

    block_expression_rule.set(Map(MapV(Spanned(Surround(
            '{',
            ( 
                w,
                ZeroOrMore(inner_attribute),
                w,
                Map(Maybe(statements), |res| { println!("Statements: {:?}", res); res }),
                w,
            ),
            '}',
            |_, _, e| e,
            |_, s, _, _| panic(s, "block_expression", "expected matching '}' after this '{'")
        )),
        |(span, (_, (_, _, _, statements, _), _))| {
            RBlockExpr { span, statements: statements.unwrap_or_else(|| Vec::new()) }
        }
    ), |r| { println!("Block Expr: {:?}", r); r}));

    statements_rule.set(MapV(
        (Join(statement, w), Maybe((w, expression_without_block))),
        |(mut statements, expr_block)| {
            if let Some((_, expr)) = expr_block {
                statements.push(RStatement::Expr(expr));
            }
            statements
        }
    ));

    async_block_expression_rule.set(MapV(
        Spanned(("async", w, Maybe(("move", w)), block_expression)),
        |(span, (_, _, mmove, block))| { RAsyncBlockExpr { span, is_move: mmove.is_some(), block } }
    ));

    unsafe_block_expression_rule.set(MapV(
        Spanned(("unsafe", w, block_expression)),
        |(span, (_, _, block))| { RUnsafeBlockExpr { span, block } }
    ));

    // - Operator Expressions -
    operator_expression_rule.set(
        Funnel10(
            borrow_expression,
            deref_expression,
            error_propogation_expression,
            negation_expression,
            arithmetic_or_logical_expression,
            comparison_expression,
            lazy_boolean_expression,
            type_cast_expression,
            assignment_expression,
            compound_assignment_expression
        ),
    );

    borrow_expression_rule.set(MapV(
        Spanned(('&', Maybe('&'), Maybe((w, "mut")), w, expression)),
        |(span, (_, d, m, _, target))| {
            if let Some(_) = d {
                ROpExpr::BorrowBorrow { span, mutable: m.is_some(), target }
            } else {
                ROpExpr::Borrow { span, mutable: m.is_some(), target }
            }
        }
    ));

    deref_expression_rule.set(MapV(
        Spanned(('*', w, expression)),
        |(span, (_, _, target))| { ROpExpr::Deref { span, target } }
    ));

    error_propogation_expression_rule.set(MapV(
        Spanned((expression, w, '?')),
        |(span, (target, _, _))| { ROpExpr::ErrorProp { span, target } }
    ));

    negation_expression_rule.set(MapV(
        Spanned(OneOf2(
            ('-', w, expression),
            ('!', w, expression)
        )),
        |(span, two)| {
            use AnyOf2::*;
            match two {
                Child1((_, _, target)) => ROpExpr::SubNegate { span, target },
                Child2((_, _, target)) => ROpExpr::NotNegate { span, target },
            }
        }
    ));

    arithmetic_or_logical_expression_rule.set(
        Funnel10(
            MapV(Spanned((expression, w, '*' , w, expression)), |(span, (left, _, _, _, right))| ROpExpr::Mul    { span, left, right }),
            MapV(Spanned((expression, w, '/' , w, expression)), |(span, (left, _, _, _, right))| ROpExpr::Div    { span, left, right }),
            MapV(Spanned((expression, w, '%' , w, expression)), |(span, (left, _, _, _, right))| ROpExpr::Mod    { span, left, right }),
            MapV(Spanned((expression, w, '&' , w, expression)), |(span, (left, _, _, _, right))| ROpExpr::BitAnd { span, left, right }),
            MapV(Spanned((expression, w, '|' , w, expression)), |(span, (left, _, _, _, right))| ROpExpr::BitOr  { span, left, right }),
            MapV(Spanned((expression, w, '^' , w, expression)), |(span, (left, _, _, _, right))| ROpExpr::BitXOr { span, left, right }),
            MapV(Spanned((expression, w, "<<", w, expression)), |(span, (left, _, _, _, right))| ROpExpr::LShift { span, left, right }),
            MapV(Spanned((expression, w, ">>", w, expression)), |(span, (left, _, _, _, right))| ROpExpr::RShift { span, left, right }),
            Map(MapV(Spanned((expression, w, '+' , w, expression)), |(span, (left, _, _, _, right))| ROpExpr::Add    { span, left, right }), |r| { println!("Add: {:?}", r); r}),
            MapV(Spanned((expression, w, '-' , w, expression)), |(span, (left, _, _, _, right))| ROpExpr::Sub   { span, left, right }),
        )
    );

    comparison_expression_rule.set(MapV(
        Spanned(OneOf6(
            (expression, w, "==", w, expression),
            (expression, w, "!=", w, expression),
            (expression, w, '>' , w, expression),
            (expression, w, '<' , w, expression),
            (expression, w, ">=", w, expression),
            (expression, w, "<=", w, expression),
        )),
        |(span, six)| {
            use AnyOf6::*;
            match six {
                Child1((left, _, _, _, right)) => ROpExpr::Eq { span, left, right },
                Child2((left, _, _, _, right)) => ROpExpr::NotEq { span, left, right },
                Child3((left, _, _, _, right)) => ROpExpr::LessThan { span, left, right },
                Child4((left, _, _, _, right)) => ROpExpr::GreaterThan { span, left, right },
                Child5((left, _, _, _, right)) => ROpExpr::LessThanOrEq { span, left, right },
                Child6((left, _, _, _, right)) => ROpExpr::GreaterThanOrEq { span, left, right },
            }
        }
    ));

    lazy_boolean_expression_rule.set(MapV(
        Spanned(OneOf2(
            (expression, w, "||", w, expression),
            (expression, w, "&&", w, expression),
        )),
        |(span, two)| {
            use AnyOf2::*;
            match two {
                Child1((left, _, _, _, right)) => ROpExpr::LogicOr  { span, left, right },
                Child2((left, _, _, _, right)) => ROpExpr::LogicAnd { span, left, right },
            }
        }
    ));

    type_cast_expression_rule.set(MapV(
        Spanned((expression, w, "as", w, type_no_bounds)),
        |(span, (left, _, _, _, ty))| { ROpExpr::TypeCast { span, left, ty } }
    ));

    assignment_expression_rule.set(MapV(
        Spanned((expression, w, '=', w, expression)),
        |(span, (left, _, _, _, right))| { ROpExpr::Assign { span, left, right } }
    ));

    compound_assignment_expression_rule.set(MapV(
        Spanned(OneOf10(
            (expression, w, "+=", w, expression),
            (expression, w, "-=", w, expression),
            (expression, w, "*=", w, expression),
            (expression, w, "/=", w, expression),
            (expression, w, "%=", w, expression),
            (expression, w, "&=", w, expression),
            (expression, w, "|=", w, expression),
            (expression, w, "^=", w, expression),
            (expression, w, "<<=", w, expression),
            (expression, w, ">>=", w, expression),
        )),
        |(span, choice)| {
            use AnyOf10::*;
            match choice {
                Child1 ((left, _, _, _, right)) => ROpExpr::AddAssign { span, left, right },
                Child2 ((left, _, _, _, right)) => ROpExpr::SubAssign { span, left, right },
                Child3 ((left, _, _, _, right)) => ROpExpr::MulAssign { span, left, right },
                Child4 ((left, _, _, _, right)) => ROpExpr::DivAssign { span, left, right },
                Child5 ((left, _, _, _, right)) => ROpExpr::ModAssign { span, left, right },
                Child6 ((left, _, _, _, right)) => ROpExpr::BitAndAssign { span, left, right },
                Child7 ((left, _, _, _, right)) => ROpExpr::BitOrAssign  { span, left, right },
                Child8 ((left, _, _, _, right)) => ROpExpr::BitXOrAssign { span, left, right },
                Child9 ((left, _, _, _, right)) => ROpExpr::LShiftAssign { span, left, right },
                Child10((left, _, _, _, right)) => ROpExpr::RShiftAssign { span, left, right },
            }
        }
    ));

    // - GROUPED EXPRESSIONS -

    grouped_expression_rule.set(MapV(
        Spanned(('(', w, expression, w, ')')),
        |(span, (_, _, expr, _, _))| { RGroupExpr { span, expr } }
    ));

    // - ARRAY EXPRESSIONS -

    array_expression_rule.set(MapV(
        Spanned(('[', w, Spanned(Maybe(array_elements)), w, ']')),
        |(span, (_, _, (espan, elements), _, _))| {
            RArrayExpr {
                span,
                elements: elements.unwrap_or_else(
                    || RArrayElements::List { span: espan, elements: Vec::new() }
                )
            }
        }
    ));

    array_elements_rule.set(MapV(
        Spanned(OneOf2(
            (expression, w, ';', w, expression),
            (expression, ZeroOrMore((w, ',', w, expression)), Maybe((w, ','))),
        )),
        |(span, three)| {
            use AnyOf2::*;
            match three {
                Child1((duplicate, _, _, _, num_times)) => {
                    RArrayElements::Duplicate { span, duplicate, num_times }
                },
                Child2((expr, exprs, _)) => {
                    let mut exprs: Vec<RExpr> = exprs.into_iter().map(|(_, _, _, e)|e).collect();
                    exprs.insert(0, expr);
                    RArrayElements::List { span, elements: exprs }
                },
            }
        }
    ));

    index_expression_rule.set(MapV(
        Spanned((expression, '[', w, expression, w, ']')),
        |(span, (to_index, _, _, index_with, _, _))| {
            RIndexExpr { span, to_index, index_with }
        }
    ));

    // - TUPLE_EXPRESSION -

    tuple_expression_rule.set(MapV(
        Spanned(('(', w, Maybe(tuple_elements), w, ')')),
        |(span, (_, _, elems, _, _))| {
            RTupleExpr { 
                span,
                elems: elems.unwrap_or_else(||Vec::new())
            }
        }
    ));

    tuple_elements_rule.set(MapV(
        (ZeroOrMore((expression, w, ',', w)), Maybe(expression)),
        |(exprs, expr)| {
            let mut exprs: Vec<RExpr> = exprs.into_iter().map(|(e, _, _, _)|e).collect();
            if let Some(expr) = expr {
                exprs.push(expr);
            }
            exprs
        }
    ));

    tuple_indexing_expression_rule.set(MapV(
        Spanned((expression, '.', tuple_index)),
        |(span, (tuple, _, index))| {
            RTupleIndexingExpr { span, tuple, index }
        }
    ));

    // - STRUCT EXPRESSIONS -

    struct_expression_rule.set(MapV(
        Spanned(OneOf3(
            struct_expr_struct,
            struct_expr_tuple,
            struct_expr_unit
        )),
        |(span, _expr)| {
            RStructExpr { span }
        }
    ));

    struct_expr_struct_rule.set(
        (path_in_expression, '{', w, Maybe((struct_expr_fields, struct_base)), w, '}')
    );

    struct_expr_fields_rule.set(
        (struct_expr_field, ZeroOrMore((w, ',', w, struct_expr_field)), OneOf2((w, ',', w, struct_base), Maybe((w, ','))))
    );

    struct_expr_field_rule.set((
        ZeroOrMore(outer_attribute),
        OneOf2(
            identifier,
            (OneOf2(identifier, tuple_index), w, ':', w, expression)
        )
    ));

    struct_base_rule.set((
        "..", w, expression
    ));

    struct_expr_tuple_rule.set((
        path_in_expression, '(', w,
        Maybe((expression, ZeroOrMore((w, ',', w, expression)), Maybe((w, ','))))
    ));

    struct_expr_unit_rule.set(path_in_expression);

    // - CALL EXPRESSIONS -

    call_expression_rule.set(MapV(
        Spanned((expression, '(', w, Maybe(call_params), w, ')')),
        |(span, _)| {
            RCallExpr { span }
        }
    ));

    call_params_rule.set(
        (expression, w, ZeroOrMore((w, ',', w, expression)), Maybe((w, ',')))
    );

    // - METHOD CALL EXPRESSIONS -

    method_call_expression_rule.set(MapV(
        Spanned((expression, '.', path_expr_segment, '(', w, Maybe(call_params), w, ')')),
        |(span, _)| {
            RCallExpr { span }
        }       
    ));

    // - FIELD ACCESS EXPRESSIONS

    field_expression_rule.set(MapV(
        Spanned((expression, '.', identifier)),
        |(span, _)| {
            RFieldExpr { span }
        }
    ));

    // - CLOSURE EXPRESSION -

    closure_expression_rule.set(MapV(Spanned((
            Maybe("move"),
            OneOf2(
                ('|', w, '|'),
                ('|', w, Maybe(closure_parameters), w, '|')
            ),
            OneOf2(
                expression,
                ("->", w, type_no_bounds, w, block_expression)
            )
        )),
        |(span, _)| {
            RClosureExpr {
                span
            }
        }
    ));

    closure_parameters_rule.set(
        (closure_param, ZeroOrMore((w, ',', w, closure_param)), Maybe((w, ',')))
    );

    closure_param_rule.set(
        (ZeroOrMore((outer_attribute, w)), pattern_no_top_alt, Maybe((w, ':', w, _type)))
    );

    // - LOOPS AND OTHER BREAKABLE EXPRESSIONS -

    loop_expression_rule.set(MapV(Spanned((
            Maybe(loop_label),
            OneOf5(
                infinite_loop_expression,
                predicate_loop_expression,
                predicate_pattern_loop_expression,
                iterator_loop_expression,
                label_block_expression,
            )
        )),
        |(span, (label, five))| {
            use AnyOf5::*;
            RLoopExpr {
                span,
                label,
                ty: match five {
                    Child1(inf_loop) => inf_loop,
                    Child2(while_loop) => while_loop,
                    Child3(while_let_loop) => while_let_loop,
                    Child4(for_loop) => for_loop,
                    Child5(expr) => expr,
                }
            }
        }
    ));

    infinite_loop_expression_rule.set(MapV(
        Spanned(("loop", w, block_expression)),
        |(span, (_, _, body))| { RLoop::Infinite { span, body } }
    ));

    predicate_loop_expression_rule.set(MapV(
        Spanned(("while", w, Not(struct_expression), expression, w, block_expression)),
        |(span, (_, _, _, expr, _, body))| { RLoop::While { span, expr, body } }
    ));

    predicate_pattern_loop_expression_rule.set(MapV(
        Spanned(("while", w, "let", w, pattern, w, '=', w, Not(lazy_boolean_expression), scrutinee, w, block_expression)),
        |(span, (_, _, _, _, pattern, _, _, _, _, expr, _, body))| {
            RLoop::WhileLet { span, pattern, expr, body }
        }
    ));

    iterator_loop_expression_rule.set(MapV(
        Spanned(("for", w, pattern, w, "in", w, Not(struct_expression), expression, w, block_expression)),
        |(span, (_, _, pattern, _, _, _, _, expr, _, body))| {
            RLoop::For { span, pattern, expr, body }
        }
    ));

    loop_label_rule.set(MapV(
        (lifetime_or_label, w, ':'),
        |(l, _, _)| l
    ));

    break_expression_rule.set(MapV(
        Spanned(("break", Maybe((w, lifetime_or_label)), Maybe((w, expression)))),
        |(span, (_, lifetime, expr))| {
            RBreakExpr { span, lifetime: lifetime.map(|(_, l)|l), expr: expr.map(|(_, e)|e) }
        }
    ));

    label_block_expression_rule.set(MapV(
        Spanned(block_expression),
        |(span, body)| {
            RLoop::Expr { span, body }
        }
    ));

    continue_expression_rule.set(MapV(
        Spanned(("continue", Maybe((w, lifetime_or_label)))),
        |(span, (_, label))| {
            RContinueExpr { span, label: label.map(|(_, l)|l) }
        }
    ));

    // - RANGE EXPRESSIONS -

    range_expression_rule.set(
        Funnel6(
            range_expr,
            range_from_expr,
            range_to_expr,
            range_full_expr,
            range_inclusive_expr,
            range_to_inclusive_expr
        )
    );

    range_expr_rule.set(MapV(
        Spanned((expression, w, "..", w, expression)),
        |(span, (left, _, _, _, right))| { RRangeExpr::Range { span, left, right } }
    ));

    range_from_expr_rule.set(MapV(
        Spanned((expression, w, "..")),
        |(span, (left, _, _))| { RRangeExpr::RangeFrom { span, left } }
    ));

    range_to_expr_rule.set(MapV(
        Spanned(("..", w, expression)),
        |(span, (_, _, right))| { RRangeExpr::RangeTo { span, right } }
    ));

    range_full_expr_rule.set(MapV(
        Spanned(".."),
        |(span, _)| { RRangeExpr::RangeFull { span } }
    ));

    range_inclusive_expr_rule.set(MapV(
        Spanned((expression, w, "..=", w, expression)),
        |(span, (left, _, _, _, right))| { RRangeExpr::RangeInclusive { span, left, right } }
    ));

    range_to_inclusive_expr_rule.set(MapV(
        Spanned(("..=", w, expression)),
        |(span, (_, _, right))| { RRangeExpr::RangeToInclusive { span, right } }
    ));

    // - if AND if let EXPRESSIONS -

    if_expression_rule.set(MapV(Spanned((
            "if", w, Not(struct_expression), expression, w, block_expression,
            Maybe((w, "else", w, OneOf3(block_expression, if_expression, if_let_expression)))
        )),
        |(span, (_, _, _, expr, _, body, maybe_else))| {
            match maybe_else {
                Some((_, _, _, three)) => {
                    use AnyOf3::*;
                    RIfExpr::IfElse {
                        span,
                        expr,
                        body,
                        else_body: Box::new(match three {
                            Child1(block) => RIfExpr::BlockExpr(block),
                            Child2(if_expr) => if_expr,
                            Child3(if_expr) => if_expr,
                        })
                    }
                },
                None => {
                    RIfExpr::If {
                        span,
                        expr,
                        body
                    }
                }
            }
        }
    ));

    if_let_expression_rule.set(MapV(Spanned((
            "if", w, "let", w, pattern, w, '=', w, Not(lazy_boolean_expression), scrutinee, w, block_expression,
            Maybe((w, "else", w, OneOf3(block_expression, if_expression, if_let_expression)))
        )),
        |(span, (_, _, _, _, pattern, _, _, _, _, scrutinee, _, body, maybe_else))| {
            match maybe_else {
                Some((_, _, _, three)) => {
                    use AnyOf3::*;
                    RIfExpr::IfLetElse {
                        span,
                        pattern,
                        expr: scrutinee,
                        body,
                        else_body: Box::new(match three {
                            Child1(block) => RIfExpr::BlockExpr(block),
                            Child2(if_expr) => if_expr,
                            Child3(if_expr) => if_expr,
                        })
                    }
                },
                None => {
                    RIfExpr::IfLet {
                        span,
                        pattern,
                        expr: scrutinee,
                        body
                    }
                }
            }
        }
    ));

    // - MATCH EXPRESSIONS -

    match_expression_rule.set(MapV(SpanOf((
            "match", w, scrutinee, w, '{', w,
            ZeroOrMore(inner_attribute), w,
            Maybe(match_arms), w,
            '}'
        )),
        |span| {
            RMatchExpr { span }
        }
    ));

    scrutinee_rule.set(MapV(
        (Not(struct_expression), expression),
        |(_, expr)| { expr }
    ));

    match_arms_rule.set(
        OneOf2(
            ZeroOrMore((match_arm, w, "=>", w, OneOf2((expression_without_block, w, ','), (expression_with_block, Maybe((w, ',')))))),
            (match_arm, w, "=>", w, expression, Maybe((w, ',')))
        )
    );

    match_arm_rule.set((
        w,
        ZeroOrMore(outer_attribute),
        w,
        pattern,
        w,
        Maybe(match_arm_gaurd)
    ));

    match_arm_gaurd_rule.set(
        ("if", w, expression)
    );

    // - RETURN EXPRESSIONS -

    return_expression_rule.set(MapV(
        Spanned(("return", w, Maybe(expression))),
        |(span, (_, _, expr))| { RReturnExpr { span, expr } }
    ));

    // - AWAIT EXPRESSIONS -

    await_expression_rule.set(MapV(
        Spanned((expression, '.', "await")),
        |(span, (expr, _, _))| { RAwaitExpr { span, expr } }
    ));

    // - _ EXPRESSIONS -

    underscore_expression_rule.set(MapV(
        '_',
        |span| { RUnderscoreExpr { span } }
    ));

    // --- PATTERNS ---

    pattern_rule.set(MapV(
        Spanned((Maybe(('|', w)), pattern_no_top_alt, ZeroOrMore((w, '|', w, pattern_no_top_alt)))),
        |(span, (_, pattern, patterns))| {
            RPattern {
                span,
                patterns: {
                    let mut patterns: Vec<RSubPattern> = patterns.into_iter().map(|(_, _, _, p)|p).collect();
                    patterns.insert(0, pattern);
                    patterns
                }
            }
        }
    ));

    pattern_no_top_alt_rule.set(MapV(
        OneOf2(
            pattern_without_range,
            range_pattern
        ),
        |two| {
            use AnyOf2::*;
            match two {
                Child1(p) => p,
                Child2(r) => RSubPattern::Range(Box::new(r)),
            }
        }
    ));

    pattern_without_range_rule.set(MapV(
        OneOf9(
            literal_pattern,
            identifier_pattern,
            wildcard_pattern,
            rest_pattern,
            reference_pattern,
            struct_pattern,
            tuple_struct_pattern,
            tuple_pattern,
            OneOf4(
                grouped_pattern,
                slice_pattern,
                path_pattern,
                macro_invocation
            )
        ),
        |nine| {
            use AnyOf9::*;
            match nine {
                Child1(v) => RSubPattern::Lit(Box::new(v)),
                Child2(v) => RSubPattern::Ident(Box::new(v)),
                Child3(v) => RSubPattern::Wildcard(Box::new(v)),
                Child4(v) => RSubPattern::Rest(Box::new(v)),
                Child5(v) => RSubPattern::Ref(Box::new(v)),
                Child6(v) => RSubPattern::Struct(Box::new(v)),
                Child7(v) => RSubPattern::TupleStruct(Box::new(v)),
                Child8(v) => RSubPattern::Tuple(Box::new(v)),
                Child9(AnyOf4::Child1(v)) => RSubPattern::Grouped(Box::new(v)),
                Child9(AnyOf4::Child2(v)) => RSubPattern::Slice(Box::new(v)),
                Child9(AnyOf4::Child3(v)) => RSubPattern::Path(Box::new(v)),
                Child9(AnyOf4::Child4(v)) => RSubPattern::Macro(Box::new(v)),
            }
        }
    ));

    literal_pattern_rule.set(MapV(
        OneOf8(
            char_literal,
            byte_literal,
            string_literal,
            raw_string_literal,
            byte_string_literal,
            raw_byte_string_literal,
            Spanned((Maybe('-'), w, integer_literal)),
            Spanned((Maybe('-'), w, float_literal)),
        ),
        |eight| {
            use AnyOf8::*;
            match eight {
                Child1(v) => RSLit::Char(v),
                Child2(v) => RSLit::Byte(v),
                Child3(v) => RSLit::String(v),
                Child4(v) => RSLit::RawString(v),
                Child5(v) => RSLit::ByteString(v),
                Child6(v) => RSLit::RawByteString(v),
                Child7((span, (n, _, int))) => RSLit::Integer(RSIntLit { span, neg: n.is_some(), lit: int }),
                Child8((span, (n, _, flt))) => RSLit::Float(RSFloatLit { span, neg: n.is_some(), lit: flt }),
            }
        }
    ));

    identifier_pattern_rule.set(MapV(
        Spanned((Maybe(("ref", w)), Maybe(("mut", w)), identifier, Maybe((w, '@', w, pattern_no_top_alt)))),
        |(span, (reference, mutable, ident, test))| {
            RIdentPattern {
                span,
                reference: reference.map(|(r, _)|r),
                mutable: mutable.map(|(m, _)|m),
                ident,
                test: test.map(|(_, _, _, test)|test)
            }
        }
    ));

    wildcard_pattern_rule.set('_');

    range_pattern_rule.set(MapV(
        OneOf4(
            range_inclusive_pattern,
            range_from_pattern,
            range_to_inclusive_pattern,
            obsolete_range_pattern,
        ),
        |four| {
            use AnyOf4::*;
            match four {
                Child1(r) => r,
                Child2(r) => r,
                Child3(r) => r,
                Child4(r) => r,
            }
        }
    ));

    range_inclusive_pattern_rule.set(MapV(
        Spanned((range_pattern_bound, w, "..=", w, range_pattern_bound)),
        |(span, (left, _, _, _, right))| {
            RRangePattern::RangeInclusive { span, left, right }
        }
    ));

    range_from_pattern_rule.set(MapV(
        Spanned((range_pattern_bound, w, "..")),
        |(span, (left, _, _))| {
            RRangePattern::RangeFrom { span, left }
        }
    ));

    range_to_inclusive_pattern_rule.set(MapV(
        Spanned(("..=", w, range_pattern_bound)),
        |(span, (_, _, right))| { RRangePattern::RangeToInclusive { span, right } }
    ));

    obsolete_range_pattern_rule.set(Map(
        Spanned((range_pattern_bound, w, "...", w, range_pattern_bound)),
        |res| {
            use ParseResult::*;
            match res {
                Okay((span, _), _) => {
                    Panic(panic(span, "obsolete_range_pattern", "'...' is no longer valid Rust syntax"))
                },
                Error(e) => Error(e),
                Panic(e) => Panic(e),
            }
        }
    ));

    range_pattern_bound_rule.set(MapV(
        Spanned(OneOf5(
            char_literal,
            byte_literal,
            (Maybe(('-', w)), integer_literal),
            (Maybe(('-', w)), float_literal),
            path_expression
        )),
        |(span, five)| {
            use AnyOf5::*;
            match five {
                Child1(c) => RRangePatternBound::Char(c),
                Child2(b) => RRangePatternBound::Byte(b),
                Child3((neg, int)) => {
                    RRangePatternBound::Int {
                        span,
                        neg_sign: neg.is_some(),
                        int
                    }
                },
                Child4((neg, float)) => {
                    RRangePatternBound::Float {
                        span,
                        neg_sign: neg.is_some(),
                        float
                    }
                },
                Child5(path) => {
                    RRangePatternBound::Path { path }
                },
            }
        }
    ));

    reference_pattern_rule.set(MapV(
        Spanned((OneOf2('&', ('&', '&')), w, Maybe(("mut", w)), pattern_without_range)),
        |(span, (refs, _, mutable, pattern))| {
            let (ref1, ref2) = match refs {
                AnyOf2::Child1(ref1) => (Some(ref1), None),
                AnyOf2::Child2((ref1, ref2)) => (Some(ref1), Some(ref2)),
            };
            RRefPattern {
                span,
                ref1,
                ref2,
                mutable: mutable.map(|(_, m)|m),
                pattern,
            }
        }
    ));

    struct_pattern_rule.set(MapV(Spanned((
            path_in_expression, w, '{', w,
            Maybe(struct_pattern_elements)
        )),
        |(span, (path, _, _, _, elems))| {
            RStructPattern { span, path, elems }
        }
    ));

    struct_pattern_elements_rule.set(MapV(
        Spanned((struct_pattern_fields, Maybe((Maybe((w, ',')), struct_pattern_et_cetera)), Maybe((w, ',')))),
        |(span, (fields, et_cetera, _))| {
            RStructPatternElems {
                span,
                fields,
                et_cetera: et_cetera.is_some()
            }
        }
    ));

    struct_pattern_fields_rule.set(MapV(
            (struct_pattern_field, ZeroOrMore((w, ',', w, struct_pattern_field))),
        |(field, fields)| {
            let mut fields: Vec<RStructPatternField> = fields.into_iter().map(|(_, _, _, f)|f).collect();
            fields.insert(0, field);
            fields
        }
    ));

    struct_pattern_field_rule.set(MapV(Spanned((
            ZeroOrMore(outer_attribute),
            OneOf3(
                (tuple_index, w, ':', w, pattern),
                (identifier, w, ':', w, pattern),
                (Maybe(("ref", w)), Maybe(("mut", w)), identifier)
            )
        )),
        |(span, (_, three))| {
            use AnyOf3::*;
            match three {
                Child1((tuple_index, _, _, _, pattern)) => RStructPatternField::TupleMatch { span, tuple_index, pattern },
                Child2((ident, _, _, _, pattern)) => RStructPatternField::IdentMatch { span, ident, pattern },
                Child3((r, m, ident)) => RStructPatternField::Ident { span, reference: r.is_some(), mutable: m.is_some(), ident },
            }
        }
    ));

    struct_pattern_et_cetera_rule.set(MapV((
            ZeroOrMore((outer_attribute, w)),
            ".."
        ),
        |(_, a)| { a }
    ));

    tuple_struct_pattern_rule.set(MapV(
        Spanned((path_in_expression, '(', w, Maybe(tuple_struct_items), w, ')')),
        |(span, (path, _, _, items, _, _))| {
            RTupleStructPattern {
                span,
                path,
                items: items.unwrap_or_else(||Vec::new())
            }
        }
    ));

    tuple_struct_items_rule.set(MapV(
        (pattern, ZeroOrMore((w, ',', w, pattern)), Maybe((w, ','))),
        |(pattern, patterns, _)| {
            let mut patterns: Vec<RPattern> = patterns.into_iter().map(|(_, _, _, p)|p).collect();
            patterns.insert(0, pattern);
            patterns
        }
    ));

    tuple_pattern_rule.set(MapV(
        Spanned(('(', w, Maybe(tuple_pattern_items), w, ')')),
        |(span, (_, _, _items, _, _))| {
            RTuplePattern { span, }           
        }
    ));

    tuple_pattern_items_rule.set(
        OneOf3(
            (pattern, w, ','),
            rest_pattern,
            (pattern, OneOrMore((w, ',', w, pattern)), Maybe((w, ',')))
        )
    );

    rest_pattern_rule.set("..");

    grouped_pattern_rule.set(MapV(
        Spanned(('(', w, pattern, w, ')')),
        |(span, (_, _, pattern, _, _))| {
            RGroupedPattern { span, pattern }
        }
    ));

    slice_pattern_rule.set(MapV(
        Spanned(('[', w, Maybe(slice_pattern_items), w, ']')),
        |(span, (_, _, patterns, _, _))| {
            RSlicePattern {
                span,
                contents: patterns.unwrap_or_else(||Vec::new())
            }
        }
    ));

    slice_pattern_items_rule.set(MapV(
        (pattern, ZeroOrMore((w, ',', w, pattern)), Maybe((w, ','))),
        |(pattern, patterns, _)| {
            let mut patterns: Vec<RPattern> = patterns.into_iter().map(|(_, _, _, p)|p).collect();
            patterns.insert(0, pattern);
            patterns
        }
    ));

    path_pattern_rule.set(MapV(
        path_expression,
        |path| { RPathPattern { path } }
    ));

    // --- TRAIT AND LIFETIME BOUNDS ---

    type_param_bounds_rule.set(MapV(
        Spanned((type_param_bound, ZeroOrMore((w, '+', w, type_param_bound)), Maybe((w, '+')))),
        |(span, (param_bound, param_bounds, _))| {
            RTypeParamBounds {
                span,
                bounds: {
                    let mut bounds: Vec<RTypeParamBound> = param_bounds.into_iter().map(|(_, _, _, bound)|bound).collect();
                    bounds.insert(0, param_bound);
                    bounds
                }
            }
        }
    ));

    type_param_bound_rule.set(MapV(
        OneOf2(
            lifetime,
            trait_bound
        ),
        |two| {
            use AnyOf2::*;
            match two {
                Child1(lifetime) => RTypeParamBound::Lifetime(lifetime),
                Child2(trait_bound) => RTypeParamBound::Trait(trait_bound),
            }
        }
    ));

    trait_bound_rule.set(MapV(
        Spanned(OneOf2(
            (Maybe(('?', w)), Maybe((for_lifetimes, w)), type_path),
            ('(', w, Maybe(('?', w)), Maybe((for_lifetimes, w)), type_path, w, ')'),
        )),
        |(span, two)| {
            use AnyOf2::*;
            match two {
                Child1((n, _lifetimes, _ty)) | Child2((_, _, n, _lifetimes, _ty, _, _)) => {
                    RTraitBound {
                        span,
                        not: n.is_some(),
                    }
                },
            }
        }
    ));

    lifetime_bounds_rule.set(MapV(
        (ZeroOrMore((lifetime, w, '+', w)), Maybe(lifetime)),
        |(lifetimes, final_lifetime)| {
            let mut lifetimes: Vec<RLifetime> = lifetimes.into_iter().map(|(l, _, _, _)|l).collect();
            if let Some(final_lifetime) = final_lifetime {
                lifetimes.push(final_lifetime);
            }
            lifetimes
        }
    ));

    lifetime_rule.set(MapV(
        OneOf3(
            Spanned(('\'', "static")),
            Spanned(('\'', '_')),
            lifetime_or_label,
        ),
        |three| {
            use AnyOf3::*;
            match three {
                Child1((span, (_, stat))) => RLifetime::Static { span, stat },
                Child2((span, (_, underscore))) => RLifetime::Elided { span, underscore },
                Child3(l) => l,
            }
        }
    ));

    for_lifetimes_rule.set(MapV(
        Spanned(("for", w, generic_params)),
        |(span, (_, _, generics))| {
            RForLifetimes { span, generics }
        }
    ));

    // --- VISIBILITY AND PRIVACY ---

    visibility_rule.set(MapV(
        Spanned(OneOf5(
            "pub",
            ("pub", w, '(', w, "crate", w, ')'),
            ("pub", w, '(', w, "self", w, ')'),
            ("pub", w, '(', w, "super", w, ')'),
            ("pub", w, '(', w, "in", w, simple_path, w, ')')
        )),
        |(span, five)| {
            use AnyOf5::*;
            match five {
                Child1(_) => RVis::VisPub { span },
                Child2(_) => RVis::VisCrate { span },
                Child3(_) => RVis::VisSelf { span },
                Child4(_) => RVis::VisSuper { span },
                Child5((_, _, _, _, _, _, path, _, _)) => RVis::VisPath { span, path },
            }
        }
    ));

    _crate.parse(&TLRecMemTable::new(file), PPos::new())
}


#[cfg(test)]
mod test_new_parser {
    use super::parse_file;


    #[test]
    fn test_empty() {
        //println!("{:?}", parse_file(""));
        println!("{:?}", parse_file("fn hello() {3+4}"));
    }
}


