use crate::token::Span;

#[derive(Debug, PartialEq, Eq)]
pub struct SyntaxError {
    pub kind: SyntaxErrorKind,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SyntaxErrorKind {
    InvalidLexicalToken,
}
