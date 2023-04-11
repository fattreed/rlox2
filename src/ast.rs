use crate::token::{Token, Literal};

pub trait Visitor<T> {
    fn visit_expr(&mut self, e: Expr);
}

pub enum Expr {
    Binary(Box<Expr>, Box<Token>, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(Box<Token>, Box<Expr>),
}

struct Interpreter;

impl<T> Visitor<T> for Interpreter {
    fn visit_expr(&mut self, e: Expr) {
        match e {
            Expr::Binary(ref lhs, ref op, ref rhs) => (),
            Expr::Grouping(ref expr) => (),
            Expr::Literal(ref lit) => (),
            Expr::Unary(ref op, ref expr) => (),
        }
    }
}

