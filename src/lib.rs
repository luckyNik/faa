// Pipeline: text --(cli)--> statements --(lexer)--> tokens --(parser)--> Command --(executor)--> output

pub mod cli;
pub mod command;
pub mod error;
pub mod executor;
pub mod lexer;
pub mod parser;
pub mod storage;

pub type Value = i64;
