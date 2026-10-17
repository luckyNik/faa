//! Grammar (keywords are case-insensitive, `;` is stripped by the CLI before parsing):
//!
//! ```text
//! command  := "CREATE" name
//!           | "INSERT" name set
//!           | "PRINT_TREE" name
//!           | "CONTAINS" name set
//!           | "SEARCH" name [ "WHERE" query ]
//! query    := ("INTERSECTS" | "CONTAINS" | "CONTAINED_BY") set
//! set      := "{" [ number { "," number } ] "}"
//! name     := identifier that is not a reserved word
//! ```

use std::collections::HashSet;

use crate::Value;
use crate::command::{Command, SearchQuery};
use crate::error::ParseErrorKind::TrailingTokens;
use crate::error::{ParseError, ParseErrorKind};
use crate::lexer::{Token, TokenKind, tokenize};

pub const RESERVED_WORDS: &[&str] = &[
    "CREATE", "INSERT", "PRINT_TREE", "CONTAINS", "SEARCH", "WHERE", "INTERSECTS", "CONTAINED_BY",
];

pub fn parse(input: &str) -> Result<Command, ParseError> {
    let tokens = tokenize(input)?;
    Parser::new(tokens).parse_command()
}

fn describe(token: &Token) -> String {
    match &token.kind {
        TokenKind::Ident(s) => format!("identifier '{s}'"),
        TokenKind::Number(n) => format!("number {n}"),
        TokenKind::LBrace => "'{'".to_string(),
        TokenKind::RBrace => "'}'".to_string(),
        TokenKind::Comma => format!("','")
    }
}

/// Recursive-descent parser over a token slice with a cursor.
pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, cursor: 0 }
    }

    /// Look at the current token without consuming it.
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }

    fn advance(&mut self) -> Option<Token> {
        let next_token = self.tokens.get(self.cursor).cloned();
        if next_token.is_some() { self.cursor += 1; }
        next_token
    }

    fn expect_ident(&mut self, expected: &str) -> Result<(String, usize), ParseError> {
        match self.advance() {
        None => Err(ParseError {
            pos: self.end_pos(),
            kind: ParseErrorKind::UnexpectedEnd { expected: expected.to_string() },
        }),
        Some(Token { kind: TokenKind::Ident(text), pos }) => Ok((text, pos)),
        Some(tok) => Err(ParseError {
            pos: tok.pos,
            kind: ParseErrorKind::UnexpectedToken {
                expected: expected.to_string(),
                found: describe(&tok),
            },
        }),
    }
    }

    fn end_pos(&self) -> usize {
        self.tokens.last().map(|t| t.pos).unwrap_or(0)
    }

    fn parse_name(&mut self) -> Result<String, ParseError> {
        let (candidate, pos) = self.expect_ident("collection name")?;
        if RESERVED_WORDS.iter().any(|w| w.eq_ignore_ascii_case(&candidate)) {
            return  Err (ParseError { pos, kind: ParseErrorKind::ReservedWord(candidate) });
        }
        Ok(candidate)
    }

    fn expect_token(&mut self, kind: TokenKind, expected: &str) -> Result<(), ParseError> {
        match self.advance() {
            Some(tok) if tok.kind == kind => Ok(()),
            Some(tok) => Err(ParseError {
                pos: tok.pos,
                kind: ParseErrorKind::UnexpectedToken {
                    expected: expected.to_string(),
                    found: describe(&tok),
                },
            }),
            None => Err(ParseError {
                pos: self.end_pos(),
                kind: ParseErrorKind::UnexpectedEnd { expected: expected.to_string() },
            }),
        }
    }

    fn expect_number(&mut self) -> Result<(Value, usize), ParseError> {
        match self.advance() {
            Some(Token { kind: TokenKind::Number(n), pos }) => Ok((n, pos)),
            Some(tok) => Err(ParseError {
                pos: tok.pos,
                kind: ParseErrorKind::UnexpectedToken {
                    expected: "number".to_string(),
                    found: describe(&tok),
                },
            }),
            None => Err(ParseError {
                pos: self.end_pos(),
                kind: ParseErrorKind::UnexpectedEnd { expected: "number".to_string() },
            }),
        }
    }

    fn parse_set(&mut self) -> Result<Vec<Value>, ParseError> {
        self.expect_token(TokenKind::LBrace, "'{'")?;

        if matches!(self.peek(), Some(Token { kind: TokenKind::RBrace, .. })) {
            self.advance();
            return Ok(Vec::new());
        }

        let mut seen = HashSet::new();
        let mut set = Vec::new();
        loop {
            let (n, pos) = self.expect_number()?;
            if !seen.insert(n) {
                return Err(ParseError { pos, kind: ParseErrorKind::DuplicateElement(n.to_string()) });
            }
            set.push(n);

            match self.advance() {
                Some(Token { kind: TokenKind::Comma, .. }) => {}
                Some(Token { kind: TokenKind::RBrace, .. }) => break,
                Some(tok) => {
                    return Err(ParseError {
                        pos: tok.pos,
                        kind: ParseErrorKind::UnexpectedToken {
                            expected: "',' or '}'".to_string(),
                            found: describe(&tok),
                        },
                    });
                }
                None => {
                    return Err(ParseError {
                        pos: self.end_pos(),
                        kind: ParseErrorKind::UnexpectedEnd { expected: "',' or '}'".to_string() },
                    });
                }
            }
        }
        Ok(set)
    }

    fn parse_search_query(&mut self) -> Result<SearchQuery, ParseError> {
        const EXPECTED: &str = "INTERSECTS, CONTAINS or CONTAINED_BY";
        let (word, pos) = self.expect_ident(EXPECTED)?;
        match word.to_ascii_uppercase().as_str() {
            "INTERSECTS" => Ok(SearchQuery::Intersects(self.parse_set()?)),
            "CONTAINS" => Ok(SearchQuery::Contains(self.parse_set()?)),
            "CONTAINED_BY" => Ok(SearchQuery::ContainedBy(self.parse_set()?)),
            _ => Err(ParseError {
                pos,
                kind: ParseErrorKind::UnexpectedToken {
                    expected: EXPECTED.to_string(),
                    found: format!("identifier '{word}'"),
                },
            }),
        }
    }

    pub fn parse_command(&mut self) -> Result<Command, ParseError> {
        if self.tokens.is_empty() {
            return Err(ParseError { pos: 0, kind: ParseErrorKind::EmptyStatement });
        }
        let (word, pos) = self.expect_ident("command")?;
        let command = match word.to_ascii_uppercase().as_str() {
            "CREATE" => Command::Create { name: self.parse_name()? },
            "INSERT" => {
                let collection = self.parse_name()?;
                let set = self.parse_set()?;
                Command::Insert { collection, set }
            }
            "PRINT_TREE" => Command::PrintTree { collection: self.parse_name()? },
            "CONTAINS" => {
                let collection = self.parse_name()?;
                let set = self.parse_set()?;
                Command::Contains { collection, set }
            }
            "SEARCH" => {
                let collection = self.parse_name()?;
                let has_where = matches!(
                    self.peek(),
                    Some(Token { kind: TokenKind::Ident(w), .. }) if w.eq_ignore_ascii_case("WHERE")
                );
                let filter = if has_where {
                    self.advance();
                    Some(self.parse_search_query()?)
                } else {
                    None
                };
                Command::Search { collection, filter }
            }
            _ => return Err(ParseError { pos, kind: ParseErrorKind::UnknownCommand(word) }),
        };
        self.expect_end()?;
        Ok(command)
    }

    fn expect_end(&self) -> Result<(), ParseError> {
        match self.peek() {
            Some(t) => {
                Err(ParseError { pos: t.pos, kind: TrailingTokens(describe(t)) })
            }
            None => Ok(())
        }
    }
}
