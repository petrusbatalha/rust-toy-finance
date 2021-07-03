#![feature(in_band_lifetimes)]

mod types;
mod transaction_service;
mod traits;
mod map_adapter;

use std::io;
use csv::Trim;
use tokio::sync::mpsc;
use crate::types::Transaction;
use crate::transaction_service::TransactionService;
use crate::map_adapter::MapAdapter;
use crate::traits::TransactionDB;
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::mpsc::error::SendError;

#[tokio::main]
async fn main() {
    let mut rdr = csv::ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(io::stdin());
    let mut raw_record = csv::StringRecord::new();
    let headers = rdr.headers().unwrap().clone();

    let (tx, rx) = mpsc::channel(256);
    let db_adapter = MapAdapter::new();
    let transaction_service = TransactionService {
        db_adapter
    };

    tokio::spawn(async move {
        transaction_service.receive(rx).await;
    });

    while rdr.read_record(&mut raw_record).unwrap() {
            let record: Transaction = raw_record.deserialize(Some(&headers)).unwrap();
            match tx.send(record).await {
                Ok(_) => {},
                Err(err) => {
                    println!("Failed to send message {}", err.to_string());
                }
            };
    }
}
