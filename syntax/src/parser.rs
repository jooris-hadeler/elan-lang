use std::iter::Peekable;

use crate::{
    ast,
    error::{SyntaxError, SyntaxErrorKind},
    lexer::Lexer,
    token::{Token, TokenKind},
};

pub type ParserResult<T> = Result<T, SyntaxError>;

pub struct Parser<'src> {
    lexer: Peekable<Lexer<'src>>,
}

/// Rewrite this to use a Vec of tokens!!!!

impl<'src> Parser<'src> {
    /// Constructs a new [Parser] from the given [Lexer].
    pub fn new(lexer: Lexer<'src>) -> Self {
        let lexer = lexer.peekable();

        Self { lexer }
    }

    #[inline]
    /// Returns the next [Token] without consuming it.
    fn peek(&mut self) -> ParserResult<Option<Token<'src>>> {
        self.lexer
            .peek()
            .copied()
            .map(|res| res.map(Some))
            .unwrap_or(Ok(None))
    }

    #[inline]
    /// Consumes and returns the next [Token].
    fn next(&mut self) -> ParserResult<Option<Token<'src>>> {
        self.lexer
            .next()
            .map(|res| res.map(Some))
            .unwrap_or(Ok(None))
    }

    #[inline]
    /// Checks if the peek [Token] is one of the given [TokenKind]s.
    fn is_peek(&mut self, kinds: &'static [TokenKind]) -> bool {
        self.peek()
            .is_ok_and(|opt| opt.is_some_and(|tok| kinds.contains(&tok.kind)))
    }

    /// Consumes and returns the next [Token] if it is of the given [TokenKind]s,
    /// otherwise returns a [SyntaxError].
    fn expect(&mut self, kinds: &'static [TokenKind]) -> ParserResult<Token<'src>> {
        match self.next()? {
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
        let peek_token = self.peek()?.ok_or(SyntaxError::UNEXPECTED_EOI)?;

        match peek_token.kind {
            TokenKind::Identifier => todo!(),
            TokenKind::Integer => todo!(),
            TokenKind::Float => todo!(),

            kind => Err(SyntaxError {
                kind: SyntaxErrorKind::UnexpectedToken {
                    expected: &[TokenKind::Identifier, TokenKind::Integer, TokenKind::Float],
                    got: kind,
                },
                span: peek_token.span,
            }),
        }
    }
}
