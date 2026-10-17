use crate::command::Command;
use crate::error::ExecError;
use crate::storage::Database;

#[derive(Default)]
pub struct Executor {
    pub db: Database,
}

impl Executor {
    pub fn new() -> Self {
        Self::default()
    }

    /// Deadline 1 stub: return a human readable echo of the command, e.g. `Parsed: Create { name: "grades" }`.
    /// Later: match on `cmd` and call the real database operations.
    pub fn execute(&mut self, cmd: Command) -> Result<String, ExecError> {
        Ok(format!("Parsed: {cmd:?}"))
    }
}
