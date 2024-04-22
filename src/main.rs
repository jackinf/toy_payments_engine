use std::cmp::Ordering;
use std::collections::HashMap;
use csv::{Reader, StringRecord};
use std::error::Error;
use std::fs::File;
use std::str::FromStr;

use clap::{Command, Arg};
use rust_decimal::Decimal;

#[derive(Debug)]
enum InputRowTransactionType {
    Deposit,
    Withdrawal
}

#[derive(Debug)]
struct InputRow {
    transaction_type: InputRowTransactionType,
    client: u16,
    tx: u32,
    amount: Decimal
}

// impl Eq for InputRow {}
//
// impl PartialEq<Self> for InputRow {
//     fn eq(&self, other: &Self) -> bool {
//         self.client == other.client
//     }
// }
//
// impl PartialOrd<Self> for InputRow {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
//
// impl Ord for InputRow {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.client.cmp(&other.client) && self.tx.cmp(&other.tx)
//     }
// }

impl Into<InputRow> for StringRecord {
    fn into(self) -> InputRow {
        let transaction_type = match self.get(0).unwrap() {
            "deposit" => InputRowTransactionType::Deposit,
            "withdrawal" => InputRowTransactionType::Withdrawal,
            _ => panic!("Invalid transaction type")
        };

        let client_str = self.get(1).unwrap();
        let tx_str = self.get(2).unwrap();
        let amount_str = self.get(3).unwrap();

        let client = client_str.parse::<u16>().unwrap();
        let tx = tx_str.parse::<u32>().unwrap();
        let amount = Decimal::from_str(amount_str).unwrap();

        InputRow {
            transaction_type,
            client,
            tx,
            amount
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("CSV Reader")
        .version("1.0")
        .author("Your Name. <your_email@example.com>")
        .about("Reads CSV files and prints to stdout")
        .arg(
            Arg::new("filename")
                .help("The CSV file to read")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Get the filename from the command line arguments
    let filename = matches.get_one::<String>("filename").unwrap();

    let file = File::open(filename)?;
    let mut reader = Reader::from_reader(file);

    // Iterate through each record
    let mut rows = vec![];
    for result in reader.records() {
        let record = result?;
        let row: InputRow = record.into();
        rows.push(row);
    }

    for row in &rows {
        println!("{:?}", row);
    }

    let mut db = HashMap::new();
    // &rows.iter().for_each(|row| {
    //     let client = row.client;
    //     let amount = row.amount;
    //     let client_db = db.entry(client).or_insert(Decimal::ZERO);
    //     match row.transaction_type {
    //         InputRowTransactionType::Deposit => {
    //             *client_db += amount;
    //         }
    //         InputRowTransactionType::Withdrawal => {
    //             *client_db -= amount;
    //         }
    //     }
    // });

    for (client, amount) in db.iter() {
        println!("Client: {}, Amount: {}", client, amount);
    }

    Ok(())
}
