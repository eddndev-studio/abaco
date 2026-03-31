use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::value_objects::AccountCode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: Uuid,
    pub date: NaiveDate,
    pub description: String,
    pub lines: Vec<JournalLine>,
    pub is_posted: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalLine {
    pub id: Uuid,
    pub account_code: AccountCode,
    pub debit: Decimal,
    pub credit: Decimal,
    pub description: Option<String>,
}

impl JournalEntry {
    pub fn new(date: NaiveDate, description: String, lines: Vec<JournalLine>) -> Self {
        Self {
            id: Uuid::new_v4(),
            date,
            description,
            lines,
            is_posted: false,
            created_at: Utc::now(),
        }
    }

    pub fn is_balanced(&self) -> bool {
        let total_debit: Decimal = self.lines.iter().map(|l| l.debit).sum();
        let total_credit: Decimal = self.lines.iter().map(|l| l.credit).sum();
        total_debit == total_credit
    }
}
