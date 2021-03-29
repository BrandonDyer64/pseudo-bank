use rust_decimal::Decimal;

pub struct Account {
    available: Decimal,
    // held: Decimal,
    // is_locked: bool,
}

impl Account {
    pub fn new() -> Account {
        Account {
            available: 0.into(),
            // held: 0.into(),
            // is_locked: false,
        }
    }

    pub fn deposit(&mut self, amount: Decimal) {
        self.available += amount;
    }

    pub fn debit(&mut self, amount: Decimal) {
        let new_amount = self.available - amount;
        if new_amount > 0.into() {
            self.available = new_amount;
        }
    }
}
