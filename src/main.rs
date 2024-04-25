use clap::{Arg, Command};
use std::error::Error;
use toy_payments_engine::{run_transactions_from_file, write_output};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("CSV Reader")
        .version("1.0")
        .author("Your Name. <your_email@example.com>")
        .about("Toy Engine")
        .arg(
            Arg::new("filename")
                .help("The CSV file to read")
                .required(false)
                .default_value("transactions.csv")
                .index(1),
        )
        .get_matches();

    // safe to unwrap because the argument is required
    let filename = matches.get_one::<String>("filename").unwrap();

    let clients = run_transactions_from_file(filename)?;
    write_output(&clients)?;

    Ok(())
}
