use std::{fmt::Display, collections::HashSet};

use crate::{parser::{ParseContext, Not, AnyV, Funnel4, AnyMemTable, Funnel6, Join, Funnel2, OneOf8, MapPValue, Funnel9, LRJoin, Funnel3, Funnel8, OneOf11, RLJoin, Funnel12, Funnel, AnyOf4, AnyOf11, AnyOf8, OJoin}, srule, symex::{SymExEngine, self, new_assert}};

use super::{ParseResult, Span, ZeroOrMore, ParseNode, SpanOf, Map, OneOf3, Spanned, OneOrMore, AnyOf3, Maybe, AnyOf2, MapV, OneOf6, Leader, Surround, End, Req, OneOf5, OneOf4, OneOf2, ParsePos, ParseStore};

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

    /// Creates a new `PPos` with the given index.
    pub fn with_index(index: usize) -> Self {
        Self { index, column: 1, line: 1 }
    }
}

impl PartialEq for PPos {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for PPos { }

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

type ExErr = ();

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExOk {
    pub cont: bool,
    pub res: Vec<SymexRes>,
    pub continues: HashSet<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SymexRes {
    symex_pos: Span<PPos>,
    res: String,
}

type ReturnResult = Result<ExOk, ExErr>;

pub trait Execute<Store: ParseStore<PPos, char> + ?Sized> {
    fn execute(&self, store: &Store, engine: &mut Vec<SymExEngine>, ids: HashSet<usize>) -> Result<ExOk, ExErr>;
}

impl <Store: ParseStore<PPos, char> + ?Sized> Execute<Store> for RCrate {
    fn execute(&self, store: &Store, engine: &mut Vec<SymExEngine>, ids: HashSet<usize>) -> Result<ExOk, ExErr> {
        let mut results = Vec::new();

        for item in self.items.iter() {
            let res = item.execute(store, engine, HashSet::new())?;
            results.extend(res.res);
        }

        Ok(ExOk { cont: true, res: results, continues: HashSet::new() })
    }
}

impl <Store: ParseStore<PPos, char> + ?Sized> Execute<Store> for RItem {
    fn execute(&self, store: &Store, engine: &mut Vec<SymExEngine>, ids: HashSet<usize>) -> Result<ExOk, ExErr> {
        match self {
            RItem::Fn {span, vis, val}
                => val.execute(store, engine, ids),
        }
    }
}

impl <Store: ParseStore<PPos, char> + ?Sized> Execute<Store> for RFn {
    fn execute(&self, store: &Store, engine: &mut Vec<SymExEngine>, ids: HashSet<usize>) -> Result<ExOk, ExErr> {
        let fn_name = &self.id.into_string(store);
        let mut ids = HashSet::from([symex::new_engine(engine, fn_name)]);

        // add the params
        for arg in &self.args {
            arg.execute(store, engine, ids.clone())?;
        }

        // execute the body now that we have the params
        let mut res = self.body.execute(store, engine, ids)?;
        res.continues.clear();
        Ok(res)
    }
}
impl <Store: ParseStore<PPos, char> + ?Sized> Execute<Store> for RBlock {
    fn execute(&self, store: &Store, engine: &mut Vec<SymExEngine>, ids: HashSet<usize>) -> Result<ExOk, ExErr> {
        let mut results = Vec::new();
        let mut continues = ids.clone();

        for id in ids {
            let mut internal_continues = HashSet::from([id]);

            for stmt in &self.statements {
                let res = stmt.execute(store, engine, internal_continues.clone())?;
                results.extend(res.res);
                internal_continues.extend(res.continues.clone());
                continues.extend(res.continues);
                if !res.cont {
                    break;
                }
            }
        }

        return Ok(ExOk { cont: true, res: results, continues });
    }
}

impl <Store: ParseStore<PPos, char> + ?Sized> Execute<Store> for RStatement {
    fn execute(&self, store: &Store, engine: &mut Vec<SymExEngine>, ids: HashSet<usize>) -> Result<ExOk, ExErr> {
        use RStatement::*; 
        match self {
            Comment{comment} => {
                match comment {
                    RComment::Symex { symex, .. } => {
                        let mut res = Vec::new();

                        for id in ids {
                            engine[id].reached_symex = true;

                            res.push(SymexRes {
                                symex_pos: symex.clone(),
                                res: engine[id].to_string()
                            });
                        }
                        
                        return Ok(ExOk { cont: true, res, continues: HashSet::new() });
                    },
                    _ => Ok(ExOk { cont: true, res: Vec::new(), continues: HashSet::new() }),
                }
            },
            Expr {expr, semi} => expr.execute(store, engine, ids),
            Return { expr, .. } => {
                let mut res = expr.execute(store, engine, ids)?;
                res.cont = false;
                Ok(res)
            },
            SColon { .. } => Ok(ExOk { cont: true, res: Vec::new(), continues: HashSet::new() }),
            If {stmt} => stmt.execute(store, engine, ids),
            Loop {stmt} => stmt.execute(store, engine, ids),
            Assign {ident, ty, equal_value,..} => {
                for id in ids {
                    engine[id].new_variable_assign(
                        ident.into_string(store),
                        ty.clone().map(|v|v.into_string(store)).unwrap_or_else(||"i32".to_string()),
                        equal_value.span().into_string(store),
                        equal_value.into_lisp(store)
                    );
                }

                Ok(ExOk { cont: true, res: Vec::new(), continues: HashSet::new() })
            }
        }
    }
}


impl <Store: ParseStore<PPos, char> + ?Sized> Execute<Store> for RParam {
    fn execute(&self, store: &Store, engine: &mut Vec<SymExEngine>, ids: HashSet<usize>) -> Result<ExOk, ExErr> {
        for id in ids {
            engine[id].new_variable(self.id.into_string(store), self.ty.into_string(store));
        }
        return Ok(ExOk { cont: true, res: Vec::new(), continues: HashSet::new() });
    }
}

impl <Store: ParseStore<PPos, char> + ?Sized> Execute<Store> for RExpr {
    fn execute(&self, store: &Store, engine: &mut Vec<SymExEngine>, ids: HashSet<usize>) -> Result<ExOk, ExErr> {
        use RExpr::*;
        match self {
            Lit(_) => {},
            Var(_) => {},
            Path(_, _) => {},
            Group { expr, .. } => return expr.execute(store, engine, ids),
            Block(b) => return b.execute(store, engine, ids),
            If(i) => return i.execute(store, engine, ids),
            Loop(l) => return l.execute(store, engine, ids),
            Call { span, ident, args } => {},
            Deref { span, star, expr } => {},
            Borrow { span, and, expr } => {},
            BorrowMut { span, and, mutable, expr } => {},
            Negate { span, neg, expr } => {},
            Not { span, not, expr } => {},
            AssignOp { span, left, op, op_span, right } => {
                for id in ids {
                    engine[id].assign_symvar_value(
                        right.span().into_string(store),
                        left.span().into_string(store),
                        right.into_lisp(store)
                    );
                }
            },
            BinOp { span, left, op, op_span, right } => { },
        }       
        Ok(ExOk { cont: true, res: Vec::new(), continues: HashSet::new() })
    }
}

impl <Store: ParseStore<PPos, char> + ?Sized> Execute<Store> for RIf {
    fn execute(&self, store: &Store, engine: &mut Vec<SymExEngine>, ids: HashSet<usize>) -> Result<ExOk, ExErr> {
        let mut res = ExOk { cont: true, res: Vec::new(), continues: HashSet::new() };

        for bad_path in ids {
            for (expr, block) in self.ifs.iter() {
                let good_path = new_assert(engine, bad_path, expr.span().into_string(store), expr.into_lisp(store));
                let result = block.execute(store, engine, HashSet::from([good_path]))?;
                res.res.extend(result.res);
                res.continues.extend(result.continues);
            }

            if let Some(block) = &self.else_stmt {
                let result = block.execute(store, engine, HashSet::from([bad_path]))?;
                res.res.extend(result.res);
                res.continues.extend(result.continues);
            }
        }

        Ok(res)
    }
}

impl <Store: ParseStore<PPos, char> + ?Sized> Execute<Store> for RLoop {
    fn execute(&self, store: &Store, engine: &mut Vec<SymExEngine>, id: HashSet<usize>) -> Result<ExOk, ExErr> {
        Ok(ExOk { cont: true, res: Vec::new(), continues: HashSet::new() })
    }
}

pub trait IntoLisp<Store: ParseStore<Pos, char> + ?Sized, Pos: ParsePos> {
    fn into_lisp(&self, store: &Store) -> String;
}

impl <Store: ParseStore<Pos, char> + ?Sized, Pos: ParsePos> IntoLisp<Store, Pos> for Span<Pos> {
    fn into_lisp(&self, store: &Store) -> String {
        self.into_string(store)
    }
}

impl <Store: ParseStore<Pos, char> + ?Sized, Pos: ParsePos, T: IntoLisp<Store, Pos> + ?Sized> IntoLisp<Store, Pos> for Box<T> {
    fn into_lisp(&self, store: &Store) -> String {
        (**self).into_lisp(store)
    }
}

impl <Store: ParseStore<Pos, char> + ?Sized, Pos: ParsePos, T: IntoLisp<Store, Pos>> IntoLisp<Store, Pos> for Vec<T> {
    fn into_lisp(&self, store: &Store) -> String {
        let mut out = String::new();
        if self.len() > 0 {
            out.extend(self[0].into_lisp(store).chars());

            for i in 1..self.len() {
                out.push(' ');
                out.extend(self[i].into_lisp(store).chars());
            }
        }
        out
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RParam {
    pub mutable: Option<Span<PPos>>,
    pub id: Span<PPos>,
    pub ty: RType,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RBlock {
    pub span: Span<PPos>,
    pub statements: Vec<RStatement>,
}

/// 
/// An if statement or chain of if statments.
/// 
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RIf {
    /// The span of the `RIf`.
    span: Span<PPos>,
    /// The list of if statements.
    ifs: Vec<(RExpr, RBlock)>,
    /// The optional else block to run if all if statements fail.
    else_stmt: Option<RBlock>
}

impl RIf {
    pub fn span(&self) -> Span<PPos> {
        self.span.clone()
    } 
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RLoop {
    Infinite {
        span: Span<PPos>,
        block: RBlock,
    },
    While {
        span: Span<PPos>,
        expr: RExpr,
        block: RBlock,
    },
    For {
        span: Span<PPos>,
        var: Span<PPos>,
        expr: RExpr,
        block: RBlock,
    },
}

impl RLoop {
    pub fn span(&self) -> Span<PPos> {
        use RLoop::*;
        match self {
            Infinite { span, .. } => span.clone(),
            While { span, .. } => span.clone(),
            For { span, .. } => span.clone(),
        }
    } 
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RType {
    Array {
        span: Span<PPos>,
        /// The type of every item in the array (since arrays can only hold 1
        /// type of item, this is that type).
        item_type: Box<Self>,
        /// The number of items in the array.
        item_number: PPos,
    },
    Tuple {
        span: Span<PPos>,
        /// The types in the tuple, stored in the same order as given.
        types: Vec<Self>,
    },
    Template {
        span: Span<PPos>,
        /// The name of the type.
        name: Span<PPos>,
        /// The arguments for the type.
        args: Vec<Self>,
    },
}

impl RType {
    pub fn into_string<Store: ParseStore<PPos, char> + ?Sized>(&self, store: &Store) -> String {
        match self {
            RType::Array { span, .. } => span.into_string(store),
            RType::Tuple { span, .. } => span.into_string(store),
            RType::Template { span, .. } => span.into_string(store),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RExpr {
    Lit(RLit),
    Var(Span<PPos>),
    Path(Span<PPos>, Vec<Span<PPos>>),
    Block(RBlock),
    If(Box<RIf>),
    Loop(Box<RLoop>),
    Group { span: Span<PPos>, expr: Box<RExpr> },

    Call      { span: Span<PPos>, ident: Span<PPos>, args: Vec<RExpr> },

    Deref     { span: Span<PPos>, star: Span<PPos>, expr: Box<RExpr> },
    Borrow    { span: Span<PPos>, and: Span<PPos>, expr: Box<RExpr> },
    BorrowMut { span: Span<PPos>, and: Span<PPos>, mutable: Span<PPos>, expr: Box<RExpr> },
    Negate    { span: Span<PPos>, neg: Span<PPos>, expr: Box<RExpr> },
    Not       { span: Span<PPos>, not: Span<PPos>, expr: Box<RExpr> },

    AssignOp  { span: Span<PPos>, left: Box<RExpr>, op: AssignOp, op_span: Span<PPos>, right: Box<RExpr> },
    BinOp     { span: Span<PPos>, left: Box<RExpr>, op: BinOp, op_span: Span<PPos>, right: Box<RExpr> },
}

impl RExpr {
    pub fn span(&self) -> Span<PPos> {
        use RExpr::*;
        match self {
            Lit(l) => l.span(),
            Var(v) => v.clone(),
            Path(p, _) => p.clone(),
            Group { span, .. } => span.clone(),
            Block(b) => b.span.clone(),
            If(f) => f.span().clone(),
            Loop(l) => l.span().clone(),
            Call { span, .. } => span.clone(),
            Deref { span, .. } => span.clone(),
            Borrow { span, .. } => span.clone(),
            BorrowMut { span, .. } => span.clone(),
            Negate { span, .. } => span.clone(),
            Not { span, .. } => span.clone(),
            AssignOp { span, .. } => span.clone(),
            BinOp { span, .. } => span.clone(),
        }
    }
}


impl <Store: ParseStore<PPos, char> + ?Sized> IntoLisp<Store, PPos> for RExpr {
    fn into_lisp(&self, store: &Store) -> String {
        //use RExpr::*;
        match self {
            RExpr::Lit(l) => l.into_lisp(store),
            RExpr::Var(v) => v.into_lisp(store),
            RExpr::Path(span, _) => span.into_lisp(store),
            RExpr::Block(_) => format!("Block"),
            RExpr::If(_) => format!("IfStatement"),
            RExpr::Loop(_) => format!("Loop"),
            RExpr::Group { expr, .. } => expr.into_lisp(store),
            RExpr::Call { ident, args, .. } => {
                format!("({} {})", ident.into_lisp(store), args.into_lisp(store))
            },
            RExpr::Deref { expr, .. } => {
                format!("*{}", expr.into_lisp(store))
            },
            RExpr::Borrow { expr, .. } => {
                format!("&{}", expr.into_lisp(store))
            },
            RExpr::BorrowMut { expr, .. } => {
                format!("&{}", expr.into_lisp(store))
            },
            RExpr::Negate { expr, .. } => {
                format!("-{}", expr.into_lisp(store))
            },
            RExpr::Not { expr, .. } => {
                format!("!{}", expr.into_lisp(store))
            },
            RExpr::AssignOp { left, op, op_span, right, .. } => {
                format!("(setq {} {})", left.into_lisp(store), right.into_lisp(store))
            },
            RExpr::BinOp { left, op, op_span, right, .. } => {
                let op = match op {
                    BinOp::As => "as",
                    BinOp::EqEq => "=",
                    BinOp::NotEq => return format!("(not (= {} {}))", left.into_lisp(store), right.into_lisp(store)),
                    BinOp::LessThan => "<",
                    BinOp::MoreThan => ">",
                    BinOp::LessThanEq => "<=",
                    BinOp::MoreThanEq => ">=",
                    BinOp::And => "&&",
                    BinOp::Or => "||",
                    BinOp::LSh => "<<",
                    BinOp::RSh => ">>",
                    BinOp::BitAnd => "&",
                    BinOp::BitOr => "|",
                    BinOp::BitXOr => "^",
                    BinOp::Add => "+",
                    BinOp::Sub => "-",
                    BinOp::Div => "/",
                    BinOp::Mod => "%",
                    BinOp::Mul => "*",
                };
                format!("({} {} {})", op.to_string(), left.into_lisp(store), right.into_lisp(store))
            },
    
        }
    }
}



#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BinOp {
    // Type Cast
    As,

    // Logical
    EqEq,
    NotEq,
    LessThan,
    MoreThan,
    LessThanEq,
    MoreThanEq,
    And,
    Or,

    // Bits
    LSh,
    RSh,
    BitAnd,
    BitOr,
    BitXOr,

    // Arithmetic
    Add,
    Sub,
    Div,
    Mod,
    Mul,
}

impl AsRef<str> for BinOp {
    fn as_ref(&self) -> &str {
        match self.clone() {
            BinOp::As => "as",
            BinOp::EqEq => "=",
            BinOp::NotEq => "/=",
            BinOp::LessThan => "<",
            BinOp::MoreThan => ">",
            BinOp::LessThanEq => "<=",
            BinOp::MoreThanEq => ">=",
            BinOp::And => "&&",
            BinOp::Or => "||",
            BinOp::LSh => "<<",
            BinOp::RSh => ">>",
            BinOp::BitAnd => "&",
            BinOp::BitOr => "|",
            BinOp::BitXOr => "^",
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Div => "/",
            BinOp::Mod => "%",
            BinOp::Mul => "*",
        }
    }
}

impl <Store: ParseStore<Pos, char> + ?Sized, Pos: ParsePos> IntoLisp<Store, Pos> for BinOp {
    fn into_lisp(&self, store: &Store) -> String {
        format!("{}", self.as_ref())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AssignOp {
    // Assignment
    Assign,

    // Compound Assignment
    AAdd,
    ASub,
    AMul,
    ADiv,
    AMod,
    AAnd,
    AOr,
    AXOr,
    ALSh,
    ARSh,
}

impl AsRef<str> for AssignOp {
    fn as_ref(&self) -> &str {
        match self.clone() {
            AssignOp::Assign => "=",
            AssignOp::AAdd => "+=",
            AssignOp::ASub => "-=",
            AssignOp::AMul => "*=",
            AssignOp::ADiv => "/=",
            AssignOp::AMod => "%=",
            AssignOp::AAnd => "&=",
            AssignOp::AOr => "|=",
            AssignOp::AXOr => "^=",
            AssignOp::ALSh => "<<=",
            AssignOp::ARSh => ">>=",
        }
    }
}

impl <Store: ParseStore<Pos, char> + ?Sized, Pos: ParsePos> IntoLisp<Store, Pos> for AssignOp {
    fn into_lisp(&self, store: &Store) -> String {
        format!("{}", self.as_ref())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RStatement {
    Comment {
        comment: RComment,
    },
    /// An expression with an optional semicolon after it (semicolon can only be omitted if it is at the end of a group).
    Expr {
        expr: RExpr,
        semi: Option<Span<PPos>>
    },
    /// A return statement.
    Return {
        return_span: Span<PPos>,
        expr: RExpr,
        semi: Option<Span<PPos>>
    },
    /// A semicolon
    SColon {
        semi: Span<PPos>
    },
    /// An if statement or chain of if statements.
    If {
        stmt: RIf,
    },
    /// A loop of one form or another.
    Loop {
        stmt: RLoop,
    },
    /// Assign a variable a value
    Assign {
        /// The `let` keyword span.
        let_: Span<PPos>,
        /// The span of the mutable keyword (if given)
        mutable: Option<Span<PPos>>,
        /// The variable name.
        ident: Span<PPos>,
        /// The optionally-specified type.
        ty: Option<RType>,
        /// The `=` sign.
        equal_sign: Span<PPos>,
        /// The expression after the `=`.
        equal_value: RExpr,
        /// The optional semicolon after the assignment statement (can only be
        /// omitted if on last line of group).
        semicolon: Option<Span<PPos>>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RReturnType {
    /// Returns the given type.
    Type(RType),
    /// Function never returns.
    Never,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RFn {
    pub span: Span<PPos>,
    pub fn_span: Span<PPos>,
    pub id: Span<PPos>,
    pub args: Vec<RParam>,
    pub ret_type: Option<RReturnType>,
    pub body: RBlock,
}

// Comments

/// 
/// A comment in Rust.
/// 
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RComment {
    Symex {
        symex: Span<PPos>,
        follow: Span<PPos>,
    },
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


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Whitespace {
    Comment(RComment),
    Blank(Span<PPos>),
}

// -- Literals --

///
/// A literal that cannot have a sign.
/// 
#[derive(Debug, PartialEq, Eq, Clone)]
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

impl RLit {
    pub fn span(&self) -> Span<PPos> {
        use RLit::*;
        match self {
            Char(s) => s.span.clone(),
            String(s) => s.span.clone(),
            RawString(s) => s.span.clone(),
            Byte(s) => s.span.clone(),
            ByteString(s) => s.span.clone(),
            RawByteString(s) => s.span.clone(),
            Integer(s) => s.span(),
            Float(s) => s.span.clone(),
            Bool(s) => s.span(),
        }
    }
}

impl <Store: ParseStore<PPos, char> + ?Sized> IntoLisp<Store, PPos> for RLit {
    fn into_lisp(&self, store: &Store) -> String {
        use RLit::*;
        match self {
            Char(c) => c.into_lisp(store),
            String(s) => s.into_lisp(store),
            RawString(s) => s.into_lisp(store),
            Byte(b) => b.into_lisp(store),
            ByteString(b) => b.into_lisp(store),
            RawByteString(b) => b.into_lisp(store),
            Integer(i) => i.into_lisp(store),
            Float(f) => f.into_lisp(store),
            Bool(b) => b.into_lisp(store),
        }
    }
}

// --- Crate ---

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RCrate {
    pub utf8bom: Option<Span<PPos>>,
    pub shebang: Option<Span<PPos>>,
    // pub attrs: RInnerAttributes,
    pub items: Vec<RItem>,
}

/// 
/// An item in a crate.
/// 
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RItem {
//    Mod {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: RMod,
//    },
//    ExternCrate {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: RExternCrate,
//    },
//    UseDecl {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: RUseDecl,
//    },
    Fn {
        span: Span<PPos>,
        vis: Option<RVis>,
        val: RFn,
    },
//    TypeAlias {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: RTypeAlias,
//    },
//    Struct {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: RStruct,
//    },
//    Enum {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: REnum,
//    },
//    Union {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: RUnion,
//    },
//    Const {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: RConstItem,
//    },
//    Static {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: RStaticItem,
//    },
//    Trait {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: RTrait,
//    },
//    Impl {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: RImpl,
//    },
//    ExternBlock {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: RExternBlock,
//    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
/// The visibility of an item.
pub enum RVis {
    VisPub   { span: Span<PPos>, },
    VisCrate { span: Span<PPos>, },
    VisSelf  { span: Span<PPos>, },
    VisSuper { span: Span<PPos>, },
//    VisPath  {
//        span: Span<PPos>,
//        path: RSimplePath,
//    }
}

// --- Literals ---

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RBoolLit {
    True  { span: Span<PPos> },
    False { span: Span<PPos> }
}

impl RBoolLit {
    pub fn span(&self) -> Span<PPos> {
        use RBoolLit::*;
        match self {
            True { span } => span.clone(),
            False { span } => span.clone(),
        }
    }
}

impl <Store: ParseStore<Pos, char> + ?Sized, Pos: ParsePos> IntoLisp<Store, Pos> for RBoolLit {
    fn into_lisp(&self, store: &Store) -> String {
        use RBoolLit::*;
        match self {
            True { .. } => format!("true"),
            False { .. } => format!("false"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RCharLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
    pub suffix: Option<Span<PPos>>,
}

impl <Store: ParseStore<PPos, char> + ?Sized> IntoLisp<Store, PPos> for RCharLit {
    fn into_lisp(&self, store: &Store) -> String {
        self.span.into_lisp(store)
    }
}

/// 
/// A string where escape sequences are allowed.
/// 
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RStrLit {
    pub span: Span<PPos>,
    pub text: Span<PPos>,
    pub suffix: Option<Span<PPos>>,
}

impl <Store: ParseStore<PPos, char> + ?Sized> IntoLisp<Store, PPos> for RStrLit {
    fn into_lisp(&self, store: &Store) -> String {
        format!("{}", self.span.into_lisp(store))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RByteStrLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
    pub suffix: Option<Span<PPos>>,
}

impl <Store: ParseStore<PPos, char> + ?Sized> IntoLisp<Store, PPos> for RByteStrLit {
    fn into_lisp(&self, store: &Store) -> String {
        format!("{}", self.span.into_lisp(store))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RRawByteStrLit {
    pub span: Span<PPos>,
    pub text: Span<PPos>,
    pub suffix: Option<Span<PPos>>,
}

impl <Store: ParseStore<PPos, char> + ?Sized> IntoLisp<Store, PPos> for RRawByteStrLit {
    fn into_lisp(&self, store: &Store) -> String {
        format!("{}", self.text.into_lisp(store))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RByteLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
    pub suffix: Option<Span<PPos>>
}

impl <Store: ParseStore<PPos, char> + ?Sized> IntoLisp<Store, PPos> for RByteLit {
    fn into_lisp(&self, store: &Store) -> String {
        format!("{}", self.value.into_lisp(store))
    }
}

/// 
/// A string where no escape sequences are allowed i.e. the string is
/// exactly as it looks.
/// 
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RRawStrLit {
    pub span: Span<PPos>,
    pub text: Span<PPos>,
    pub suffix: Option<Span<PPos>>,
}

impl <Store: ParseStore<PPos, char> + ?Sized> IntoLisp<Store, PPos> for RRawStrLit {
    fn into_lisp(&self, store: &Store) -> String {
        format!("{}", self.span.into_lisp(store))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RFloatLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
    pub exp: Option<Span<PPos>>,
    pub value_exp_span: Span<PPos>,
    pub suffix: Option<Span<PPos>>
}

impl <Store: ParseStore<PPos, char> + ?Sized> IntoLisp<Store, PPos> for RFloatLit {
    fn into_lisp(&self, store: &Store) -> String {
        format!("{}", self.span.into_lisp(store))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RSFloatLit {
    pub span: Span<PPos>,
    pub neg: bool,
    pub lit: RFloatLit,
}

impl <Store: ParseStore<PPos, char> + ?Sized> IntoLisp<Store, PPos> for RSFloatLit {
    fn into_lisp(&self, store: &Store) -> String {
        format!("{}", self.span.into_lisp(store))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RDecLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
}

impl <Store: ParseStore<PPos, char> + ?Sized> IntoLisp<Store, PPos> for RDecLit {
    fn into_lisp(&self, store: &Store) -> String {
        format!("{}", self.span.into_lisp(store))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RBinLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
}

impl <Store: ParseStore<PPos, char> + ?Sized> IntoLisp<Store, PPos> for RBinLit {
    fn into_lisp(&self, store: &Store) -> String {
        format!("{}", self.span.into_lisp(store))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ROctLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
}

impl <Store: ParseStore<PPos, char> + ?Sized> IntoLisp<Store, PPos> for ROctLit {
    fn into_lisp(&self, store: &Store) -> String {
        format!("{}", self.span.into_lisp(store))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RHexLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
}

impl <Store: ParseStore<PPos, char> + ?Sized> IntoLisp<Store, PPos> for RHexLit {
    fn into_lisp(&self, store: &Store) -> String {
        format!("{}", self.span.into_lisp(store))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl RIntLit {
    pub fn span(&self) -> Span<PPos> {
        use RIntLit::*;
        match self {
            DecLit { span, .. } => span.clone(),
            BinLit { span, .. } => span.clone(),
            OctLit { span, .. } => span.clone(),
            HexLit { span, .. } => span.clone(),
        }
    }
}

impl <Store: ParseStore<PPos, char> + ?Sized> IntoLisp<Store, PPos> for RIntLit {
    fn into_lisp(&self, store: &Store) -> String {
        match self {
            RIntLit::DecLit { lit, .. } => lit.into_lisp(store),
            RIntLit::BinLit { lit, .. } => lit.into_lisp(store),
            RIntLit::OctLit { lit, .. } => lit.into_lisp(store),
            RIntLit::HexLit { lit, .. } => lit.into_lisp(store),
        }
    }
}

/// 
/// Parses a file and returns the result.
/// 
pub fn parse_file(file_text: &str) -> ParseResult<RCrate, String, PPos> {
    //println!("Parsing: \"{}\"", file_text);

    // create `expr` (it requires a number of recursive child nodes)
    srule!(w, w_rule);
    srule!(alpha, alpha_rule);
    srule!(numeric, numeric_rule);
    srule!(ident, ident_rule);
    srule!(param, param_rule);
    srule!(params, params_rule);
    srule!(return_type, return_type_rule);
    srule!(expr, expr_rule);
    srule!(mul_or_div, mul_or_div_rule);
    srule!(power, power_rule);
    srule!(value, value_rule);
    srule!(block, block_rule);
    srule!(type_tuple, type_tuple_rule);
    srule!(ty, ty_rule);
    srule!(statement, statement_rule);
    srule!(func, func_rule);
    srule!(file, file_rule);
    srule!(item, item_rule);
    srule!(vis, vis_rule);

    srule!(literal_expression, literal_expression_rule);
    srule!(char_literal, char_literal_rule);
    srule!(string_literal, string_literal_rule);
    srule!(raw_string_literal, raw_string_literal_rule);
    srule!(byte_literal, byte_literal_rule);
    srule!(byte_string_literal, byte_string_literal_rule);
    srule!(raw_byte_string_literal, raw_byte_string_literal_rule);
    srule!(integer_literal, integer_literal_rule);
    srule!(float_literal, float_literal_rule);
    srule!(bool_literal, bool_literal_rule);
    srule!(dec_literal, dec_literal_rule);
    srule!(bin_literal, bin_literal_rule);
    srule!(oct_literal, oct_literal_rule);
    srule!(hex_literal, hex_literal_rule);

    srule!(quote_escape, quote_escape_rule);
    srule!(ascii_escape, ascii_escape_rule);
    srule!(unicode_escape, unicode_escape_rule);
    srule!(oct_digit, oct_digit_rule);
    srule!(hex_digit, hex_digit_rule);
    srule!(string_continue, string_continue_rule);
    srule!(suffix, suffix_rule);
    srule!(suffix_no_e, suffix_no_e_rule);
    srule!(raw_string_content, raw_string_content_rule);
    srule!(ascii_for_char, ascii_for_char_rule);
    srule!(byte_escape, byte_escape_rule);
    srule!(ascii_for_string, ascii_for_string_rule);
    srule!(raw_byte_string_content, raw_byte_string_content_rule);
    srule!(ascii, ascii_rule);
    srule!(dec_digit, dec_digit_rule);
    srule!(bin_digit, bin_digit_rule);
    srule!(float_exponent, float_exponent_rule);

    srule!(return_statement, return_statement_rule);
    srule!(let_statement, let_statement_rule);
    srule!(expr_semi_statement, expr_semi_statement_rule);
    srule!(semi_statement, semi_statement_rule);

    srule!(if_statement, if_statement_rule);
    srule!(loop_statement, loop_statement_rule);
    srule!(infinite_loop, infinite_loop_rule);
    srule!(while_loop, while_loop_rule);
    srule!(for_loop, for_loop_rule);

    srule!(logic_op, logic_op_rule);
    srule!(add_or_sub, add_or_sub_rule);
    srule!(assign, assign_rule);

    srule!(comment, comment_rule);
    srule!(line_comment, line_comment_rule);
    srule!(block_comment, block_comment_rule);
    srule!(inner_line_doc, inner_line_doc_rule);
    srule!(inner_block_doc, inner_block_doc_rule);
    srule!(outer_line_doc, outer_line_doc_rule);
    srule!(outer_block_doc, outer_block_doc_rule);
    srule!(block_comment_or_doc, block_comment_or_doc_rule);

    // define function to produce "panic" uniform messages of parse
    let panic = &|pos: Span<PPos>, fn_name: &str, message: &str| -> String {
        format!("{}: ({}) {}", pos, fn_name, message)
    };

    // IDENTIFIERS

    let isolated_cr = &SpanOf(('\r', Not('\n')));

    // unicode groups
    let xid_start = &MapPValue(|span, ch| {
        if UnicodeXID::is_xid_start(ch) {
            Okay(span.clone(), span.end)
        } else {
            Error(panic(span, "xid_start", "expected character in the [:XID_Start:] unicode group"))
        }
    });

//    let xid_continue = MapPValue(|span, ch| {
//        if UnicodeXID::is_xid_continue(ch) {
//            Okay(span.clone(), span.end)
//        } else {
//            Error(panic(span, "xid_continue", "expected character in the [:XID_Continue:] unicode group"))
//        }
//    });
//    let xid_continue = &xid_continue;

    // --- COMMENTS ---

    comment_rule.set(Funnel8(
        MapV(("//", OneOf2(Not(Funnel(['/', '!', '\n'])), "//"), "symex", SpanOf(ZeroOrMore((Not('\n'), AnyV())))), |(_, _, symex, follow)| RComment::Symex { symex, follow }),
        line_comment,
        block_comment,
        inner_line_doc,
        inner_block_doc,
        outer_line_doc,
        outer_block_doc,
        block_comment_or_doc,
    ));

    line_comment_rule.set(MapV(
        Spanned(OneOf2(
            ("//", OneOf2(Not(Funnel(['/', '!', '\n'])), "//"), SpanOf(ZeroOrMore((Not('\n'), AnyV())))),
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
                    Not(Funnel(['*', '!'])),
                    "**",
                    block_comment_or_doc
                ),
                ZeroOrMore(OneOf2(
                    block_comment_or_doc,
                    (Not("*/"), AnyV())
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
            SpanOf(ZeroOrMore((Not(OneOf2('\n', isolated_cr)), AnyV())))
        )),
        |(span, (_, text))| {
            RComment::InnerLineDoc { span, text }
        }
    ));

    inner_block_doc_rule.set(MapV(
        Spanned(Surround(
            "/*!",
            SpanOf(ZeroOrMore(OneOf2(block_comment_or_doc, (Not(OneOf2("*/", isolated_cr)), AnyV())))),
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
            SpanOf((OneOf2(Not('*'), block_comment_or_doc), ZeroOrMore(OneOf2(block_comment_or_doc, (Not(OneOf2("*/", isolated_cr)), AnyV()))))),
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

    suffix_rule.set(ident);

    suffix_no_e_rule.set(SpanOf((Not(Funnel(['e', 'E'])), suffix)));

    // --- whitespace ---

    // a rule that just consumes whitespace space
    w_rule.set(ZeroOrMore(Funnel2(
        MapV(SpanOf(OneOf2(..=32u32, 127u32)), |s| Whitespace::Blank(s)),
        MapV(comment, |c| Whitespace::Comment(c)),
    )));

    // a rule to parse an ascii letter (lower case or upper case)
    alpha_rule.set(SpanOf(OneOf2(97..=122, 65..=90)));
    // a rule to parse an ascii numeric (0,1,2,etc.)
    numeric_rule.set(SpanOf(48..=57));
    // a rule to parse an identifier
    ident_rule.set(SpanOf(
        (
            OneOf2(alpha.clone(), '_'),
            ZeroOrMore(OneOf3(alpha.clone(), numeric.clone(), '_'))
        )
    ));

    return_type_rule.set(Map(
        Leader("->", (w, OneOf2(ty, '!')), |_, arrow_span, _| panic(arrow_span, "return_type", "missing return type")),
        |res| {
            res.map_value(|(_arrow, (_, any_of_two))| {
                match any_of_two {
                    AnyOf2::Child1(ty) => RReturnType::Type(ty),
                    AnyOf2::Child2(_) => RReturnType::Never,
                }
            })
        }
    ));

    // --- Literals ---

    literal_expression_rule.set(
        Funnel9(
            MapV(char_literal,            |c| RLit::Char(c)),
            MapV(string_literal,          |s| RLit::String(s)),
            MapV(raw_string_literal,      |r| RLit::RawString(r)),
            MapV(byte_literal,            |b| RLit::Byte(b)),
            MapV(byte_string_literal,     |b| RLit::ByteString(b)),
            MapV(raw_byte_string_literal, |r| RLit::RawByteString(r)),
            MapV(integer_literal,         |i| RLit::Integer(i)),
            MapV(float_literal,           |f| RLit::Float(f)),
            MapV(bool_literal,            |b| RLit::Bool(b)),
        )
    );

    // - Char Literal -

    char_literal_rule.set(MapV(
        Spanned((
            '\'',
            SpanOf(OneOf4(
                (Not(Funnel(['\'', '\\', '\n', '\r', '\t'])), AnyV()),
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

    quote_escape_rule.set(SpanOf(Funnel(["\\'", "\\\""])));

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

    // - STRING LITERAL -

    string_literal_rule.set(MapV(Spanned((
            '"',
            SpanOf(ZeroOrMore(OneOf5(
                (Not(OneOf2('"', '\\')), AnyV()),
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
            ('"', SpanOf(ZeroOrMore((Not(isolated_cr), Not('"'), AnyV()))), '"'),
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

    // - BYTE LITERAL -

    byte_literal_rule.set(MapV(
        Spanned(('b', '0', SpanOf(OneOf2(ascii_for_char, byte_escape)), '\'', Maybe(suffix))),
        |(span, (_, _, value, _, suffix))| { RByteLit { span, value, suffix } }
    ));

    ascii_for_char_rule.set((Not(Funnel(['\\', '\n', '\r', '\t'])), 0x00..=0x7F));

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

    bin_digit_rule.set(Funnel(['0', '1']));

    oct_digit_rule.set(48..=55);

    dec_digit_rule.set(48..=57);

    hex_digit_rule.set(SpanOf(OneOf3(
        dec_digit,
        65..=70, // upper case
        97..=102 // lower case
    )));

    // - FLOAT LITERALS -

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
        Funnel(['e', 'E']),
        Maybe(Funnel(['+', '-'])),
        ZeroOrMore(OneOf2(dec_digit, '_')),
        dec_digit,
        ZeroOrMore(OneOf2(dec_digit, '_'))
    )));

    // - BOOLEAN LITERAL -

    bool_literal_rule.set(Funnel2(
        MapV("true" , |span| RBoolLit::True  { span }),
        MapV("false", |span| RBoolLit::False { span }),
    ));

    // --- Expressions ---

    use BinOp::*;
    use AssignOp::*;
    {
        use RExpr::{BinOp, AssignOp};

        expr_rule.set(assign);

        assign_rule.set(
            RLJoin(logic_op, (w, OneOf11('=', "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>="), w),
                |left: RExpr, (_, op, _), right| {
                    let span = Span::new(left.span().start, right.span().end);
                    use AnyOf11::*;
                    match op {
                        Child1(op_span) => AssignOp { span, left: Box::new(left), op: Assign, op_span, right: Box::new(right) },
                        Child2(op_span) => AssignOp { span, left: Box::new(left), op: AAdd, op_span, right: Box::new(right) },
                        Child3(op_span) => AssignOp { span, left: Box::new(left), op: ASub, op_span, right: Box::new(right) },
                        Child4(op_span) => AssignOp { span, left: Box::new(left), op: AMul, op_span, right: Box::new(right) },
                        Child5(op_span) => AssignOp { span, left: Box::new(left), op: ADiv, op_span, right: Box::new(right) },
                        Child6(op_span) => AssignOp { span, left: Box::new(left), op: AMod, op_span, right: Box::new(right) },
                        Child7(op_span) => AssignOp { span, left: Box::new(left), op: AAnd, op_span, right: Box::new(right) },
                        Child8(op_span) => AssignOp { span, left: Box::new(left), op: AOr, op_span, right: Box::new(right) },
                        Child9(op_span) => AssignOp { span, left: Box::new(left), op: AXOr, op_span, right: Box::new(right) },
                        Child10(op_span) => AssignOp { span, left: Box::new(left), op: ALSh, op_span, right: Box::new(right) },
                        Child11(op_span) => AssignOp { span, left: Box::new(left), op: ARSh, op_span, right: Box::new(right) },
                    }
                }
            )
        );

        logic_op_rule.set(
            LRJoin(add_or_sub, (w, OneOf8("&&", "||", "==", "!=", "<=", ">=", "<", ">"), w),
                |left: RExpr, (_, op, _), right| {
                    let span = Span::new(left.span().start, right.span().end);
                    use AnyOf8::*;
                    match op {
                        Child1(op_span) => BinOp { span, left: Box::new(left), op: And       , op_span, right: Box::new(right) },
                        Child2(op_span) => BinOp { span, left: Box::new(left), op: Or        , op_span, right: Box::new(right) },
                        Child3(op_span) => BinOp { span, left: Box::new(left), op: EqEq      , op_span, right: Box::new(right) },
                        Child4(op_span) => BinOp { span, left: Box::new(left), op: NotEq     , op_span, right: Box::new(right) },
                        Child5(op_span) => BinOp { span, left: Box::new(left), op: LessThanEq, op_span, right: Box::new(right) },
                        Child6(op_span) => BinOp { span, left: Box::new(left), op: MoreThanEq, op_span, right: Box::new(right) },
                        Child7(op_span) => BinOp { span, left: Box::new(left), op: LessThan  , op_span, right: Box::new(right) },
                        Child8(op_span) => BinOp { span, left: Box::new(left), op: MoreThan  , op_span, right: Box::new(right) },
                    }
                }
            )
        );

        add_or_sub_rule.set(
            LRJoin(mul_or_div, (w, OneOf2('+', '-'), w),
            |left: RExpr, (_, op, _), right| {
                let span = Span::new(left.span().start, right.span().end);
                match op {
                    AnyOf2::Child1(op_span) => BinOp { span, left: Box::new(left), op: Add, op_span, right: Box::new(right) },
                    AnyOf2::Child2(op_span) => BinOp { span, left: Box::new(left), op: Sub, op_span, right: Box::new(right) },
                }
            })
        );

        mul_or_div_rule.set(
            LRJoin(power, (w, OneOf3('*', '/', '%'), w),
            |left: RExpr, (_, op, _), right| {
                let span = Span::new(left.span().start, right.span().end);
                match op {
                    AnyOf3::Child1(op_span) => BinOp { span, left: Box::new(left), op: Mul, op_span, right: Box::new(right) },
                    AnyOf3::Child2(op_span) => BinOp { span, left: Box::new(left), op: Div, op_span, right: Box::new(right) },
                    AnyOf3::Child3(op_span) => BinOp { span, left: Box::new(left), op: Mod, op_span, right: Box::new(right) },
                }
            })
        );

        power_rule.set(
            LRJoin(value, (w, '^', w),
            |left: RExpr, (_, op_span, _), right| {
                let span = Span::new(left.span().start, right.span().end);
                BinOp { span, left: Box::new(left), op: BitXOr, op_span, right: Box::new(right) }
            })
        );

        value_rule.set(
            Funnel12(
                MapV(Spanned(('!', w, expr)), |(span, (not, _, expr))| RExpr::Not { span, not, expr: Box::new(expr) }),
                MapV(Spanned(('*', w, expr)), |(span, (star, _, expr))| RExpr::Deref { span, star, expr: Box::new(expr) }),
                MapV(Spanned(('&', w, "mut", w, expr)), |(span, (and, _, mutable, _, expr))| RExpr::BorrowMut { span, and, mutable, expr: Box::new(expr) }),
                MapV(Spanned(('&', w, expr)), |(span, (and, _, expr))| RExpr::Borrow { span, and, expr: Box::new(expr) }),
                MapV(Spanned(('-', w, expr)), |(span, (neg, _, expr))| RExpr::Negate { span, neg, expr: Box::new(expr) }),
                MapV(block, |group| RExpr::Block(group)),
                MapV(Spanned(('(', w, expr, w, ')')), |(span, (_, _, e, _, _))| RExpr::Group { span, expr: Box::new(e) }),
                MapV(literal_expression, |lit| RExpr::Lit(lit)),
                MapV(if_statement, |if_| RExpr::If(Box::new(if_))),
                MapV(loop_statement, |loop_| RExpr::Loop(Box::new(loop_))),
                MapV(Spanned((ident, '(', w, Join(expr, (w, ',', w)), Maybe((w, ',', w)), w, ')')), |(span, (ident, _, _, args, _, _, _))| RExpr::Call { span, ident, args }),
                MapV(ident, |span| RExpr::Var(span)),
            ),
        );
    }

    block_rule.set(
        MapV(
            Spanned(Surround(
                    '{',
                        (w, ZeroOrMore((statement, w)), Maybe((expr, w))),
                    '}',
                    |_, _, e| e,
                    |_, ocbrace_span, _, _| panic(ocbrace_span, "block", "openning curly brace is missing its complementary closing curly brace to end the scope"),
                )),
            |(span, (_lcbrace, (w, statements, expr), _rcbrace))| {
                RBlock {
                    span,
                    statements: {
                        let mut stmts: Vec<RStatement> = Vec::new();
                        for wh in w {
                            match wh {
                                Whitespace::Blank(_) => {},
                                Whitespace::Comment(comment) => stmts.push(RStatement::Comment { comment })
                            }
                        }
                        for (stmt, whitespace) in statements {
                            stmts.push(stmt);
                            for w in whitespace {
                                match w {
                                    Whitespace::Blank(_) => {},
                                    Whitespace::Comment(comment) => stmts.push(RStatement::Comment { comment })
                                }
                            }
                        }
                        if let Some(expr) = expr.map(|(e, _)| e) {
                            stmts.push(RStatement::Expr { expr, semi: None });
                        }
                        stmts
                    },
                }
            }
        )
    );

    type_tuple_rule.set(
        MapV(
            Spanned(Surround(
                '(',
                    (
                        w,
                        Maybe((ty, w, ZeroOrMore((',', w, ty, w))))
                    ),
                ')',
                |_, _, e| e,
                |_, oparen_span, _, _| panic(oparen_span, "type_tuple", "missing closing parenthesis after this open parenthesis"),
            )),
            |(span, (_, (_, maybe_types), _))| {
                match maybe_types {
                    Some((t1, _, types)) => {
                        let mut types: Vec<RType> = types.into_iter().map(|v|v.2).collect();
                        types.insert(0, t1);
                        RType::Tuple { span, types }
                    },
                    None => RType::Tuple { span, types: Vec::new() },
                }
            }
        )
    );

    ty_rule.set(
        MapV(
            Spanned(OneOf2(
                type_tuple,
                (
                    ident,
                    Maybe((
                        w,
                        Surround(
                            '<',
                            ( 
                                w,
                                ZeroOrMore((ty, w))
                            ),
                            '>',
                        |_, oarrow_span, _| panic(oarrow_span, "ty", "expected values within this type bounds"),
                            |_, oarrow_span, _, _| panic(oarrow_span, "ty", "expected closing arrow ('>') after this openning arrow ('<')"),
                        )
                    ))
                )
            )),
            |(span, any_of_two)| {
                match any_of_two {
                    AnyOf2::Child1(ty) => {
                        ty
                    },
                    AnyOf2::Child2((ident, args)) => {
                        RType::Template {
                            span,
                            name: ident,
                            args: match args {
                                Some((_, (_, (_, args), _))) => {
                                    args.into_iter().map(|v|v.0).collect()
                                },
                                None => Vec::new(),
                            },
                        }
                    }
                }
            }
        )
    );

    // --- STATEMENTS ---

    statement_rule.set(Funnel6(
            MapV(if_statement,   |stmt| RStatement::If { stmt }),
            MapV(loop_statement, |stmt| RStatement::Loop { stmt }),
            return_statement,
            let_statement,
            expr_semi_statement,
            semi_statement,
        ),
    );

    return_statement_rule.set(
        MapV(
            Leader(
                "return",
                (w, expr, Maybe((w, ';'))),
                |_, return_span, _| panic(return_span, "statement", "expected expression after this \"return\" keyword")
            ),
            |(return_span, (_, expr, maybe_semi))| {
                RStatement::Return {
                    return_span,
                    expr,
                    semi: match maybe_semi { Some(v) => Some(v.1), None => None }
                }
            }
        ),
    );

    let_statement_rule.set(
        MapV(
            Leader(
                "let",
                (w, Maybe(("mut", w)), ident, Maybe((w, ':', w, ty)), w, '=', w, expr, Maybe((w, ';'))),
                |_, let_span, _| panic(let_span, "statement", "expected_variable assignment after this let statement")
            ),
            |(let_span, (_, mutable, ident, maybe_type, _, eq_span, _, expr, maybe_semi))| {
                RStatement::Assign {
                    let_: let_span,
                    mutable: mutable.map(|(s, _)|s),
                    ident,
                    ty: match maybe_type {
                        Some((_, _colon_span, _, ty)) => Some(ty),
                        None => None,
                    },
                    equal_sign: eq_span,
                    equal_value: expr,
                    semicolon: match maybe_semi {
                        Some((_, span)) => Some(span),
                        None => None,
                    },
                }
            }
        ),
    );

    expr_semi_statement_rule.set(MapV(
        (expr, Maybe((w, ';'))),
        |(expr, maybe_semi)| {
            RStatement::Expr { expr, semi: maybe_semi.map(|(_, s)|s) }
        }
    ));

    semi_statement_rule.set(MapV(
        ";",
        |semi| RStatement::SColon { semi }
        ),
    );

//     Spanned(Leader(
//            "if", (w, expr, w, block,
//                Maybe((w, Leader("else", (w, OneOf2(block, if_statement)),
//                    |_, pos, _| panic(pos, "if_statement", "expected code block after this \"else\"")
//                )))
//            )

    if_statement_rule.set(MapV(Spanned((
            OJoin(
                MapV(
                    Leader("if", (w, expr, w, block), |_, pos, _| panic(pos, "if_statement", "expected expression and body of the if statement after this \"if\" keyword")),
                    |(_, (_, expr, _, block))| (expr, block) 
                ),
                (w, "else", w)
            ),
            Maybe(MapV((w, "else", w, block), |(_, _, _, block)| block))
        )),
        |(span, (ifs, else_stmt))| {
            RIf { span, ifs, else_stmt }
        }
    ));

    loop_statement_rule.set(Funnel3(
        infinite_loop,
        while_loop,
        for_loop,
    ));

    infinite_loop_rule.set(MapV(
        Spanned(("loop", w, block)),
        |(span, (_, _, block))| RLoop::Infinite { span, block }
    ));

    while_loop_rule.set(MapV(
        Spanned(("while", w, expr, w, block)),
        |(span, (_, _, expr, _, block))| RLoop::While { span, expr, block }
    ));

    for_loop_rule.set(MapV(
        Spanned(("for", w, ident, w, "in", w, expr, w, block)),
        |(span, (_, _, var, _, _, _, expr, _, block))| RLoop::For { span, var, expr, block }
    ));

    // --- Function ---

    // the rule to parse a function
    func_rule.set(
        MapV(
            Spanned(Leader(
                "fn",
                (w, Leader(ident,
                        (w,
                            Surround(
                                '(', (w, params, w), ')',
                                |_, oparen_span, _| panic(oparen_span, "func", "expected parameters in this function argument scope"),
                                |_, oparen_span, _, _| panic(oparen_span, "func", "expected closing parenthesis to match this open parenthesis")
                            ),
                            Maybe((w, return_type)),
                            Req((w, block), |_, p, _| panic(Span::new(p.clone(), p), "func", "function requires a function body"))
                        ),
                        |_, ident_span, _| panic(ident_span, "func", "expected function parameters and body after function identifier")
                    )
                ),
                |_, fn_span, _| panic(fn_span, "func", "expected correct function syntax after 'fn' keyword"))),
            |(span, (fn_span, (_, (id_span, (_, (_oparen, (_, params, _), _cparen), ret_type, (_, body))))))| {
                RFn {
                    span,
                    fn_span,
                    id: id_span,
                    args: params,
                    ret_type: ret_type.map(|(_, t)| t),
                    body,
                }
            }
        )
    );

    // a vector of function parameters
    params_rule.set(MapV(
        (Join(param, (w, ',', w)), Maybe((w, ','))),
        |(params, _)| params
    ));

    // function parameter
    param_rule.set(MapV(
        (
            Maybe(("mut", w)),
            Leader(
                ident,
                (w, Leader(
                        ':', (w, ty),
                        |_, colon_span, _| panic(colon_span, "param", "missing type after this colon"),
                    )
                ),
                |_, id_span, _| panic(id_span, "param", "missing arg's type"),
            )
        ),
        |(mutable, (id, (_, (_colon, (_, ty)))))| RParam { mutable: mutable.map(|(s, _)|s), id, ty, }
    ));

    vis_rule.set(
        Funnel4(
            MapV(SpanOf(("pub", w, '(', w, "crate", w, ')')), |span| RVis::VisCrate { span }),
            MapV(SpanOf(("pub", w, '(', w, "self" , w, ')')), |span| RVis::VisSelf  { span }),
            MapV(SpanOf(("pub", w, '(', w, "super", w, ')')), |span| RVis::VisSuper { span }),
            MapV("pub", |span| RVis::VisPub { span }),
            //MapV(("pub", w, '(', w, "in", w, simple_path, w, ')'), |(_, _, _, _, _, _, path, _, _)| RVis::VisPath { span, path })
        ),
    );

    item_rule.set(
        MapV(Spanned((Maybe(vis), w, func)), |(span, (vis, _, val))| RItem::Fn { span, vis, val })
    );

    // the rule to parse a `File`

    file_rule.set(MapV((
            w,
            Maybe(("\\uFEFF", w)),
            Maybe((SpanOf(("#!", OneOrMore((Not('\n'), AnyV())))), w)),
            //ZeroOrMore((inner_attribute, w)),
            ZeroOrMore((item, w)),
            Req(End(), |_, pos: PPos, _| panic(Span::new(pos.clone(), pos.clone()), "file", "parser failed to reach the end of the file (from this pos)"))
        ),
        |(_, utf8bom, shebang, items, _)| {
            RCrate {
                utf8bom: utf8bom.map(|(s, _)|s),
                shebang: shebang.map(|(s, _)|s),
                items: items.into_iter().map(|(i, _)|i).collect()
            }
        }
    ));

    file.parse(ParseContext::new(&AnyMemTable::new(file_text), PPos::new()))
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::parser::parser::{RCrate, RFn, Execute};

    use super::parse_file;
    use super::super::ParseResult;
    use ParseResult::*;
    use super::PPos;

    #[test]
    fn test_if_stmt() {
        match parse_file("
            fn a(x:i32, y:i32) -> u128 { 
                if x == 2 { }
            }
            ") {
            Okay(value, advance) => {
                println!("{:?} {:?}", value, advance)
            },
            Error(error) => panic!("Error: {}", error),
            Panic(error) => panic!("Panic: {}", error),
        }
    }

    #[test]
    fn test_stmt_semi_end_fn() {
        match parse_file("
            fn a(x:i32, y:i32) -> u128 { 
                let u = 6; 
                8 / 4; 
                5 - 2;
            }
            ") {
            Okay(value, advance) => {
                println!("{:?} {:?}", value, advance)
            },
            Error(error) => panic!("Error: {}", error),
            Panic(error) => panic!("Panic: {}", error),
        }
    }

    #[test]
    fn test_stmt_fn() {
        match parse_file(" fn hello() -> u8 {  (10 + 3) * 3 / 4 ^ (10); 10; 23; 0 } ") {
            Okay(value, advance) => {
                println!("{:?} {:?}", value, advance)
            },
            Error(error) => panic!("Error: {}", error),
            Panic(error) => panic!("Panic: {}", error),
        }
    }

    #[test]
    fn test_expr_fn() {
        match parse_file(" fn hello() -> u8 {  (10 + 3) * 3 / 4 ^ (10) } ") {
            Okay(value, advance) => {
                println!("{:?} {:?}", value, advance)
            },
            Error(error) => panic!("Error: {}", error),
            Panic(error) => panic!("Panic: {}", error),
        }
    }

    #[test]
    fn test_add_fn() {
        match parse_file(" fn hello() -> u8 {10 + 3}") {
            Okay(value, advance) => {
                println!("{:?} {:?}", value, advance)
            },
            Error(error) => panic!("Error: {}", error),
            Panic(error) => panic!("Panic: {}", error),
        }
    }

    #[test]
    fn test_empty_fn() {
        match parse_file(" fn hello() -> u8 {0}") {
            Okay(value, advance) => {
                println!("{:?} {:?}", value, advance)
            },
            Error(error) => panic!("Error: {}", error),
            Panic(error) => panic!("Panic: {}", error),
        }
    }

    #[test]
    fn test_empty() {
        match parse_file("") {
            Okay(value, advance) => {
                println!("{}: {:?}", advance, value);
            },
            Error(error) => panic!("Error: {}", error),
            Panic(error) => panic!("Panic: {}", error),
        }
    }

    #[test]
    fn full_test() {
        let test = "
fn main() {
    let mut x = 5;
    let mut y = 18;

    s_algebra(x, y);
}

fn s_algebra(mut x:i32, mut y:i32) -> i32 {
    x = y + 4;
    y = 2*x; 
    let mut w = (x*4) + y;
    //symex - what are the possible values?
    return w;
}

fn b_algebra(mut x:i32, mut y:i32) -> i32 {
    x = y + 4;
    y = 2*x; 
    let mut w = x / y;
    //symex - division by zero?
    return w;
}

fn b_ifStmt(mut x:i32, mut y:i32) -> i32 {
    x = y + 4;
    y = 2*x; 
    if x <= 4 {
        x = 4;
    }

    else if x > 4 {
        x = 2;
    }

    else {
        y = 0;
        //symex - is this reachable?
    }
    return y;
}
fn s_ifStmt(mut x:i32, mut y:i32) -> i32 {
    x = y + 4;
    y = 2*x; 
    if x < 4 {
        x = 4;
    }

    else if x > 4 {
        x = 2;
    }

    else {
        y = 0;
        //symex - is this reachable?
    }
    return y;
}

fn b2_ifStmt(mut x:i32, mut y:i32) -> i32 {
    if x < 5 {
        if x >= 5 {
            y = x;
        }
        y = y + 1;
        //symex - what values can y have?
    }
    return y;
}

fn s2_ifStmt(mut x:i32, mut y:i32) -> i32 {
    if x < 5 {
        if x > 5 {
            y = x;
        }
        x = y * 2;
        //symex
    }
    return x;
}

//fn s_loop(n: i64) -> i64 {
//    let mut i: i64 = 0;
//    let mut j: i64 = 1;
//    while i < n {
//        j = j * 2;
//        i = i + 1;
//    }
//    //symex - what is the value of i
//	return i;
//}
//
//fn b_loop(n: i64) -> i64 {
//    let mut i: i64 = 0;
//    let mut j: i64 = 1;
//    while i <= n {
//        j = j * 2;
//        i = i + 1;
//    }
//    //symex - what is the value of i
//	return i;
//}
//
//fn b_infLoop(n: i64) -> i64 {
//    let mut i = 0;
//    let mut j = 1;
//    while i < n {
//        j = j * 2;
//    }
//	return i;
//}
        ";

        match parse_file(test) {
            Okay(value, advance) => {
                //println!("{}: {:?}", advance, value);
                let mut engine = Vec::new();
                println!("{:?}", value);
                println!("{:?}", value.execute(test, &mut engine, HashSet::from([0])));
            },
            Error(error) => panic!("Error: {}", error),
            Panic(error) => panic!("Panic: {}", error),
        }
    }

    #[test]
    fn test_symex() {
        let s = "
fn test(mut y: i32) {
    y = 8;
    let w = 9;
    //symex
}
fn s_if(mut x:i32, mut y:i32) -> i32 {
    x = y * 2;
    if x == 6 {
        y = y + 3;
        if y > 2 {
            y = y + 2;
            //symex
        }
        
    }
    else {
        y = y + 4;
        
    }

    x = x / 2;

}
";
        match parse_file(s) {
            Okay(value, _) => {
                let mut engine = Vec::new();
                //println!("{:?}", value);
                //print!("{:?}", value.execute(s, &mut engine, 0));
                let result = value.execute(s, &mut engine, HashSet::from([0]));
                match result {
                    Ok(ok) => {
                        for res in ok.res.iter() {
                            println!("{}: {}", res.symex_pos, res.res);
                        }
                    },
                    Err(_) => panic!("Error!"),
                }
                //let mut i = 0;
                //while i < engine.len() {
                    //println!("{}", i);
                //    if engine[i].pi.satisfiable && engine[i].reached_symex {
                //      println!("{}", engine[i].to_string());
                //    }
                //   // = i + 1;
                //}
            },
            Error(error) => panic!("Error: {}", error),
            Panic(error) => panic!("Panic: {}", error),
        }
    }
}