mod types;
use std::io::Cursor;
use tokio::fs::File;
use tokio::io::AsyncRead;
use std::io;
use crate::types::Transactions;
use csv::Trim;

#[tokio::main]
async fn main() {
    let mut rdr = csv::ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(io::stdin());

    for result in rdr.deserialize() {
        let record: Transactions = result.unwrap();
        println!("{:?}", record);
    }

}
