use flate2::bufread::GzDecoder;
use time::precise_time_ns;

use std::fs;
use std::io;
use std::io::Read;
use std::str;

use errors::*;

pub struct Streamer {
    buffer: Vec<u8>,
    bytes: io::Bytes<GzDecoder<io::BufReader<fs::File>>>,
}

impl Streamer {
    pub fn new(input_file: &str) -> Result<Streamer, WikiError> {
        let f = fs::File::open(input_file)?;
        let bf = io::BufReader::new(f);
        let rdr = GzDecoder::new(bf)?;
        let bytes = rdr.bytes();
        Ok(Streamer { buffer: Vec::new(),
                      bytes: bytes })
    }

    fn advance_to_eol(&mut self) {
        let itr = (&mut self.bytes).take_while(|b| match b {
            &Ok(bb) => { bb != 0xA },
            _ => { false },
        });
        for b in itr {
            if let Ok(bb) = b {
                self.buffer.push(bb);
            }
        }
    }
}

impl Iterator for Streamer {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let t0 = precise_time_ns();
        self.buffer.clear();
        self.advance_to_eol();
        let ret = if self.buffer.len() > 0 {
            if let Ok(s) = String::from_utf8(self.buffer.clone()) {
                Some(s)
            } else {
                None
            }
        } else {
            None
        };
        let t1 = precise_time_ns();
        println!("Streamer::next: {} us", (t1 - t0) / 1000 );
        ret
    }
}
