use std::error::Error;
use std::fs::File;
use std::path::Path;
use csv::Reader;
use crate::managers::transaction_manager::TransactionManager;
use crate::models::transaction::Transaction;

pub mod types;
mod models {
    pub mod client;
    pub mod transaction;
}
mod managers {
    pub mod transaction_manager;
}

pub fn read_transactions_from_file<P: AsRef<Path>>(path: P) -> Result<TransactionManager, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = Reader::from_reader(file);

    let mut transaction_manager = TransactionManager::new();

    // read the csv file; each row is streamed into the transaction manager
    for result in reader.records() {
        let transaction: Transaction = result?.into();
        transaction_manager.add_transaction(transaction);
    }
    Ok(transaction_manager)
}
