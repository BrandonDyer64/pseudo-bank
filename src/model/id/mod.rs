pub mod client_id;
pub mod transaction_id;

use std::fmt;

macro_rules! impl_display {
    ($name:path) => {
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

impl_display!(client_id::ClientId);
impl_display!(transaction_id::TransactionId);
