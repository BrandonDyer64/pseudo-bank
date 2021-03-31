//! Useful datatypes for storing identifiers
//!
//! These provide a little extra type safety when working with ids.
//! It should aid the programmer when using these ids as they will always know
//! what type of id it is instead of something ambiguous like `u64`.

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
