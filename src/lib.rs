pub mod ntrip_client {
    use deku::error::DekuError;
    use rtcm_parser::rtcm_parser::{Rtcm, RtcmParser};
    use std::{fs::File, io::Write, net::AddrParseError, time::Duration};

    use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
    use tokio::{
        io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
        net::TcpStream,
        time::timeout,
    };

    #[derive(Debug, Clone)]
    pub struct NtripConfig {
        url: String,
        port: String,
        mountpoint: String,
        credentials: String,
    }

    #[derive(Debug)]
    pub struct NtripConnection {
        config: NtripConfig,
        stream: TcpStream,
    }

    #[derive(Debug)]
    pub enum NtripClientError {
        ParseError,
        IoError,
        DekuError,
        Err,
    }

    impl From<AddrParseError> for NtripClientError {
        fn from(_: AddrParseError) -> Self {
            NtripClientError::ParseError
        }
    }

    impl From<std::io::Error> for NtripClientError {
        fn from(_: io::Error) -> Self {
            NtripClientError::IoError
        }
    }

    impl From<DekuError> for NtripClientError {
        fn from(_: DekuError) -> Self {
            NtripClientError::DekuError
        }
    }

    impl NtripConfig {
        pub fn new(
            url: &str,
            port: &str,
            mountpoint: &str,
            username: &str,
            password: &str,
        ) -> Self {
            let credentials = format!("{}:{}", username, password);
            let encoded_credentials = STANDARD_NO_PAD.encode(credentials);

            NtripConfig {
                url: url.to_string(),
                port: port.to_string(),
                mountpoint: mountpoint.to_string(),
                credentials: encoded_credentials,
            }
        }

        fn get_formated_address(&self) -> String {
            format!("{}:{}", self.url, self.port)
        }

        pub async fn connect(&self) -> Result<NtripConnection, NtripClientError> {
            let full_url = self.get_formated_address();

            let stream = TcpStream::connect(full_url).await?;

            Ok(NtripConnection::new(self.clone(), stream))
        }
    }

    fn fromat_get_request(mountpoint: &str) -> String {
        format!(
            "GET /{} HTTP/1.0\r\nUser-Agent: ntrip_client_rust\r\n\r\n",
            mountpoint
        )
    }

    fn print_hex(buf: &[u8], width: usize) {
        for (i, v) in buf.iter().enumerate() {
            if i != 0 && i % width == 0 {
                println!("");
            }
            print!("{v:02X} ");
        }
        println!();
    }

    impl NtripConnection {
        pub fn new(config: NtripConfig, stream: TcpStream) -> Self {
            NtripConnection { config, stream }
        }

        pub async fn get_caster_table(&mut self) -> Result<String, NtripClientError> {
            let mut stream = BufReader::new(&mut self.stream);

            // Send request
            let request = fromat_get_request("");
            stream.write_all(request.as_bytes()).await?;

            // Receive response
            let mut response = String::new();
            while let Ok(_) =
                timeout(Duration::from_millis(500), stream.read_line(&mut response)).await
            {
                // Check end of the HTTP Response
                if response.contains("\r\n\r\n") {
                    break;
                }
            }

            Ok(response)
        }
    }

    pub async fn read_stream(
        connection: NtripConnection,
        output: Option<File>,
    ) -> Result<(), NtripClientError> {
        let mut stream = BufReader::new(connection.stream);

        // Send request
        let request = fromat_get_request(&connection.config.mountpoint);
        stream.write_all(request.as_bytes()).await?;

        let mut response = String::new();
        stream.read_line(&mut response).await?;
        stream.read_line(&mut response).await?;
        println!("{response}");

        // Initialize the RTCM parser
        let mut parser = RtcmParser::new();

        // Read stream and print to screen
        let mut buffer = [0; 256];
        while let Ok(Ok(n)) = timeout(Duration::from_secs(5), stream.read(&mut buffer)).await {
            if let Some(mut file) = output.as_ref() {
                file.write_all(&buffer[0..n])?;
            }

            let messages = parser.parse(&buffer[..n]);

            // Debug
            for msg in messages {
                let rtcm = Rtcm::parse(&msg[3..msg.len() - 3])?;
                match rtcm {
                    Rtcm::Rtcm1005(msg) => {
                        println!("{msg:?}");
                    }
                    Rtcm::RtcmMSM7(msg) => {
                        println!("{msg}");
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}
