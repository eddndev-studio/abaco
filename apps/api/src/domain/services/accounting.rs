use crate::domain::entities::JournalEntry;

pub struct AccountingService;

impl AccountingService {
    pub fn validate_entry(entry: &JournalEntry) -> Result<(), &'static str> {
        if entry.lines.is_empty() {
            return Err("Journal entry must have at least one line");
        }

        if entry.lines.len() < 2 {
            return Err("Journal entry must have at least two lines (double-entry)");
        }

        if !entry.is_balanced() {
            return Err("Journal entry debits and credits must balance");
        }

        Ok(())
    }
}
