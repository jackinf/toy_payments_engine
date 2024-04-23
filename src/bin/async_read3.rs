use std::io::Error;
use futures::future::join_all;
use tokio::fs::{File, metadata};
use tokio::io::{self, AsyncReadExt, AsyncSeekExt};
use tokio::task;

#[tokio::main]
async fn main() -> io::Result<()> {
    read_file_sync();
    read_file_async().await?
}

fn read_file_sync() {
    let very_start = std::time::Instant::now();

    let contents = std::fs::read_to_string("transactions.csv")
        .expect("Should have been able to read the file");

    let duration = very_start.elapsed();
    println!("Time elapsed in read_file_sync() is: {:?}", duration);
}

async fn read_file_async() -> Result<Result<(), Error>, Error> {
    let very_start = std::time::Instant::now();

    let filename = "transactions.csv";
    let metadata = metadata(filename).await?;
    let total_size = metadata.len() as usize;
    let cpus = num_cpus::get(); // amount of streams to read the file
    let chunk_size = total_size / cpus;

    let mut handles = Vec::new();

    for i in 0..cpus {
        let start = i * chunk_size;
        let end = if i == cpus - 1 {
            total_size
        } else {
            (i + 1) * chunk_size
        };

        let handle = task::spawn(async move {
            read_part(filename, start, end).await
        });
        handles.push(handle);
    }

    // let mut all_contents = Vec::new();
    // for handle in handles {
    //     let contents = handle.await??;
    //     all_contents.push(contents);
    // }

    let results = join_all(handles).await;
    println!("All parts read successfully. Data collected from {} streams.", cpus);

    // let everything = all_contents.join("");
    // println!("{}", everything);

    let duration = very_start.elapsed();
    println!("Time elapsed in read_file_async() is: {:?}", duration);

    Ok(Ok(()))
}

async fn read_part(filename: &str, start: usize, end: usize) -> io::Result<String> {
    let mut file: File = File::open(filename).await?;
    file.seek(io::SeekFrom::Start(start as u64)).await?;

    let mut buffer = vec![0; end - start];
    let mut handle = file.take((end - start) as u64);

    handle.read_exact(&mut buffer).await?;
    let text = String::from_utf8(buffer).expect("Invalid UTF-8");
    println!("{:?}", text);

    Ok(text)
}