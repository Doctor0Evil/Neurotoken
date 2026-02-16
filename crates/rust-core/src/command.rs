use serde::{Deserialize, Serialize};
use crate::role::Role;
use crate::scope::Scope;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CommandKind {
    NeuralCachePurge,
    ContextExpand { amount: usize },
    SessionArchive,
    SessionSnapshot,
    SessionRecover,
    CodexValidate,
    CodexCompress,
    NodeRebalance,
    QueryTrace,
    QueryOptimize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub id: uuid::Uuid,
    pub kind: CommandKind,
    pub role: Role,
    pub scope: Scope,
}

impl Command {
    pub fn new(kind: CommandKind, role: Role, scope: Scope) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            kind,
            role,
            scope,
        }
    }
}
