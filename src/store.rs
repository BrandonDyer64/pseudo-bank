//! The "Bank"

use std::collections::HashMap;

use crate::model::{
    account::Account,
    error::transaction_error::TransactionError,
    id::{client_id::ClientId, transaction_id::TransactionId},
    transaction::Transaction,
};

/// The core "bank" of the program
///
/// Stores the map of accounts and transactions.
/// Handles transactions applied to accounts.
///
/// For more on how transactions are handled, see [Account].
#[derive(Debug)]
pub struct Store {
    accounts: HashMap<ClientId, Account>,
    transactions: HashMap<(ClientId, TransactionId), Transaction>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            accounts: HashMap::new(),
            transactions: HashMap::new(),
        }
    }

    /// A passthrough for [Account]'s [apply_transaction](Account::apply_transaction) method.
    ///
    /// Creates a new account if one doesn't exist.
    /// Saves the transaction to the hashmap depending on the output of the account's
    /// [apply_transaction](Account::apply_transaction)
    pub fn apply_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<(), (Transaction, TransactionError)> {
        let account = self
            .accounts
            .entry(transaction.client)
            .or_insert_with(|| Account::new(transaction.client));

        let transaction_result = account.apply_transaction(&self.transactions, &transaction);

        match transaction_result {
            Ok(true) => {
                self.transactions
                    .insert((transaction.client, transaction.tx), transaction);
                Ok(())
            }
            Err(err) => Err((transaction, err)),
            _ => Ok(()),
        }
    }

    pub fn get_accounts(&self) -> &HashMap<ClientId, Account> {
        &self.accounts
    }

    pub fn get_transactions(&self) -> &HashMap<(ClientId, TransactionId), Transaction> {
        &self.transactions
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{
        id::{client_id::ClientId, transaction_id::TransactionId},
        transaction::Transaction,
        transaction_type::TransactionType,
    };

    use super::Store;

    #[test]
    fn creates_accounts_and_transactions() {
        let mut store = Store::new();
        let mut deposit = Transaction {
            transaction_type: TransactionType::Deposit,
            client: ClientId(0),
            tx: TransactionId(1),
            amount: Some(10.into()),
        };

        assert_eq!(store.get_accounts().len(), 0);
        assert_eq!(store.get_transactions().len(), 0);

        deposit.client = ClientId(1);
        assert!(store.apply_transaction(deposit.clone()).is_ok());
        deposit.client = ClientId(2);
        assert!(store.apply_transaction(deposit.clone()).is_ok());
        deposit.client = ClientId(3);
        assert!(store.apply_transaction(deposit.clone()).is_ok());

        assert_eq!(store.get_accounts().len(), 3);
        assert_eq!(store.get_transactions().len(), 3);

        deposit.transaction_type = TransactionType::Dispute;
        assert!(store.apply_transaction(deposit.clone()).is_ok());
        assert_eq!(store.get_accounts().len(), 3);
        assert_eq!(store.get_transactions().len(), 3);
    }
}
