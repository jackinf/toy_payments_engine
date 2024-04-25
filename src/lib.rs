use crate::managers::output_manager::{CsvOutputManager, OutputError, OutputManager};
use crate::managers::transaction_manager::TransactionManager;
use crate::models::client_snapshot::ClientSnapshot;
use crate::models::transaction::{Transaction, TxError};
use csv::Error as CsvError;
use std::fs::File;
use std::path::Path;
use thiserror::Error;

pub mod models {
    pub mod client;
    pub mod client_snapshot;
    pub mod transaction;
}

mod managers {
    pub mod output_manager;
    pub mod transaction_manager;
}
mod common {
    pub mod types;
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("failed to read from CSV: {0}")]
    CsvReadError(#[from] CsvError),

    #[error("failed to read from stdout: {0}")]
    IoReadError(#[from] std::io::Error),

    #[error("failed to parse transaction: {0}")]
    TxError(#[from] TxError),
}

pub fn run_transactions_from_file<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<ClientSnapshot>, AppError> {
    let file = File::open(path)?;
    let mut reader = csv::Reader::from_reader(file);
    let mut transaction_manager = TransactionManager::new();

    // read the csv file; each row is streamed into the transaction manager
    for result in reader.records() {
        let transaction = Transaction::try_from(result?)?;

        // just ignore any errors for now
        let _ = transaction_manager.add_transaction(transaction);
    }

    let results = transaction_manager.get_all_values();
    Ok(results)
}

pub fn write_output(clients: &[ClientSnapshot]) -> Result<(), OutputError> {
    let output_manager = CsvOutputManager::new();
    output_manager.write_output(clients)
}
