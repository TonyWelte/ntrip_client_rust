use crc_any::CRC;

pub mod ntrip_client {

    pub struct RtcmParser {
        buffer: Vec<u8>,
        max_size: usize,
        crc24: crc_any::CRC,
    }

    impl RtcmParser {
        pub fn new() -> RtcmParser {
            RtcmParser {
                buffer: vec![],
                max_size: 10000,
                crc24: crc_any::CRC::create_crc(0b1100001100100110011111011, 24, 0, 0, false),
            }
        }

        pub fn parse(&mut self, input: &mut Vec<u8>) -> Vec<Vec<u8>> {
            // Update buffer
            self.buffer.append(input);

            let mut result: Vec<Vec<u8>> = Vec::new();

            // Scan for RTCM preamble
            let RTCM_3_2_PREAMBLE = 0b11010011;
            let mut last_i = 0;
            for i in 0..self.buffer.len() - 2 {
                // - 2 prevent checking when the message size will not be available
                if self.buffer[i] != RTCM_3_2_PREAMBLE {
                    continue;
                }

                // println!("RTCM Preamble found");

                // Read message length (stored in 10 bits)
                let rtcm_length = ((usize::from(self.buffer[i + 1]) << 8)
                    + usize::from(self.buffer[i + 2]))
                    & 0x3FFusize;
                if i + 6 + rtcm_length >= self.buffer.len() {
                    continue; // This might not be a real message so the rest of the buffer still need to be checked
                }

                // println!("RTCM length: {rtcm_length}");

                // Compute the checksum
                self.crc24.digest(&self.buffer[i..i + 3 + rtcm_length]);
                let checksum_computed = self.crc24.get_crc();
                self.crc24.reset();
                let checksum_message = (u64::from(self.buffer[i + 3 + rtcm_length]) << 16)
                    + (u64::from(self.buffer[i + 3 + rtcm_length + 1]) << 8)
                    + u64::from(self.buffer[i + 3 + rtcm_length + 2]);

                if checksum_computed == checksum_message {
                    // println!("RTCM message found");
                } else {
                    // Bad checksum
                    continue;
                }

                result.push(self.buffer[i..i + 6 + rtcm_length].to_vec());
                last_i = i + 6 + rtcm_length;
            }

            self.buffer.drain(..last_i);

            let result_length = result.len();
            // println!("{result_length} messages found");

            return result;
        }
    }
}
