use std::{iter::Peekable, str::Chars};

use crate::{
    error::{SyntaxError, SyntaxErrorKind},
    token::{Span, Token, TokenKind},
};

pub type LexerResult<'t> = Option<Result<Token<'t>, SyntaxError>>;

#[derive(Debug)]
pub struct Lexer<'src> {
    iter: Peekable<Chars<'src>>,
    text: &'src str,
    pos: usize,
    byte_pos: usize,
}

impl<'src> Lexer<'src> {
    /// Creates a new [Lexer] from the given source text.
    pub fn new(text: &'src str) -> Self {
        let iter = text.chars().peekable();
        let pos = 0;
        let byte_pos = 0;

        Self {
            iter,
            text,
            pos,
            byte_pos,
        }
    }

    #[inline]
    /// Returns the next [char] in the source text without advancing.
    fn peek(&mut self) -> Option<char> {
        self.iter.peek().copied()
    }

    #[inline]
    /// Consumes the next [char] in the source text.
    fn next(&mut self) {
        let Some(ch) = self.iter.next() else {
            return;
        };

        self.pos += 1;
        self.byte_pos += ch.len_utf8();
    }

    #[inline]
    /// Returns whether the peek [char] is a given char.
    fn is_peek(&mut self, ch: char) -> bool {
        self.peek().is_some_and(|peek| peek == ch)
    }

    #[inline]
    /// Returns the next [char] if it is a given char.
    fn try_next(&mut self, ch: char) -> bool {
        if self.is_peek(ch) {
            self.next();
            true
        } else {
            false
        }
    }

    /// Skips whitespace [char]s.
    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(char::is_whitespace) {
            self.next();
        }
    }

    #[inline]
    /// Creates a one-character [Token] with given [TokenKind].
    fn create_simple_token(&mut self, kind: TokenKind) -> Token<'src> {
        let start = self.pos;
        let byte_start = self.byte_pos;

        self.next();
        self.create_token(start, byte_start, kind)
    }

    #[inline]
    /// Creates a [Token] with given [TokenKind] and position.
    fn create_token(&self, start: usize, byte_start: usize, kind: TokenKind) -> Token<'src> {
        let span = Span {
            start,
            end: self.pos,
        };
        let text = &self.text[byte_start..self.byte_pos];

        Token { kind, span, text }
    }

    /// Used to lex the next [TokenKind::Identifier] [Token].
    fn next_identifier_token(&mut self) -> LexerResult<'src> {
        let start = self.pos;
        let byte_start = self.byte_pos;

        while self
            .peek()
            .is_some_and(|ch| matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'))
        {
            self.next();
        }

        Some(Ok(self.create_token(
            start,
            byte_start,
            TokenKind::Identifier,
        )))
    }

    /// Used to lex the next [TokenKind::Integer] or [TokenKind::Float] [Token].
    fn next_number_token(&mut self) -> LexerResult<'src> {
        const HEX_CHARS: fn(char) -> bool = |ch| matches!(ch, 'a'..='f' | 'A'..='F' | '0'..='9');
        const BIN_CHARS: fn(char) -> bool = |ch| matches!(ch, '0' | '1');
        const OCT_CHARS: fn(char) -> bool = |ch| matches!(ch, '0'..='7');

        let start = self.pos;
        let byte_start = self.byte_pos;

        if self.try_next('0') {
            if self.try_next('x') {
                return self.next_integer_token(start, byte_start, HEX_CHARS);
            } else if self.try_next('b') {
                return self.next_integer_token(start, byte_start, BIN_CHARS);
            } else if self.try_next('o') {
                return self.next_integer_token(start, byte_start, OCT_CHARS);
            }
        }

        while self.peek().is_some_and(|ch| matches!(ch, '0'..='9')) {
            self.next();
        }

        if self.try_next('.') {
            return self.next_float_token(start, byte_start);
        }

        Some(Ok(self.create_token(start, byte_start, TokenKind::Integer)))
    }

    /// Used to lex the next [TokenKind::Integer] [Token].
    fn next_integer_token(
        &mut self,
        start: usize,
        byte_start: usize,
        valid_chars: fn(char) -> bool,
    ) -> LexerResult<'src> {
        while self.peek().is_some_and(valid_chars) {
            self.next();
        }

        // if the number is only the base prefix throw an error
        if self.pos - start <= 2 {
            return Some(Err(SyntaxError {
                kind: SyntaxErrorKind::InvalidNumber,
                span: Span {
                    start,
                    end: self.pos,
                },
            }));
        }

        Some(Ok(self.create_token(start, byte_start, TokenKind::Integer)))
    }

    /// Used to lex the next [TokenKind::Float] [Token].
    fn next_float_token(&mut self, start: usize, byte_start: usize) -> LexerResult<'src> {
        let after_dot_start = self.pos;
        while self.peek().is_some_and(|ch| matches!(ch, '0'..='9')) {
            self.next();
        }

        // if we haven't had a digit after the `.` throw an error.
        if self.pos - after_dot_start == 0 {
            return Some(Err(SyntaxError {
                kind: SyntaxErrorKind::InvalidNumber,
                span: Span {
                    start,
                    end: self.pos,
                },
            }));
        }

        if self.try_next('e') {
            self.try_next('-');

            let after_e_start = self.pos;
            while self.peek().is_some_and(|ch| matches!(ch, '0'..='9')) {
                self.next();
            }

            // if we haven't found a digit after the `e` throw an error.
            if self.pos - after_e_start == 0 {
                return Some(Err(SyntaxError {
                    kind: SyntaxErrorKind::InvalidNumber,
                    span: Span {
                        start,
                        end: self.pos,
                    },
                }));
            }
        }

        Some(Ok(self.create_token(start, byte_start, TokenKind::Float)))
    }

    /// Used to lex the next [Token].
    pub fn next_token(&mut self) -> LexerResult<'src> {
        self.skip_whitespace();

        let start = self.pos;
        let byte_start = self.byte_pos;

        let Some(ch) = self.peek() else {
            return None;
        };

        Some(Ok(match ch {
            'a'..='z' | 'A'..='Z' | '_' => return self.next_identifier_token(),
            '0'..='9' => return self.next_number_token(),

            '+' => self.create_simple_token(TokenKind::Plus),
            '-' => self.create_simple_token(TokenKind::Minus),
            '*' => self.create_simple_token(TokenKind::Asterisk),
            '/' => self.create_simple_token(TokenKind::Slash),
            '%' => self.create_simple_token(TokenKind::Percent),

            '=' => {
                self.next();

                if self.try_next('=') {
                    self.create_token(start, byte_start, TokenKind::Equal)
                } else {
                    self.create_token(start, byte_start, TokenKind::Assign)
                }
            }

            '!' => {
                self.next();

                if self.try_next('=') {
                    self.create_token(start, byte_start, TokenKind::Unequal)
                } else {
                    self.create_token(start, byte_start, TokenKind::Bang)
                }
            }

            '<' => {
                self.next();

                if self.try_next('=') {
                    self.create_token(start, byte_start, TokenKind::LessEqual)
                } else {
                    self.create_token(start, byte_start, TokenKind::LessThan)
                }
            }

            '>' => {
                self.next();

                if self.try_next('=') {
                    self.create_token(start, byte_start, TokenKind::GreaterEqual)
                } else {
                    self.create_token(start, byte_start, TokenKind::GreaterThan)
                }
            }

            '.' => self.create_simple_token(TokenKind::Dot),
            '(' => self.create_simple_token(TokenKind::LParen),
            ')' => self.create_simple_token(TokenKind::RParen),

            _ => {
                self.next();

                let span = Span {
                    start,
                    end: self.pos,
                };

                return Some(Err(SyntaxError {
                    kind: SyntaxErrorKind::InvalidLexicalToken,
                    span,
                }));
            }
        }))
    }

    /// Collects the lexed [Token]s into a [Vec] unless a [SyntaxError] occurs.
    pub fn collect_tokens(mut self) -> Result<Vec<Token<'src>>, SyntaxError> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token() {
            tokens.push(token?);
        }

        Ok(tokens)
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Result<Token<'src>, SyntaxError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        error::{SyntaxError, SyntaxErrorKind},
        lexer::Lexer,
        token::{Span, Token, TokenKind::*},
    };

    #[test]
    fn all_tokens() -> Result<(), SyntaxError> {
        let input = "hello 12 0xAFFE 0b1001 0o777 1.003 1.23e12 1.67e-3 + - * / % = ! < > == != <= >= . ( )";
        let expected = [
            Token {
                kind: Identifier,
                span: Span { start: 0, end: 5 },
                text: "hello",
            },
            Token {
                kind: Integer,
                span: Span { start: 6, end: 8 },
                text: "12",
            },
            Token {
                kind: Integer,
                span: Span { start: 9, end: 15 },
                text: "0xAFFE",
            },
            Token {
                kind: Integer,
                span: Span { start: 16, end: 22 },
                text: "0b1001",
            },
            Token {
                kind: Integer,
                span: Span { start: 23, end: 28 },
                text: "0o777",
            },
            Token {
                kind: Float,
                span: Span { start: 29, end: 34 },
                text: "1.003",
            },
            Token {
                kind: Float,
                span: Span { start: 35, end: 42 },
                text: "1.23e12",
            },
            Token {
                kind: Float,
                span: Span { start: 43, end: 50 },
                text: "1.67e-3",
            },
            Token {
                kind: Plus,
                span: Span { start: 51, end: 52 },
                text: "+",
            },
            Token {
                kind: Minus,
                span: Span { start: 53, end: 54 },
                text: "-",
            },
            Token {
                kind: Asterisk,
                span: Span { start: 55, end: 56 },
                text: "*",
            },
            Token {
                kind: Slash,
                span: Span { start: 57, end: 58 },
                text: "/",
            },
            Token {
                kind: Percent,
                span: Span { start: 59, end: 60 },
                text: "%",
            },
            Token {
                kind: Assign,
                span: Span { start: 61, end: 62 },
                text: "=",
            },
            Token {
                kind: Bang,
                span: Span { start: 63, end: 64 },
                text: "!",
            },
            Token {
                kind: LessThan,
                span: Span { start: 65, end: 66 },
                text: "<",
            },
            Token {
                kind: GreaterThan,
                span: Span { start: 67, end: 68 },
                text: ">",
            },
            Token {
                kind: Equal,
                span: Span { start: 69, end: 71 },
                text: "==",
            },
            Token {
                kind: Unequal,
                span: Span { start: 72, end: 74 },
                text: "!=",
            },
            Token {
                kind: LessEqual,
                span: Span { start: 75, end: 77 },
                text: "<=",
            },
            Token {
                kind: GreaterEqual,
                span: Span { start: 78, end: 80 },
                text: ">=",
            },
            Token {
                kind: Dot,
                span: Span { start: 81, end: 82 },
                text: ".",
            },
            Token {
                kind: LParen,
                span: Span { start: 83, end: 84 },
                text: "(",
            },
            Token {
                kind: RParen,
                span: Span { start: 85, end: 86 },
                text: ")",
            },
        ];

        let tokens = Lexer::new(input).collect_tokens()?;
        assert_eq!(tokens.as_slice(), expected.as_slice());

        Ok(())
    }

    #[test]
    fn error() {
        let input = "@";
        let expected = Some(Err(SyntaxError {
            kind: SyntaxErrorKind::InvalidLexicalToken,
            span: Span { start: 0, end: 1 },
        }));

        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), expected);
    }

    #[test]
    fn invalid_numbers() {
        let test_cases = [
            (
                "0x",
                Err(SyntaxError {
                    kind: SyntaxErrorKind::InvalidNumber,
                    span: Span { start: 0, end: 2 },
                }),
            ),
            (
                "0b",
                Err(SyntaxError {
                    kind: SyntaxErrorKind::InvalidNumber,
                    span: Span { start: 0, end: 2 },
                }),
            ),
            (
                "0o",
                Err(SyntaxError {
                    kind: SyntaxErrorKind::InvalidNumber,
                    span: Span { start: 0, end: 2 },
                }),
            ),
            (
                "1.e-3",
                Err(SyntaxError {
                    kind: SyntaxErrorKind::InvalidNumber,
                    span: Span { start: 0, end: 2 },
                }),
            ),
            (
                "1.3e-",
                Err(SyntaxError {
                    kind: SyntaxErrorKind::InvalidNumber,
                    span: Span { start: 0, end: 5 },
                }),
            ),
        ];

        for (input, output) in test_cases {
            let lexer = Lexer::new(input);
            assert_eq!(lexer.collect_tokens(), output);
        }
    }
}
