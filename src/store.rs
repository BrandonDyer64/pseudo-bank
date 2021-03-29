use std::collections::HashMap;

use crate::{
    account::Account,
    client_id::ClientId,
    transaction::{Transaction, TransactionType},
};

#[derive(Debug)]
pub struct Store {
    accounts: HashMap<ClientId, Account>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            accounts: HashMap::new(),
        }
    }

    pub fn apply_transaction(&mut self, transaction: Transaction) {
        let account = self
            .accounts
            .entry(transaction.client)
            .or_insert_with(|| Account::new(transaction.client));

        match transaction.transaction_type {
            TransactionType::Deposit => {
                account.deposit(transaction.amount.unwrap_or(0.into()));
            }
            TransactionType::Withdraw => {
                account.withdraw(transaction.amount.unwrap_or(0.into()));
            }
            _ => unimplemented!(),
        }
    }

    pub fn get_account(&self, client: ClientId) -> Option<&Account> {
        self.accounts.get(&client)
    }

    pub fn get_accounts(&self) -> &HashMap<ClientId, Account> {
        &self.accounts
    }
}
