use crate::token::{Token, TokenType, Literal};
use crate::ast::{Expr};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    #[must_use] const fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
        }
    }

    fn expression(&self) -> Expr {
        self.equality()
    }

    fn equality(&self) -> Expr {
        let mut expr = self.comparison();

        while self.match_op(vec![TokenType::BANG_EQ, TokenType::EQ_EQ]) {
            let op = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), Box::new(op), Box::new(right));
        }

        expr
    }

    fn comparison(&self) -> Expr {
        let mut expr = self.term();

        while self.match_op(vec![TokenType::GT, TokenType::GT_EQ, TokenType::LT, TokenType::LT_EQ]) {
            let op = self.previous();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), Box::new(op), Box::new(right));
        }

        expr
    }

    fn term(&self) -> Expr {
        let mut expr = self.factor();

        while self.match_op(vec![TokenType::MINUS, TokenType::PLUS]) {
            let op = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), Box::new(op), Box::new(right));
        }
 
        expr
    }

    fn factor(&self) -> Expr {
        let mut expr = self.unary();

        while self.match_op(vec![TokenType::SLASH, TokenType::STAR]) {
            let op = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), Box::new(op), Box::new(right));
        }

        expr
    }
    
    fn unary(&self) -> Expr {
        if self.match_op(vec![TokenType::BANG, TokenType::MINUS]) {
            let op = self.previous();
            let right = self.unary();
            return Expr::Unary(Box::new(op), Box::new(right));
        }

        self.primary()
    }

    fn primary(&self) -> Expr {
        if self.match_op(vec![TokenType::FALSE]) { return Expr::Literal(Literal::Bool(false)); }
        if self.match_op(vec![TokenType::TRUE]) { return Expr::Literal(Literal::Bool(true)); }
        if self.match_op(vec![TokenType::NIL]) { return Expr::Literal(Literal::Nil); }

        if self.match_op(vec![TokenType::NUMBER, TokenType::STRING]) { return Expr::Literal(self.previous().literal); }

        if self.match_op(vec![TokenType::LPAREN]) {
            let expr = self.expression();
            self.consume(TokenType::RPAREN, "Expected ')' after expression");
            return Expr::Grouping(Box::new(expr));
        }
        self.expression()
    }

    fn match_op(&mut self, token_types: Vec<TokenType>) -> bool {
        for t in token_types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current-1]
    }
}
