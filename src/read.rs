use flate2::bufread::GzDecoder;

use errors::*;

use std::fs;
use std::io;

pub fn read_file(input_file: &str) -> Result<GzDecoder<io::BufReader<fs::File>>,
                                         WikiError> {
    let f = fs::File::open(input_file)?;
    let bf = io::BufReader::new(f);
    let rdr = GzDecoder::new(bf)?;
    Ok(rdr)
}

