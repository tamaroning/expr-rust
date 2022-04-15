mod typeck;

use crate::lexer::Ident;
use crate::ty::Type;

#[derive(Debug)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub enum Item {
    Func(Func),
}

#[derive(Debug)]
pub struct Func {
    pub name: Ident,
    pub args: Vec<String>,
    pub ret_ty: Type,
    pub body: Box<Expr>,
    pub ty: Type,
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub ty: Type,
}

impl Expr {
    pub fn new(kind: ExprKind) -> Self {
        Expr {
            kind,
            ty: Type::Unresolved,
        }
    }
}

#[derive(Debug)]
pub enum ExprKind {
    LitExpr(LitExpr),
    IdentExpr(Ident),
    BinaryOpExpr(BinaryOpExpr),
    UnaryOpExpr(UnaryOpExpr),
}

#[derive(Debug)]
pub enum LitExpr {
    Num(i32),
}

#[derive(Debug)]
pub enum UnaryOpExpr {
    Neg(Box<Expr>),
}

#[derive(Debug)]
pub enum BinaryOpExpr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}
