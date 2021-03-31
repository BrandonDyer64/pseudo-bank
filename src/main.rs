//! # PseudoBank
//! A binary executable to handle a spreadsheet of transactions
//!
//! ## Documentation
//!
//! The best way to view the documentation for this program is to use `cargo doc`
//!
//! An online version of the documentation can be found [here](https://brandondyer64.github.io/pseudo-bank/pseudo_bank/index.html).
//!
//! ## Running
//! ```bash
//! cargo run input-file.csv
//! ```
//!
//! This will process the input file and output the final account balances
//!
//! Errors are output to standard error output.
//! This application should run quickly when there aren't too many transactions that it
//! needs to print due to error.
//!
//! ### Example Input
//! ```csv,no_run
//! type,       client, tx, amount
//! deposit,    1,      1,  1.0
//! deposit,    2,      2,  2.0
//! deposit,    1,      3,  2.0
//! withdraw,   1,      4,  1.5
//! withdraw,   2,      5,  3.0
//! dispute,    1,      1,
//! resolve,    1,      1,
//! dispute,    1,      1,
//! chargeback, 1,      1,
//! ```
//!
//! ### Example Output
//! ```csv,no_run
//! client,available,held,total,locked
//! 1,0.5,0.0000,0.5,true
//! 2,2,0.0000,2,false
//! ```

use std::env;

use csv::{ReaderBuilder, WriterBuilder};
use spreadsheet::{read_spreadsheet, write_spreadsheet};
use store::Store;

pub mod model;
pub mod spreadsheet;
pub mod store;

/// Processes a given csv and outputs the final account data to std::io
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut reader = ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(args[1].to_owned())
        .unwrap();
    let mut store = Store::new();
    read_spreadsheet(&mut store, &mut reader);
    let mut writer = WriterBuilder::new().from_writer(std::io::stdout());
    write_spreadsheet(&mut store, &mut writer);
}
