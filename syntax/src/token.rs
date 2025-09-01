use std::usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token<'src> {
    pub kind: TokenKind,
    pub span: Span,
    pub text: &'src str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Identifier,
    Integer,
    Float,

    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,

    Assign,
    Bang,
    Equal,
    Unequal,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,

    Dot,
    LParen,
    RParen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub const EOI: Span = Span {
        start: usize::MAX,
        end: usize::MAX,
    };
}
