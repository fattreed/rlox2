use crate::token::{Token, Literal};

pub trait Visitor<T> {
    fn visit_expr(&mut self, e: &Expr) -> T;
}

pub enum Expr {
    Binary(Box<Expr>, Box<Token>, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(Box<Token>, Box<Expr>),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
}

