use std::collections::HashMap;
use std::io::{Write, Read};
use std::{env, fs::File};
use std::net::TcpStream;
use ntrip_client::rtcm_parser::RtcmParser;
use itertools::Itertools;

fn print_hex(buf: &[u8], width: usize) {
    for(i, v) in buf.iter().enumerate() {
        if i != 0 && i % width == 0 {
            println!("");
        }
        print!("{v:02X} ");
    }
    println!();
}

fn main() {
    // Arguments
    let args: Vec<String>  = env::args().collect();

    let address = &args[1];
    let mountpoint = &args[2];
    
    // Optional save file
    let mut output_file = if args.len() == 4 {
        match File::create(&args[3]) {
            Ok(file) => Some(file),
            Error => None
        }
    }
    else {
        None
    };
    
    println!("Attempting connection:");
    println!("- Address: {address}");
    println!("- Mountpoint: {mountpoint}");

    // Prepare RTCM parser
    let mut parser = RtcmParser::new();

    // Open connection
    if let Ok(mut stream) = TcpStream::connect(address) {
        // Send GET request
        let request = format!("GET /{mountpoint} HTTP/1.0\r\nUser-Agent: NTRIP ntrip_client_rust\r\n\r\n");

        // Read stream
        match stream.write_all(&request.as_bytes()) {
            Ok(_) => {
                let mut received_msg_id: HashMap<u16, isize> = HashMap::new();

                let mut buffer: [u8; 256] = [0; 256];
                while let Ok(_) = stream.read_exact(&mut buffer) {
                    // print_hex(buffer, 8);

                    let messages = parser.parse(&mut buffer.to_vec());

                    for msg in &messages {
                        let msg_id = u16::from(msg[3]) << 4 | (u16::from(msg[4]) >> 4);
                        *received_msg_id.entry(msg_id).or_insert(0) +=1;
                    }

                    for msg in &messages {
                        print_hex(&msg[..], 16);
                    }

                    println!("Received messages:");
                    for id  in received_msg_id.keys().sorted() {
                        let value = received_msg_id[id];
                        println!("  - {id}: {value}");
                    }
                    
                    // Write file
                    if let Some(mut file) = output_file.as_ref() {
                        file.write_all(&buffer[..]);
                    }
                }
            }
            Err(error) => {
                eprintln!("Error: {error}");
            }
        }

    }
}