#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub lexeme: String,
    pub token_type: TokenType,
    pub literal: Literal,
    pub line: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    LPAREN, RPAREN, LBRACE, RBRACE, COMMA, DOT, MINUS, PLUS, SEMICOLON, STAR,

    EOF
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    None
}
