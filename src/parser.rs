use crate::token::{Token, TokenType, Literal};
use crate::ast::{Expr};
use std::{fmt, error::Error};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    #[must_use] const fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    fn parse(&mut self) -> Option<Expr> {
        let expr = self.expression();
        match expr {
            Ok(e) => Some(e),
            Err(_) => None,
        }
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_op(vec![TokenType::BANG_EQ, TokenType::EQ_EQ]) {
            let op = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), Box::new(op), Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.match_op(vec![TokenType::GT, TokenType::GT_EQ, TokenType::LT, TokenType::LT_EQ]) {
            let op = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), Box::new(op), Box::new(right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_op(vec![TokenType::MINUS, TokenType::PLUS]) {
            let op = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), Box::new(op), Box::new(right));
        }
 
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_op(vec![TokenType::SLASH, TokenType::STAR]) {
            let op = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), Box::new(op), Box::new(right));
        }

        Ok(expr)
    }
    
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_op(vec![TokenType::BANG, TokenType::MINUS]) {
            let op = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(Box::new(op), Box::new(right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_op(vec![TokenType::FALSE]) { return Ok(Expr::Literal(Literal::Bool(false))); }
        if self.match_op(vec![TokenType::TRUE]) { return Ok(Expr::Literal(Literal::Bool(true))); }
        if self.match_op(vec![TokenType::NIL]) { return Ok(Expr::Literal(Literal::Nil)); }

        if self.match_op(vec![TokenType::NUMBER, TokenType::STRING]) { return Ok(Expr::Literal(self.previous().literal)); }

        if self.match_op(vec![TokenType::LPAREN]) {
            let expr = self.expression()?;
            self.consume(TokenType::RPAREN, "Expected ')' after expression")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        Err(ParseError{ token: self.peek(), message: "Expected expression".to_string() })
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParseError> {
        if self.check(token_type) { return Ok(self.advance()); }

        Err(ParseError{ token: self.peek(), message: message.to_string() })
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SEMICOLON { return; }

            match self.peek().token_type {
                TokenType::CLASS | TokenType::FOR | TokenType::FUN | 
                    TokenType::IF | TokenType::PRINT | TokenType::RETURN | 
                    TokenType::VAR | TokenType::WHILE => return,
                _ => (),
            }
            self.advance();
        }
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
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current-1].clone()
    }
}

#[derive(Debug)]
struct ParseError {
    token: Token,
    message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.token.token_type == TokenType::EOF {
            write!(f, "{} at end. ", self.message)
        } else {
            write!(f, "{} at line {}", self.message, self.token.line)
        }
    }
}

impl Error for ParseError {}
