#[macro_use]
extern crate log;

mod map_adapter;
mod traits;
mod transaction_service;
mod types;

use crate::map_adapter::MapAdapter;
use crate::transaction_service::TransactionService;
use crate::types::{Action, Transaction};
use csv::Trim;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::env;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut rdr = csv::ReaderBuilder::new()
        .trim(Trim::All)
        .flexible(true)
        .from_path(path)
        .unwrap();

    let mut raw_record = csv::StringRecord::new();
    let headers = rdr.headers().unwrap().clone();

    let (transaction_sender, transaction_receiver) = mpsc::channel(256);

    let db_adapter = MapAdapter::new();
    let transaction_service = TransactionService { db_adapter };

    let finished = Arc::new(AtomicBool::new(false));
    let (status_sender, mut status_receiver) = mpsc::channel(1);

    let must_stop = finished.clone();
    tokio::spawn(async move {
        loop {
            while let Some(status) = status_receiver.recv().await {
                if let Action::DisplayTransactionFinished = status {
                    must_stop.store(true, Ordering::Relaxed);
                }
            }
        }
    });

    tokio::spawn(async move {
        transaction_service
            .receive(transaction_receiver, status_sender)
            .await;
    });

    while rdr.read_record(&mut raw_record).unwrap() {
        let record: Transaction = raw_record.deserialize(Some(&headers)).unwrap();

        match transaction_sender
            .send(Action::NewTransaction(record))
            .await
        {
            Ok(_) => {}
            Err(err) => {
                error!("Failed to send transaction {}", err.to_string());
            }
        };
    }

    transaction_sender
        .send(Action::DisplayTransaction)
        .await
        .ok();
    while !finished.load(Ordering::Acquire) {}
}
