extern crate wikidata;
use wikidata::parse::parse_item;
use wikidata::read::read_file;

#[macro_use]
extern crate clap;

use std::io::Read;

extern crate time;

fn main() {
    let t0 = time::precise_time_ns();
    let matches = clap_app!(myapp =>
        (version: "0.1.0")
        (author: "Radu Popescu <mail@radupopescu.net>")
        (about: "Process the Wikidata JSON dump with Rust")
        (@arg FILE: -f --file +takes_value +required "Wikidump data file")
        (@arg LANGUAGES: -l --languages +takes_value +required +multiple "List of languages to use")
    ).get_matches();

    let input_file = matches.value_of("FILE").unwrap(); // safe <= required parameter
    println!("Input file: {}", input_file);

    let languages = matches
        .values_of("LANGUAGES")
        .unwrap()
        .collect::<Vec<_>>(); // safe <= required parameters

    println!("Languages: {:?}", languages);

    let mut elements = Vec::new();
    if let Ok(reader) = read_file(input_file) {
        let mut buf = String::new();
        for b in reader.bytes() {
            if let Ok(bite) = b {
                if bite == 0xA {
                    let end = if buf.ends_with(",") {
                        buf.len() - 1
                    } else {
                        buf.len()
                    };
                    match parse_item(&buf[0..end], &languages) {
                        Ok(elem) => {
                            if let Some(el) = elem {
                                elements.push(el);
                            }
                        },
                        Err(_) => {
                        }
                    }
                    buf.clear();
                } else{
                    if let Ok(bb) = std::str::from_utf8(&[bite]) {
                        buf.push_str(bb);
                    }
                }
            }
        }
    }
    let mut same = 0;
    let mut different = 0;
    for e in elements {
        if e[languages[0]] == e[languages[1]] {
            same += 1
        } else {
            different += 1
        }
    }

    let t1 = time::precise_time_ns();
    println!("Results - Same: {}, Different: {}", same, different);
    println!("Time: {} ms", (t1 - t0) / 1000000 )
}
