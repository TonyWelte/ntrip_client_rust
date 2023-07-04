use ntrip_client::ntrip_client::{NtripClientError, NtripConfig};
use std::{env, fs::File};

fn get_output_file(args: &Vec<String>) -> Option<File> {
    if args.len() == 7 {
        match File::create(&args[6]) {
            Ok(file) => Some(file),
            Error => None,
        }
    } else {
        None
    }
}

#[tokio::main]
async fn main() -> Result<(), NtripClientError> {
    // Arguments
    let args: Vec<String> = env::args().collect();

    let output_file = get_output_file(&args);

    let server = NtripConfig::new(&args[1], &args[2], &args[3], &args[4], &args[5]);

    // Create connection
    println!("Connecting to {server:?}");
    let connection = server.connect().await.unwrap();

    ntrip_client::ntrip_client::read_stream(connection, output_file).await?;

    Ok(())
}
