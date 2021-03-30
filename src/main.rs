use std::env;

use csv::{ReaderBuilder, WriterBuilder};
use spreadsheet::{read_spreadsheet, write_spreadsheet};
use store::Store;

pub mod model;
pub mod spreadsheet;
pub mod store;

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
