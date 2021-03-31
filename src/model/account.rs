//! Holds a client's account information, such as current balance

use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{ser::SerializeStruct, Serialize, Serializer};

use super::{
    error::transaction_error::TransactionError,
    id::{client_id::ClientId, transaction_id::TransactionId},
    transaction::Transaction,
    transaction_type::TransactionType,
};

/// Holds the data needed for a client's account
///
/// Instead of having a separate field for held funds, this model has only a total balance and a list of current disputes.
/// The available balance is determined by subtracting the disputed transactions from the current total balance.
/// This provides a layer of safety over just using calculations on a couple of numeric fields, namely that a dispute
/// could be resolved multiple times, or a transaction disputed multiple times.
///
/// With this method it _should_ be impossible to accidentally dispute twice or resolve twice.
///
/// **Note:** There is currently no protection against deposits or withdrawals with the same transaction id.
/// This would need to be implemented in the [Store](crate::store::Store).
///
/// ## Performance Considerations
///
/// Calculating the available balance is fast as long as there aren't many disputes.
/// This should be relatively rare, so the safety guarantees should be ultimately worth it.
#[derive(Debug)]
pub struct Account {
    id: ClientId,
    balance: Decimal,
    is_locked: bool,
    disputes: Vec<Transaction>,
}

impl Account {
    pub fn new(id: ClientId) -> Account {
        Account {
            id,
            balance: 0.into(),
            is_locked: false,
            disputes: Vec::new(),
        }
    }

    /// `total - disputed`
    pub fn get_available(&self) -> Decimal {
        self.balance - self.get_held()
    }

    /// Sum of all disputed transactions
    pub fn get_held(&self) -> Decimal {
        self.disputes
            .iter()
            .fold(0.into(), |acc, t| acc + t.amount.unwrap_or(0.into()))
    }

    /// Alters the current balances using a given transaction.
    ///
    /// This is where the bulk of the processing in the application is done.
    /// A clients account can be credited, depited, disputed, and locked through this method.
    ///
    /// ```
    /// # let client_id = ClientId(0);
    /// # let transaction_id = TransactionId(0);
    /// # let mut account = Account::new(ClientId(1234));
    /// # let transaction_store: HashMap<TransactionId, Transaction> = HashMap::new();
    /// let transaction = Transaction {
    ///     transaction_type: TransactionType::Deposit,
    ///     client: client_id,
    ///     tx: transaction_id,
    ///     amount: Some(10_000.into()),
    /// };
    ///
    /// account.apply_transaction(transaction_store, transaction);
    ///
    /// assert_eq!(account.get_available(), Decimal::from(10_000));
    /// ```
    pub fn apply_transaction(
        &mut self,
        transaction_store: &HashMap<TransactionId, Transaction>,
        transaction: Transaction,
    ) -> Result<Option<Transaction>, TransactionError> {
        if self.is_locked {
            return Err(TransactionError::AccountLocked(self.id));
        }
        match transaction.transaction_type {
            TransactionType::Deposit => {
                self.balance += transaction.amount.unwrap_or(0.into());
                Ok(Some(transaction))
            }
            TransactionType::Withdraw => {
                let amount = transaction.amount.unwrap_or(0.into());
                let available = self.get_available();
                if available - amount >= 0.into() {
                    self.balance -= amount;
                } else {
                    return Err(TransactionError::Overdraft {
                        client: self.id,
                        available,
                        transaction_amount: amount,
                    });
                }
                Ok(Some(transaction))
            }
            TransactionType::Dispute => {
                if let Some(disputed_transaction) = transaction_store.get(&transaction.tx) {
                    if !self.disputes.iter().any(|t| t.tx == transaction.tx) {
                        self.disputes.push(disputed_transaction.clone());
                    }
                }
                Ok(None)
            }
            TransactionType::Resolve => {
                self.disputes = self
                    .disputes
                    .drain(..)
                    .filter(|t| t.tx != transaction.tx)
                    .collect::<Vec<_>>();
                Ok(None)
            }
            TransactionType::Chargeback => {
                if let Some(disputed_transaction_amount) = self
                    .disputes
                    .iter()
                    .find(|t| t.tx == transaction.tx)
                    .and_then(|t| t.amount)
                {
                    self.is_locked = true;
                    self.disputes = self
                        .disputes
                        .drain(..)
                        .filter(|t| t.tx != transaction.tx)
                        .collect::<Vec<_>>();
                    self.balance -= disputed_transaction_amount;
                } else {
                    return Err(TransactionError::TransactionNotDisputed(
                        transaction.tx,
                        self.id,
                    ));
                }
                Ok(None)
            }
        }
    }
}

impl Serialize for Account {
    /// Serializes the account data into a human readable set of fields.
    ///
    /// This does not represent the underlying data in the account.
    /// It is only intended for outputing a summary.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Account", 4)?;
        state.serialize_field("client", &self.id)?;
        state.serialize_field("available", &self.get_available().round_dp(4))?;
        state.serialize_field("held", &self.get_held().round_dp(4))?;
        state.serialize_field("total", &self.balance.round_dp(4))?;
        state.serialize_field("locked", &self.is_locked)?;
        state.end()
    }
}
