use crate::token::Span;

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Identifier(Identifier),
    Integer(IntegerLiteral),
    Float(FloatLiteral),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Identifier {
    pub text: String,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IntegerLiteral {
    pub value: u64,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FloatLiteral {
    /// The bit representation of the f64.
    pub value_bits: u64,
    pub span: Span,
}
