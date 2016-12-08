use flate2::bufread::GzDecoder;
use time::precise_time_ns;

use std::fs;
use std::io;
use std::io::Read;
use std::str;

use errors::*;

const BUFFER_SIZE: usize = 65536;

pub struct Streamer {
    carry: Vec<u8>,
    reader: GzDecoder<io::BufReader<fs::File>>,
}

impl Streamer {
    pub fn new(input_file: &str) -> Result<Streamer, WikiError> {
        let f = fs::File::open(input_file)?;
        let bf = io::BufReader::new(f);
        let rdr = GzDecoder::new(bf)?;
        Ok(Streamer { carry: Vec::new(),
                      reader: rdr })
    }

    fn return_line(&mut self) -> Option<Vec<u8>> {
        if let Some(position) = self.carry.iter().position(|&c| c == 0xA) {
            let rem = self.carry.split_off(position+1);
            let ret = Some(self.carry.clone());
            self.carry = rem;
            ret
        } else {
            None
        }
    }

    fn read_to_eol(&mut self) -> Option<Vec<u8>> {
        let mut ret = None;
        if let Some(ret) = self.return_line() {
            return Some(ret);
        }
        loop {
            let mut buffer : [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
            match self.reader.read(&mut buffer) {
                Ok(size) if size > 0 => {
                    self.carry.extend_from_slice(&buffer[0..size]);
                    if let Some(ret1) = self.return_line() {
                        ret = Some(ret1);
                        break;
                    }
                },
                _ => {
                    break;
                },
            }
        };
        ret
    }
}

impl Iterator for Streamer {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let t0 = precise_time_ns();
        let mut ret = None;
        match self.read_to_eol() {
            Some(line) => {
                if let Ok(s) = String::from_utf8(line) {
                    ret = Some(s)
                }
            },
            None => {},
        };
        let t1 = precise_time_ns();
        println!("Streamer::next: {} us", (t1 - t0) / 1000 );
        ret
    }
}
