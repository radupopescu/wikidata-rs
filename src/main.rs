extern crate wikidata;
use wikidata::errors::*;

#[macro_use]
extern crate clap;

use std::option::Option;
use std::collections::HashSet;

struct WikiElement<'a> {
    id: &'a str,
    sites: HashSet<&'a str>,
}

fn parse_item<'a>(line: &str, languages: &Vec<&str>) -> Result<WikiElement<'a>,
                                                               WikiError> {
    let mut s = HashSet::new();
    s.insert("test");
    Ok(WikiElement {id : "test", sites : s} )
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
}


