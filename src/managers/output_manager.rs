use crate::models::client_snapshot::ClientSnapshot;
use csv::Error as CsvError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OutputError {
    #[error("failed to write to CSV: {0}")]
    CsvWriteError(#[from] CsvError),

    #[error("failed to write to stdout: {0}")]
    IoWriteError(#[from] std::io::Error),
}

pub trait OutputManager {
    fn new() -> Self;
    fn write_output(&self, clients: &[ClientSnapshot]) -> Result<(), OutputError>;
}

pub struct CsvOutputManager;

impl OutputManager for CsvOutputManager {
    fn new() -> Self {
        CsvOutputManager {}
    }

    fn write_output(&self, clients: &[ClientSnapshot]) -> Result<(), OutputError> {
        let mut wtr = csv::Writer::from_writer(std::io::stdout());
        wtr.write_record(["client", "available", "held", "total", "locked"])?;

        for client in clients.iter() {
            let client_id = client.get_id().to_string();
            let available = client.get_available().to_string();
            let held = client.get_held().to_string();
            let total = client.get_total().to_string();
            let locked = client.get_locked().to_string();

            wtr.write_record(&[client_id, available, held, total, locked])?;
        }

        wtr.flush()?;

        Ok(())
    }
}
