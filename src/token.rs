#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub lexeme: String,
    pub token_type: TokenType,
    pub literal: Literal,
    pub line: usize,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    LPAREN, RPAREN, LBRACE, RBRACE, COMMA, DOT, MINUS, PLUS, SEMICOLON, STAR, SLASH, COLON,

    BANG, BANG_EQ, EQ, EQ_EQ, LT, LT_EQ, GT, GT_EQ, QUESTION,

    STRING, NUMBER, 

    IDENTIFIER,

    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
    None
}
