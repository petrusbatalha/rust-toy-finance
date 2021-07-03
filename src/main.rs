#![feature(in_band_lifetimes)]
#![feature(num_as_ne_bytes)]

mod sled_adapter;
mod traits;
mod transaction_service;
mod types;

use crate::sled_adapter::SledAdapter;
use crate::transaction_service::TransactionService;
use crate::types::{Action, Transaction};
use csv::Trim;
use std::io;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let mut rdr = csv::ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(io::stdin());

    let mut raw_record = csv::StringRecord::new();
    let headers = rdr.headers().unwrap().clone();

    let (tx, rx) = mpsc::channel(256);

    let db_adapter = SledAdapter::new();
    let transaction_service = TransactionService { db_adapter };

    tokio::spawn(async move {
        transaction_service.receive(rx).await;
    });

    while rdr.read_record(&mut raw_record).unwrap() {
        let record: Transaction = raw_record.deserialize(Some(&headers)).unwrap();
        match tx.send(Action::NewTransaction(record)).await {
            Ok(_) => {}
            Err(err) => {
                println!("Failed to send message {}", err.to_string());
            }
        };
    }
    tx.send(Action::DisplayTransaction).await.ok();
}
