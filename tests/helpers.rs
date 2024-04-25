use csv::StringRecord;
use rust_decimal::Decimal;
use std::error::Error;
use std::path::PathBuf;
use std::result;
use std::str::FromStr;
use toy_payments_engine::models::client_snapshot::ClientSnapshot;

type Result<T> = result::Result<T, Box<dyn Error>>;

pub(crate) fn get_test_file_path(path: &str) -> PathBuf {
    let mut test_file_path = PathBuf::from(file!());
    test_file_path.pop();
    test_file_path.push(path);
    test_file_path
}

pub(crate) fn read_csv(file_path: &PathBuf) -> Result<Vec<StringRecord>> {
    let mut reader = csv::Reader::from_path(file_path)?;
    let mut records = Vec::new();

    for result in reader.records() {
        let record = result?;
        records.push(record);
    }

    Ok(records)
}

#[derive(Debug)]
pub struct OutputItem {
    pub id: u16,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
}

pub fn deserialize_output_lines(output_lines_raw: Vec<StringRecord>) -> Vec<OutputItem> {
    output_lines_raw
        .iter()
        .map(|record| {
            let id = record.get(0).unwrap().parse::<u16>().unwrap();
            let available = Decimal::from_str(record.get(1).unwrap()).unwrap();
            let held = Decimal::from_str(record.get(2).unwrap()).unwrap();
            let total = Decimal::from_str(record.get(3).unwrap()).unwrap();

            OutputItem {
                id,
                available,
                held,
                total,
            }
        })
        .collect()
}

pub fn compare_expected_output_with_actual(
    output_lines: &mut Vec<OutputItem>,
    mut clients: Vec<ClientSnapshot>,
) {
    // sort by id to make sure the order is the same when comparing rows
    output_lines.sort_by(|a, b| a.id.cmp(&b.id));
    clients.sort_by(|a, b| a.get_id().cmp(&b.get_id()));

    assert_eq!(output_lines.len(), clients.len());
    output_lines
        .iter()
        .zip(clients.iter())
        .for_each(|(expected, actual)| {
            assert_eq!(expected.available, actual.get_available());
            assert_eq!(expected.held, actual.get_held());
            assert_eq!(expected.total, actual.get_total());
        });
}
