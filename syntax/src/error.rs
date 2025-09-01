use crate::token::{Span, TokenKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyntaxError {
    pub kind: SyntaxErrorKind,
    pub span: Span,
}

impl SyntaxError {
    pub const UNEXPECTED_EOI: SyntaxError = SyntaxError {
        kind: SyntaxErrorKind::UnexpectedEndOfInput,
        span: Span::EOI,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxErrorKind {
    InvalidLexicalToken,
    UnexpectedToken {
        expected: &'static [TokenKind],
        got: TokenKind,
    },
    UnexpectedEndOfInput,
}
