use std::io::{self, BufRead, Write};

use crate::error::Error;
use crate::executor::Executor;
use crate::parser;

pub const PROMPT: &str = "> ";
pub const CONTINUATION_PROMPT: &str = "... ";

pub fn split_statements(buffer: &str) -> (Vec<String>, String) {
    match buffer.rfind(';') {
        None => (Vec::new(), buffer.to_string()),
        Some(last) => {
            let statements = buffer[..last].split(';').map(|s| s.trim().to_string()).collect();
            (statements, buffer[last + 1..].to_string())
        }
    }
}

pub fn handle_statement(stmt: &str, executor: &mut Executor) -> Result<String, Error> {
    let cmd = parser::parse(stmt)?;
    Ok(executor.execute(cmd)?)
}

pub fn run<R: BufRead, W: Write>(mut input: R, mut output: W) -> io::Result<()> {
    let mut executor = Executor::new();
    let mut buffer = String::new();
    let mut line = String::new();

    loop {
        let prompt = if buffer.trim().is_empty() { PROMPT } else { CONTINUATION_PROMPT };
        write!(output, "{prompt}")?;
        output.flush()?;

        line.clear();
        if input.read_line(&mut line)? == 0 {
            break;
        }
        buffer.push_str(&line);

        let (statements, rest) = split_statements(&buffer);
        buffer = rest;

        for stmt in statements {
            match stmt.to_ascii_lowercase().as_str() {
                "exit" => return Ok(()),
                "help" => writeln!(output, "{}", help_text())?,
                _ => match handle_statement(&stmt, &mut executor) {
                    Ok(msg) => writeln!(output, "{msg}")?,
                    Err(e) => writeln!(output, "{e}")?,
                },
            }
        }
    }
    Ok(())
}

pub fn help_text() -> &'static str {
    "Commands:\n  \
     CREATE name;\n  \
     INSERT name {1, 2, 3};\n  \
     PRINT_TREE name;\n  \
     CONTAINS name {1, 2, 3};\n  \
     SEARCH name [WHERE (INTERSECTS | CONTAINS | CONTAINED_BY) {1, 2, 3}];\n  \
     help; exit;"
}
