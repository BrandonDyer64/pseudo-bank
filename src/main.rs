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
