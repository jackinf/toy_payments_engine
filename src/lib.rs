use crate::managers::output_manager::{CsvOutputManager, OutputError, OutputManager};
use crate::managers::transaction_manager::TransactionManager;
use crate::models::client_snapshot::ClientSnapshot;
use crate::models::transaction::Transaction;
use std::error::Error;
use std::fs::File;
use std::path::Path;

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

pub fn run_transactions_from_file<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<ClientSnapshot>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = csv::Reader::from_reader(file);
    let mut transaction_manager = TransactionManager::new();

    // read the csv file; each row is streamed into the transaction manager
    for result in reader.records() {
        let transaction = Transaction::try_from(result?).expect("Invalid format in CSV file");

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
