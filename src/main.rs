pub mod types;

mod models {
    pub mod client;
    pub mod transaction;
}
mod managers {
    pub mod transaction_manager;
}

use csv::Reader;
use std::error::Error;
use std::fs::File;

use crate::managers::transaction_manager::TransactionManager;
use crate::models::transaction::Transaction;
use clap::{Arg, Command};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("CSV Reader")
        .version("1.0")
        .author("Your Name. <your_email@example.com>")
        .about("Toy Engine")
        .arg(
            Arg::new("filename")
                .help("The CSV file to read")
                .required(true)
                .index(1),
        )
        .get_matches();

    // safe to unwrap because the argument is required
    let filename = matches.get_one::<String>("filename").unwrap();
    let file = File::open(filename)?;
    let mut reader = Reader::from_reader(file);

    let mut transaction_manager = TransactionManager::new();

    // read the csv file; each row is streamed into the transaction manager
    for result in reader.records() {
        let transaction = Transaction::from(result?);
        transaction_manager.add_transaction(transaction);
    }

    // TODO: check if the output is streamed to stdout
    let _ = transaction_manager.output_final_state();

    Ok(())
}
