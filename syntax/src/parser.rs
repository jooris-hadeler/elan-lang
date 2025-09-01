use std::{iter::Peekable, num::IntErrorKind};

use crate::{
    ast,
    error::{SyntaxError, SyntaxErrorKind},
    token::{Token, TokenKind},
};

pub type ParserResult<T> = Result<T, SyntaxError>;

pub struct Parser<'src, I>
where
    I: Iterator<Item = Token<'src>>,
{
    tokens: Peekable<I>,
}

/// Rewrite this to use a Vec of tokens!!!!

impl<'src, I> Parser<'src, I>
where
    I: Iterator<Item = Token<'src>>,
{
    /// Constructs a new [Parser] from the given [Token]s.
    pub fn new(tokens: I) -> Self {
        let tokens = tokens.peekable();

        Self { tokens }
    }

    #[inline]
    /// Returns the next [Token] without consuming it.
    fn peek(&mut self) -> Option<Token<'src>> {
        self.tokens.peek().copied()
    }

    #[inline]
    /// Consumes and returns the next [Token].
    fn next(&mut self) -> Option<Token<'src>> {
        self.tokens.next()
    }

    #[inline]
    /// Checks if the peek [Token] is one of the given [TokenKind]s.
    fn is_peek(&mut self, kinds: &'static [TokenKind]) -> bool {
        self.peek().is_some_and(|tok| kinds.contains(&tok.kind))
    }

    /// Consumes and returns the next [Token] if it is of the given [TokenKind]s,
    /// otherwise returns a [SyntaxError].
    fn expect(&mut self, kinds: &'static [TokenKind]) -> ParserResult<Token<'src>> {
        match self.next() {
            Some(tok) if kinds.contains(&tok.kind) => Ok(tok),
            Some(tok) => Err(SyntaxError {
                kind: SyntaxErrorKind::UnexpectedToken {
                    expected: kinds,
                    got: tok.kind,
                },
                span: tok.span,
            }),
            None => Err(SyntaxError::UNEXPECTED_EOI),
        }
    }

    pub fn parse_expr_atom(&mut self) -> ParserResult<ast::Expr> {
        let peek_token = self.peek().ok_or(SyntaxError::UNEXPECTED_EOI)?;

        Ok(match peek_token.kind {
            TokenKind::Identifier => ast::Expr::Identifier(self.parse_identifier()?),
            TokenKind::Integer => ast::Expr::Integer(self.parse_integer_literal()?),
            TokenKind::Float => ast::Expr::Float(self.parse_float_literal()?),

            kind => {
                return Err(SyntaxError {
                    kind: SyntaxErrorKind::UnexpectedToken {
                        expected: &[TokenKind::Identifier, TokenKind::Integer, TokenKind::Float],
                        got: kind,
                    },
                    span: peek_token.span,
                });
            }
        })
    }

    fn parse_identifier(&mut self) -> ParserResult<ast::Identifier> {
        let ident_token = self.expect(&[TokenKind::Identifier])?;

        let text = ident_token.text.to_string();
        let span = ident_token.span;

        Ok(ast::Identifier { text, span })
    }

    fn parse_integer_literal(&mut self) -> ParserResult<ast::IntegerLiteral> {
        let integer_token = self.expect(&[TokenKind::Integer])?;

        let result = match integer_token.text {
            text if text.starts_with("0x") => u64::from_str_radix(&text[2..], 16),
            text if text.starts_with("0o") => u64::from_str_radix(&text[2..], 8),
            text if text.starts_with("0b") => u64::from_str_radix(&text[2..], 2),
            text => text.parse::<u64>(),
        };

        let value = match result {
            Ok(value) => value,
            Err(err) => match err.kind() {
                IntErrorKind::NegOverflow | IntErrorKind::PosOverflow => {
                    return Err(SyntaxError {
                        kind: SyntaxErrorKind::NumberOverflow,
                        span: integer_token.span,
                    });
                }
                _ => unreachable!(),
            },
        };

        let span = integer_token.span;

        Ok(ast::IntegerLiteral { value, span })
    }

    fn parse_float_literal(&mut self) -> ParserResult<ast::FloatLiteral> {
        let float_token = self.expect(&[TokenKind::Float])?;

        let value = match float_token.text.parse::<f64>() {
            Ok(value) => value,
            Err(_) => {
                return Err(SyntaxError {
                    kind: SyntaxErrorKind::InvalidNumber,
                    span: float_token.span,
                });
            }
        };

        let span = float_token.span;
        let value_bits = value.to_bits();

        Ok(ast::FloatLiteral { value_bits, span })
    }
}

#[cfg(test)]
mod test {
    use crate::{ast, error::SyntaxError, lexer::Lexer, parser::Parser, token::Span};

    #[test]
    fn expr_atom() -> Result<(), SyntaxError> {
        let test_case = [
            (
                "0x12",
                Ok(ast::Expr::Integer(ast::IntegerLiteral {
                    value: 0x12,
                    span: Span { start: 0, end: 4 },
                })),
            ),
            (
                "12.3e-5",
                Ok(ast::Expr::Float(ast::FloatLiteral {
                    value_bits: (12.3e-5f64).to_bits(),
                    span: Span { start: 0, end: 7 },
                })),
            ),
            (
                "cents",
                Ok(ast::Expr::Identifier(ast::Identifier {
                    text: "cents".to_string(),
                    span: Span { start: 0, end: 5 },
                })),
            ),
        ];

        for (input, output) in test_case {
            let tokens = Lexer::new(input).collect_tokens()?;
            let mut parser = Parser::new(tokens.into_iter());

            assert_eq!(parser.parse_expr_atom(), output);
        }

        Ok(())
    }
}
