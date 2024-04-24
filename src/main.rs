use std::error::Error;
use clap::{Arg, Command};
use toy_payments_engine::read_transactions_from_file;

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

    let transaction_manager = read_transactions_from_file(filename)?;

    // TODO: check if the output is streamed to stdout
    let _ = transaction_manager.output_final_state();

    Ok(())
}
