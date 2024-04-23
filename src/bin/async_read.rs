use std::{fs};
use clap::{Arg, Command};
use tokio::fs::File;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

// experimenting on  reading the file in chunks
#[tokio::main]
async fn main() {
    let matches = Command::new("CSV Reader")
        .version("1.0")
        .author("Jevgeni Rumjantsev. <zeka.rum@@gmail.com>")
        .about("Toy Engine")
        .arg(
            Arg::new("filename")
                .help("The CSV file to read")
                .required(false)
                .default_value("transactions.csv")
                .index(1),
        )
        .get_matches();

    // Get the filename from the command line arguments
    let filename = matches.get_one::<String>("filename").unwrap();
    println!("Reading file: {}", filename);

    let file_meta = fs::metadata(&filename).unwrap();
    let file_size = file_meta.len();
    let cpus = 4; // num_cpus::get();
    let chunk_size = (file_size / cpus as u64).max(1);
    println!("File size: {}, CPUs: {}, Chunk size: {}", file_size, cpus, chunk_size);

    // spawn cpus threads
    let handles = (0..cpus as u64)
        .map(|i| {
            let filename = filename.clone();
            // we're moving the filename into async scope
            tokio::spawn(async move {
                do_work(&filename, i * chunk_size, chunk_size).await
            })
        })
        .collect::<Vec<_>>();

    // wait for all threads to finish
    // use futures lib to join all
    futures::future::join_all(handles).await;
}

async fn do_work(filename: &str, offset: u64, size: u64) {
    let start = offset;
    let mut file: File = File::open(filename).await.unwrap();
    file.seek(io::SeekFrom::Start(offset)).await.unwrap();

    let mut buffer = vec![0; size as usize];
    let mut handle = file.take(size);

    handle.read_exact(&mut buffer).await.unwrap();
    let text = String::from_utf8(buffer).expect("Invalid UTF-8");

    println!("Chunk start: {}, end: {}, size: {}", start, start + size, size);
    println!("{}", text);
}