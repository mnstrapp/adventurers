use serde::{Deserialize, Serialize};

/// Represents an unverified raw token as received from the client
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawToken {
    /// The raw token string
    pub value: String,
}
