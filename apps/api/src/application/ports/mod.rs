use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::value_objects::AccountType;

#[derive(Debug, Deserialize)]
pub struct CreateAccountInput {
    pub code: String,
    pub name: String,
    pub account_type: AccountType,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct CreateJournalEntryInput {
    pub date: NaiveDate,
    pub description: String,
    pub lines: Vec<JournalLineInput>,
}

#[derive(Debug, Deserialize)]
pub struct JournalLineInput {
    pub account_code: String,
    pub debit: Decimal,
    pub credit: Decimal,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BalanceOutput {
    pub account_code: String,
    pub account_name: String,
    pub debit: Decimal,
    pub credit: Decimal,
    pub balance: Decimal,
}
