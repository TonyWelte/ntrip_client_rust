# NTRIP Client

This is a Rust program that acts as an NTRIP client, allowing you to connect to an NTRIP server and receive RTCM (Radio Technical Commission for Maritime Services) data. The program can then write the received RTCM data to a file.

## Features

The current version of the program includes the following features:

- Connects to an NTRIP server and receives RTCM data.
- Writes the received RTCM data to a file.

## Planned Features

The following features are planned for future versions of the program:

- [ ] Parsing the sourcetable: The program will be able to parse the sourcetable received from the NTRIP server. This will allow you to view information about available mountpoints and their characteristics.

- [ ] ~~Parsing RTCM messages for analysis: The program will provide functionality to parse individual RTCM messages received from the NTRIP server. This will enable you to analyze the content of the messages and extract relevant information.~~ (see [rtcm_parser](https://github.com/TonyWelte/rtcm_parser))

- [ ] Multiple connections: Currently, the program supports a single connection to an NTRIP server. In future updates, the program will be enhanced to support multiple simultaneous connections. This will allow you to connect to and receive data from multiple NTRIP servers simultaneously.

- [ ] Add support for username/password protected caster 

## Getting Started

To run the program, follow these steps:

1. Install Rust: If you haven't already, you'll need to install Rust on your system. Visit the official Rust website (https://www.rust-lang.org/) for installation instructions.

2. Clone the repository: Clone this repository to your local machine using Git.

   ```bash
   $ git clone https://github.com/TonyWelte/ntrip-client.git
   $ cd ntrip-client
   ```

3. Build the program: Use Cargo, the Rust package manager, to build the program.

   ```bash
   $ cargo build
   ```

4. Run the program: Execute the compiled binary to run the program, passing the necessary arguments.

   ```bash
   $ cargo run caster.ntrip.com 2101 QWERTY username password [output_file]
   ```

   Replace `caster.ntrip.com` with the address and `2101` with the port port of the NTRIP server you want to connect to, and `QWERTY` with the mountpoint you wish to use. The `username` and `password` are placeholders and not currently implemented. An optional `output_file` can be provided.

## Configuration

The program accepts the NTRIP server address, port, and mountpoint as command-line arguments. You can specify these values directly when running the program, as shown in the example above.