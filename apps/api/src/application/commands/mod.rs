use uuid::Uuid;

use crate::domain::entities::{Account, JournalEntry, JournalLine};
use crate::domain::services::AccountingService;
use crate::domain::value_objects::AccountCode;

use super::ports::{CreateAccountInput, CreateJournalEntryInput};

pub fn create_account(input: CreateAccountInput) -> anyhow::Result<Account> {
    let code = AccountCode::new(input.code).map_err(|e| anyhow::anyhow!(e))?;
    Ok(Account::new(code, input.name, input.account_type, input.parent_id))
}

pub fn create_journal_entry(input: CreateJournalEntryInput) -> anyhow::Result<JournalEntry> {
    let lines: Vec<JournalLine> = input
        .lines
        .into_iter()
        .map(|l| {
            Ok(JournalLine {
                id: Uuid::new_v4(),
                account_code: AccountCode::new(l.account_code).map_err(|e| anyhow::anyhow!(e))?,
                debit: l.debit,
                credit: l.credit,
                description: l.description,
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let entry = JournalEntry::new(input.date, input.description, lines);
    AccountingService::validate_entry(&entry).map_err(|e| anyhow::anyhow!(e))?;
    Ok(entry)
}
