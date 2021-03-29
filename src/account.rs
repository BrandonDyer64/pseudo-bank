use rust_decimal::Decimal;
use serde::{ser::SerializeStruct, Serialize, Serializer};

use crate::client_id::ClientId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    id: ClientId,
    available: Decimal,
    held: Decimal,
    is_locked: bool,
}

impl Account {
    pub fn new(id: ClientId) -> Account {
        Account {
            id,
            available: 0.into(),
            held: 0.into(),
            is_locked: false,
        }
    }

    pub fn deposit(&mut self, amount: Decimal) {
        self.available += amount;
    }

    pub fn withdraw(&mut self, amount: Decimal) {
        let new_amount = self.available - amount;
        if new_amount > 0.into() {
            self.available = new_amount;
        }
    }
}

impl Serialize for Account {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Account", 4)?;
        state.serialize_field("client", &self.id)?;
        state.serialize_field("available", &self.available)?;
        state.serialize_field("held", &self.held)?;
        state.serialize_field("total", &(self.available + self.held))?;
        state.serialize_field("locked", &self.is_locked)?;
        state.end()
    }
}
