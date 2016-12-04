extern crate wikidata;
use wikidata::errors::*;

#[macro_use]
extern crate clap;

extern crate flate2;

use std::collections::HashMap;

extern crate serde;
extern crate serde_json;

use std::fs;
use std::io;
use std::io::Read;

use std::str;

struct WikiElement {
    id: String,
    sites: HashMap<String, String>,
}

fn parse_item(line: &str, languages: &Vec<&str>) -> Result<Option<WikiElement>,
                                                           WikiError> {
    let item: serde_json::value::Value = serde_json::from_str(line)?;

    let mut sites = HashMap::new();

    let wiki_elem = if let Some(elem) = item.find("id") {
        if let Some(i) = elem.as_str() {
            let id = i.to_string();
            if let Some(sitelinks) = item.find("sitelinks") {
                for l in languages {
                    let link = format!("{}wiki", l);
                    match sitelinks.find(&link) {
                        Some(res) => {
                            if let Some(title) = res.find("title") {
                                sites.insert(l.to_string(), title.to_string());
                            }
                        },
                        None => (),
                    }
                }
                if sites.len() == languages.len() {
                    Some(WikiElement{id : id, sites : sites})
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };
    Ok(wiki_elem)
}

fn read_file(input_file: &str) -> Result<flate2::bufread::GzDecoder<io::BufReader<fs::File>>, WikiError> {
    let f = fs::File::open(input_file)?;
    let bf = io::BufReader::new(f);
    let rdr = flate2::bufread::GzDecoder::new(bf)?;
    Ok(rdr)
}

fn main() {
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
                        Err(err) => {
                            println!("Error parsing line: {}", err);
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
        println!("id: {}, sites: {:?}", e.id, e.sites);
        if e.sites[languages[0]] == e.sites[languages[1]] {
            same += 1
        } else {
            different += 1
        }
    }

    println!("Results - Same: {}, Different: {}", same, different);
}
