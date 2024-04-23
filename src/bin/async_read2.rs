use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use rayon::prelude::*;

// experimenting on processing multiple lines in parallel
#[tokio::main]
async fn main() -> io::Result<()> {
    let file = File::open("transactions.csv").await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Use a buffer to collect a batch of lines
    let mut buffer = Vec::with_capacity(1000);

    while let Some(line) = lines.next_line().await? {
        buffer.push(line);

        // Process in batches
        if buffer.len() >= 1000 {
            let to_process = std::mem::replace(&mut buffer, Vec::with_capacity(1000));
            tokio::task::spawn_blocking(move || {
                process_lines(to_process);
            });
        }
    }

    // Process any remaining lines
    if !buffer.is_empty() {
        tokio::task::spawn_blocking(move || {
            process_lines(buffer);
        });
    }

    Ok(())
}

fn process_lines(lines: Vec<String>) {
    lines.into_par_iter().for_each(|line| {
        // Here you can parse and process each line
        // For example, split the line on commas, convert data types, etc.
        println!("Processed: {}", line);
    });
}
