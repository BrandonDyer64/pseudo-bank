use std::collections::HashMap;

use crate::model::{
    account::Account,
    id::{client_id::ClientId, transaction_id::TransactionId},
    transaction::Transaction,
};

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

    pub fn apply_transaction(&mut self, transaction: Transaction) {
        let account = self
            .accounts
            .entry(transaction.client)
            .or_insert_with(|| Account::new(transaction.client));

        let transaction_result = account.apply_transaction(&self.transactions, &transaction);

        match transaction_result {
            Ok(true) => {
                self.transactions
                    .insert((transaction.client, transaction.tx), transaction);
            }
            Err(err) => {
                eprintln!("\nError: {}\n{:?}", err, transaction);
            }
            _ => {}
        }
    }

    pub fn get_accounts(&self) -> &HashMap<ClientId, Account> {
        &self.accounts
    }

    pub fn get_transactions(&self) -> &HashMap<(ClientId, TransactionId), Transaction> {
        &self.transactions
    }
}
