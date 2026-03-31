use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountCode(String);

impl AccountCode {
    pub fn new(code: impl Into<String>) -> Result<Self, &'static str> {
        let code = code.into();
        if code.is_empty() {
            return Err("Account code cannot be empty");
        }
        Ok(Self(code))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AccountCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
