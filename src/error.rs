use std::fmt;

/// Error produced while turning text into a `Command` (lexing or parsing).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub pos: usize,
    pub kind: ParseErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorKind {
    UnexpectedChar(char),
    InvalidNumber(String),
    UnexpectedToken { expected: String, found: String },
    UnexpectedEnd { expected: String },
    UnknownCommand(String),
    ReservedWord(String),
    DuplicateElement(String),
    TrailingTokens(String),
    EmptyStatement,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParseErrorKind::*;
        match &self.kind {
            UnexpectedChar(c) => write!(f, "unexpected character '{c}' at position {}", self.pos),
            InvalidNumber(s) => write!(f, "invalid number '{s}' at position {}", self.pos),
            UnexpectedToken { expected, found } => {
                write!(f, "expected {expected}, found {found} at position {}", self.pos)
            }
            UnexpectedEnd { expected } => write!(f, "unexpected end of command, expected {expected}"),
            UnknownCommand(c) => write!(f, "unknown command '{c}'"),
            ReservedWord(w) => write!(f, "'{w}' is a reserved word and cannot be a collection name"),
            DuplicateElement(v) => write!(f, "duplicate element {v} in set"),
            TrailingTokens(t) => write!(f, "unexpected '{t}' after end of command at position {}", self.pos),
            EmptyStatement => write!(f, "empty command"),
        }
    }
}

impl std::error::Error for ParseError {}

/// Error produced while executing a valid `Command` (needed from the 2nd deadline on).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecError {
    CollectionAlreadyExists(String),
    CollectionNotFound(String),
}

impl fmt::Display for ExecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecError::CollectionAlreadyExists(n) => write!(f, "collection '{n}' already exists"),
            ExecError::CollectionNotFound(n) => write!(f, "collection '{n}' does not exist"),
        }
    }
}

impl std::error::Error for ExecError {}

/// Anything that can go wrong while handling one statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Parse(ParseError),
    Exec(ExecError),
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::Parse(e)
    }
}

impl From<ExecError> for Error {
    fn from(e: ExecError) -> Self {
        Error::Exec(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parse(e) => write!(f, "Syntax error: {e}"),
            Error::Exec(e) => write!(f, "Error: {e}"),
        }
    }
}

impl std::error::Error for Error {}
