use std::collections::HashMap;
use csv::{Reader, StringRecord};
use std::error::Error;
use std::fs::File;
use std::str::FromStr;

use clap::{Command, Arg};
use rust_decimal::Decimal;

#[derive(Debug, PartialEq)]
enum InputRowTransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

type ClientId = u16;
type TransactionId = u32;

#[derive(Debug)]
struct InputRow {
    transaction_type: InputRowTransactionType,
    client: ClientId,
    tx: TransactionId,
    available: Decimal,
    held: Decimal,
    total: Decimal,
}

#[derive(Debug)]
struct DatabaseRow {
    available: Decimal,
    held: Decimal,
    total: Decimal,
}

impl Into<InputRow> for StringRecord {
    fn into(self) -> InputRow {
        let transaction_type = match self.get(0).unwrap() {
            "deposit" => InputRowTransactionType::Deposit,
            "withdrawal" => InputRowTransactionType::Withdrawal,
            "dispute" => InputRowTransactionType::Dispute,
            "resolve" => InputRowTransactionType::Resolve,
            "chargeback" => InputRowTransactionType::Chargeback,
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
            available: amount.clone(),
            held: Decimal::ZERO,
            total: amount,
        }
    }
}

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

    // transactions are globally unique, use as keys.
    let mut transaction_db: HashMap<TransactionId, InputRow> = HashMap::new();
    transaction_db.extend(rows.into_iter().map(|row| (row.tx, row)));

    let mut client_db: HashMap<ClientId, DatabaseRow> = HashMap::new();
    apply_transactions(&mut transaction_db, &mut client_db);

    // write into stdout the csv table
    let mut wtr = csv::Writer::from_writer(std::io::stdout());
    wtr.write_record(&["client", "available", "held", "total"])?;
    for (client, row) in client_db.iter() {
        wtr.write_record(&[client.to_string(), row.available.to_string(), row.held.to_string(), row.total.to_string()])?;
    }
    wtr.flush()?;

    Ok(())
}

fn apply_transactions(transaction_db: &mut HashMap<TransactionId, InputRow>, client_db: &mut HashMap<ClientId, DatabaseRow>) {
    for (_, row) in transaction_db {
        let client_db = client_db.entry(row.client).or_insert(DatabaseRow {
            available: Decimal::ZERO,
            held: Decimal::ZERO,
            total: Decimal::ZERO,
        });

        match row.transaction_type {
            InputRowTransactionType::Deposit => {
                client_db.available += row.total;
                client_db.total += row.total;
            }
            InputRowTransactionType::Withdrawal => {
                client_db.available -= row.total;
                client_db.total -= row.total;
            }
            InputRowTransactionType::Dispute => {
                client_db.available -= row.total;
                client_db.held += row.total;
            }
            InputRowTransactionType::Resolve => {
                client_db.available += row.total;
                client_db.held -= row.total;
            }
            InputRowTransactionType::Chargeback => {
                client_db.held -= row.total;
                client_db.total -= row.total;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_row_from_string_record() {
        let record = StringRecord::from(vec!["deposit", "1", "1", "1.0"]);
        let row1: InputRow = record.into();
        assert_eq!(row1.transaction_type, InputRowTransactionType::Deposit);
        assert_eq!(row1.client, 1);
        assert_eq!(row1.tx, 1);
        assert_eq!(row1.available, Decimal::new(10, 1));
        assert_eq!(row1.held, Decimal::ZERO);
        assert_eq!(row1.total, Decimal::new(10, 1));

        let record = StringRecord::from(vec!["deposit", "2", "2", "2.0"]);
        let row2: InputRow = record.into();
        assert_eq!(row2.transaction_type, InputRowTransactionType::Deposit);
        assert_eq!(row2.client, 2);
        assert_eq!(row2.tx, 2);
        assert_eq!(row2.available, Decimal::new(20, 1));

        let record = StringRecord::from(vec!["deposit", "1", "3", "2.0"]);
        let row3: InputRow = record.into();
        assert_eq!(row3.transaction_type, InputRowTransactionType::Deposit);
        assert_eq!(row3.client, 1);
        assert_eq!(row3.tx, 3);
        assert_eq!(row3.available, Decimal::new(20, 1));

        let record = StringRecord::from(vec!["withdrawal", "1", "4", "1.5"]);
        let row4: InputRow = record.into();
        assert_eq!(row4.transaction_type, InputRowTransactionType::Withdrawal);
        assert_eq!(row4.client, 1);
        assert_eq!(row4.tx, 4);
        assert_eq!(row4.available, Decimal::new(15, 1));

        let record = StringRecord::from(vec!["withdrawal", "2", "5", "3.0"]);
        let row5: InputRow = record.into();
        assert_eq!(row5.transaction_type, InputRowTransactionType::Withdrawal);
        assert_eq!(row5.client, 2);
        assert_eq!(row5.tx, 5);
        assert_eq!(row5.available, Decimal::new(30, 1));

        let rows = vec![row1, row2, row3, row4, row5];

        let mut transaction_db: HashMap<TransactionId, InputRow> = HashMap::new();
        transaction_db.extend(rows.into_iter().map(|row| (row.tx, row)));

        let mut client_db: HashMap<ClientId, DatabaseRow> = HashMap::new();

        // Subject under test
        apply_transactions(&mut transaction_db, &mut client_db);

        assert_eq!(client_db.len(), 2);
        assert_eq!(client_db.get(&1).unwrap().available, Decimal::new(15, 1));
        assert_eq!(client_db.get(&1).unwrap().held, Decimal::ZERO);
        assert_eq!(client_db.get(&1).unwrap().total, Decimal::new(15, 1));

        assert_eq!(client_db.get(&2).unwrap().available, Decimal::new(-10, 1));
        assert_eq!(client_db.get(&2).unwrap().held, Decimal::ZERO);
        assert_eq!(client_db.get(&2).unwrap().total, Decimal::new(-10, 1));
    }
}