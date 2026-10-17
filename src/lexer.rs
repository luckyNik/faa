use crate::Value;
use crate::error::{ParseError, ParseErrorKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    /// Keyword or collection name: `[A-Za-z_][A-Za-z0-9_]*`.
    Ident(String),
    /// Integer literal with optional leading `-`.
    Number(Value),
    LBrace,
    RBrace,
    Comma,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    /// Byte offset of the first character of the token in the input.
    pub pos: usize,
}

/// Turns one statement without the trailing `;` into tokens.
pub fn tokenize(input: &str) -> Result<Vec<Token>, ParseError> {
    let mut chars = input.char_indices().peekable();
    let mut tokens = Vec::new();
    while let Some(&(pos, c)) = chars.peek() {
        match c {
            c if c.is_whitespace() => {
                chars.next();
            }
            '{' => {
                tokens.push(Token { kind: TokenKind::LBrace, pos });
                chars.next();
            }
            '}' => {
                tokens.push(Token { kind: TokenKind::RBrace, pos });
                chars.next();
            }
            ',' => {
                tokens.push(Token { kind: TokenKind::Comma, pos });
                chars.next();
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut end = pos;
                while let Some(&(p, ch)) = chars.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        end = p + 1;
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token { kind: TokenKind::Ident(input[pos..end].to_string()), pos });
            }
            c if c.is_ascii_digit() || c == '-' => {
                let mut end = pos + 1;
                chars.next();
                while let Some(&(p, ch)) = chars.peek() {
                    if ch.is_ascii_digit() {
                        end = p + 1;
                        chars.next();
                    } else {
                        break;
                    }
                }
                let text = &input[pos..end];
                match text.parse::<Value>() {
                    Ok(n) => tokens.push(Token { kind: TokenKind::Number(n), pos }),
                    Err(_) => {
                        return Err(ParseError {
                            pos,
                            kind: ParseErrorKind::InvalidNumber(text.to_string()),
                        });
                    }
                }
            }
            other => {
                return Err(ParseError { pos, kind: ParseErrorKind::UnexpectedChar(other) });
            }
        }
    }
    Ok(tokens)
}
