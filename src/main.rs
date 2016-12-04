extern crate wikidata;
use wikidata::parse::parse_item;
use wikidata::read::Streamer;
use wikidata::param::{Parameters,read_params};

extern crate time;

fn main() {
    // TODO: More fine grained time measurements on:
    //       - file streaming
    //       - JSON parsing
    //       - element production loop
    //       - element counting

    let t0 = time::precise_time_ns();

    let Parameters{input_file, languages} = read_params();
    println!("Input file: {}", input_file);
    println!("Languages: {:?}", languages);

    let mut elements = Vec::new();
    if let Ok(streamer) = Streamer::new(&input_file) {
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
    }

    // TODO: Must do counting concurrently with producing elements
    let mut same = 0;
    let mut different = 0;
    for e in elements {
        if e[&languages[0]] == e[&languages[1]] {
            same += 1
        } else {
            different += 1
        }
    }

    let t1 = time::precise_time_ns();
    println!("Results - Same: {}, Different: {}", same, different);
    println!("Time: {} ms", (t1 - t0) / 1000000 )
}
