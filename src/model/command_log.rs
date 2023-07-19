use serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;
use chrono::{DateTime, Utc};

/// The command log table name.
pub const COMMAND_LOG_TABLE: &str = "command_log";

/// The command status.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommandStatus {
    /// The command is running.
    RUNNING,
    /// The command is finished with success.
    SUCCESS,
    /// The command is finished with error.
    ERROR,
    /// The command is skipped.
    SKIPPED,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommandLog {
    pub id: Option<Thing>,
    pub command_name: String,
    pub command_args: String,
    pub status: CommandStatus,
    pub message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub elapsed: Option<i64>
}