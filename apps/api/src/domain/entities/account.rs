use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::value_objects::{AccountCode, AccountType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub code: AccountCode,
    pub name: String,
    pub account_type: AccountType,
    pub parent_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Account {
    pub fn new(code: AccountCode, name: String, account_type: AccountType, parent_id: Option<Uuid>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            code,
            name,
            account_type,
            parent_id,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}
