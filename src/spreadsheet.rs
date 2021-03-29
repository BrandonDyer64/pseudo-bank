use crate::store::Store;

pub fn read_spreadsheet<T: std::io::Read>(store: &mut Store, reader: &mut csv::Reader<T>) {
    reader
        .deserialize()
        .filter_map(|transaction| transaction.ok())
        .for_each(|transaction| store.apply_transaction(transaction));
}

pub fn write_spreadsheet<T: std::io::Write>(store: &mut Store, writer: &mut csv::Writer<T>) {
    store.get_accounts().iter().for_each(|account| {
        writer.serialize(account.1).unwrap();
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use csv::{ReaderBuilder, Writer};

    #[test]
    fn basic_spreadsheet() {
        let data = "\
type,     client, tx, amount
deposit,  1,      1,  1.0
deposit,  2,      2,  2.0
deposit,  1,      3,  2.0
withdraw, 1,      4,  1.5
withdraw, 2,      5,  3.0
";
        let mut store = Store::new();
        let mut reader = ReaderBuilder::new()
            .trim(csv::Trim::All)
            .from_reader(data.as_bytes());
        read_spreadsheet(&mut store, &mut reader);
        let mut writer = Writer::from_writer(vec![]);
        write_spreadsheet(&mut store, &mut writer);
        let data = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        println!("{}", data);
    }
}
