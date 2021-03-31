//! Enum for transactions. Deposit, Withdraw, Dispute...

use serde::Deserialize;

/// These values will be parsed from lowercase values
///
/// `"deposit" → TransactionType::Deposit`
#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdraw,
    Dispute,
    Resolve,
    Chargeback,
}
