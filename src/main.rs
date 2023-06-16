use rtcm_parser::rtcm_parser::Rtcm;
use rtcm_parser::rtcm_parser::RtcmParser;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::{env, fs::File};

fn print_hex(buf: &[u8], width: usize) {
    for (i, v) in buf.iter().enumerate() {
        if i != 0 && i % width == 0 {
            println!("");
        }
        print!("{v:02X} ");
    }
    println!();
}

fn main() {
    // Arguments
    let args: Vec<String> = env::args().collect();

    let address = &args[1];
    let mountpoint = &args[2];

    // Optional save file
    let mut output_file = if args.len() == 4 {
        match File::create(&args[3]) {
            Ok(file) => Some(file),
            Error => None,
        }
    } else {
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
        let request =
            format!("GET /{mountpoint} HTTP/1.0\r\nUser-Agent: NTRIP ntrip_client_rust\r\n\r\n");

        // Read stream
        match stream.write_all(&request.as_bytes()) {
            Ok(_) => {
                let mut received_msg_id: HashMap<u16, isize> = HashMap::new();

                let mut buffer: [u8; 256] = [0; 256];
                while let Ok(_) = stream.read_exact(&mut buffer) {
                    // print_hex(buffer, 8);

                    let messages = parser.parse(&mut buffer.to_vec());

                    for msg in &messages {
                        // Parse message content (skipping preample and checksum)
                        let rtcm = Rtcm::parse(&msg[3..msg.len() - 3]).unwrap();
                        let msg_id = match rtcm {
                            Rtcm::Rtcm1001(rtcm) => {
                                println!("{rtcm:?}");
                                rtcm.header.message_number
                            }
                            Rtcm::Rtcm1002(rtcm) => {
                                println!("{rtcm:?}");
                                rtcm.header.message_number
                            }
                            Rtcm::Rtcm1003(rtcm) => {
                                println!("{rtcm:?}");
                                rtcm.header.message_number
                            }
                            Rtcm::Rtcm1004(rtcm) => {
                                println!("{rtcm:?}");
                                rtcm.header.message_number
                            }
                            Rtcm::Rtcm1005(rtcm) => {
                                println!("{rtcm:?}");
                                rtcm.message_number
                            }
                            Rtcm::Rtcm1006(rtcm) => {
                                println!("{rtcm:?}");
                                rtcm.message_number
                            }
                            Rtcm::RtcmMSM7(rtcm) => {
                                println!("{rtcm:?}");
                                rtcm.header.message_number
                            }
                            Rtcm::UnsupportedType(id) => id,
                        };

                        *received_msg_id.entry(msg_id).or_insert(0) += 1;
                    }

                    // for msg in &messages {
                    //     print_hex(&msg[..], 16);
                    // }

                    // println!("Received messages:");
                    // for id in received_msg_id.keys().sorted() {
                    //     let value = received_msg_id[id];
                    //     println!("  - {id}: {value}");
                    // }

                    // Write file
                    if let Some(mut file) = output_file.as_ref() {
                        file.write_all(&buffer[..]);
                    }
                }
                println!("End of reading");
            }
            Err(error) => {
                eprintln!("Error: {error}");
            }
        }
    } else {
        println!("Failed to connect to the server");
    }
}
