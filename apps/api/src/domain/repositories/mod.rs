use uuid::Uuid;

use crate::domain::entities::{Account, JournalEntry};

/// Port: driven adapter interface for account persistence.
pub trait AccountRepository: Send + Sync {
    fn find_by_id(&self, id: Uuid) -> impl std::future::Future<Output = anyhow::Result<Option<Account>>> + Send;
    fn find_all(&self) -> impl std::future::Future<Output = anyhow::Result<Vec<Account>>> + Send;
    fn save(&self, account: &Account) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
    fn delete(&self, id: Uuid) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}

/// Port: driven adapter interface for journal entry persistence.
pub trait JournalEntryRepository: Send + Sync {
    fn find_by_id(&self, id: Uuid) -> impl std::future::Future<Output = anyhow::Result<Option<JournalEntry>>> + Send;
    fn find_all(&self) -> impl std::future::Future<Output = anyhow::Result<Vec<JournalEntry>>> + Send;
    fn save(&self, entry: &JournalEntry) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}
