use flate2::bufread::GzDecoder;

use errors::*;

use std::fs;
use std::io;
use std::io::Read;
use std::str;

pub fn make_reader(input_file: &str) -> Result<GzDecoder<io::BufReader<fs::File>>,
                                               WikiError> {
    let f = fs::File::open(input_file)?;
    let bf = io::BufReader::new(f);
    let rdr = GzDecoder::new(bf)?;
    Ok(rdr)
}

pub struct Streamer {
    buffer: String,
    bytes: io::Bytes<GzDecoder<io::BufReader<fs::File>>>,
}

// TODO: Is the fact that we are copying into "buffer" costing us a lot?
impl Streamer {
    pub fn new(input_file: &str) -> Result<Streamer, WikiError> {
        let f = fs::File::open(input_file)?;
        let bf = io::BufReader::new(f);
        let rdr = GzDecoder::new(bf)?;
        let bytes = rdr.bytes();
        Ok(Streamer { buffer: String::new(),
                      bytes: bytes })
    }

    fn advance_to_eol(&mut self) {
        for b in &mut self.bytes {
            if let Ok(bite) = b {
                if bite == 0xA {
                    break;
                } else {
                    if let Ok(bb) = str::from_utf8(&[bite]) {
                        self.buffer.push_str(bb);
                    }
                }
            }
        }
    }
}

// TODO: Can we make the iterator return string slices?
impl Iterator for Streamer {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        self.buffer.clear();
        self.advance_to_eol();
        if self.buffer.len() > 0 {
            Some(self.buffer.to_owned())
        } else {
            None
        }
    }
}
