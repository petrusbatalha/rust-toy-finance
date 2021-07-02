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
    let mut raw_record = csv::StringRecord::new();
    let headers = rdr.headers().unwrap().clone();

    while rdr.read_record(&mut raw_record).unwrap() {
        let record: Transactions = raw_record.deserialize(Some(&headers)).unwrap();
        println!("{:?}", record);
    }

}
