use crate::models::client::Client;

pub trait OutputManager {
    fn new() -> Self;
    fn write_output(&self, clients: &[Client]) -> Result<(), String>;
}

pub struct CsvOutputManager;

impl OutputManager for CsvOutputManager {
    fn new() -> Self {
        CsvOutputManager {}
    }

    fn write_output(&self, clients: &[Client]) -> Result<(), String> {
        let mut wtr = csv::Writer::from_writer(std::io::stdout());
        if let Err(err) = wtr.write_record(["client", "available", "held", "total"]) {
            return Err(err.to_string());
        }

        for client in clients.iter() {
            let client_id = client.get_id().to_string();
            let available = client.get_available().to_string();
            let held = client.get_held().to_string();
            let total = client.get_total().to_string();

            if let Err(err) = wtr.write_record(&[client_id, available, held, total]) {
                return Err(err.to_string());
            }
        }

        if let Err(err) = wtr.flush() {
            return Err(err.to_string());
        }

        Ok(())
    }
}
