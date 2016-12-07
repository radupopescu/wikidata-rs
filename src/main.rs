extern crate wikidata;
extern crate time;

use time::precise_time_ns;

use wikidata::parse::parse_item;
use wikidata::read::Streamer;
use wikidata::param::{Parameters,read_params};

fn main() {
    // TODO: More fine grained time measurements on:
    //       - file streaming
    //       - JSON parsing
    //       - element production loop
    //       - element counting

    let t0 = precise_time_ns();

    let Parameters{input_file, languages} = read_params();
    println!("Input file: {}", input_file);
    println!("Languages: {:?}", languages);

    let mut elements = Vec::new();
    if let Ok(streamer) = Streamer::new(&input_file) {
        let t00 = precise_time_ns();
        for line in streamer {
            match parse_item(&line, &languages) {
                Ok(elem) => {
                    if let Some(el) = elem {
                        elements.push(el);
                    }
                },
                Err(_) => {
                }
            }
        }
        let t10 = precise_time_ns();
        println!("Main loop: {} us", (t10 - t00) / 1000 );
    }

    // TODO: Must do counting concurrently with producing elements
    let t000 = precise_time_ns();
    let mut same = 0;
    let mut different = 0;
    for e in elements {
        if e[&languages[0]] == e[&languages[1]] {
            same += 1
        } else {
            different += 1
        }
    }
    let t100 = precise_time_ns();
    println!("Counting: {} us", (t100 - t000) / 1000 );

    let t1 = precise_time_ns();
    println!("Results - Same: {}, Different: {}", same, different);
    println!("Time: {} us", (t1 - t0) / 1000 )
}
