use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Admin,
    Operator,
    Observer,
}

impl Role {
    pub fn can_execute_admin_ops(&self) -> bool {
        matches!(self, Role::Admin)
    }
}
