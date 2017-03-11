extern crate futures;
extern crate futures_cpupool;
extern crate time;
extern crate wikidata;

use std::thread;
use std::sync::Arc;
use std::sync::mpsc::{channel};

use futures::Future;
use futures_cpupool::CpuPool;
use time::precise_time_ns;

use wikidata::parse::parse_item;
use wikidata::read::Streamer;
use wikidata::param::{Parameters, read_params};

fn main() {
    let t0 = precise_time_ns();

    let Parameters { input_file, languages, threads } = read_params();
    println!("Input file: {}", input_file);
    println!("Languages: {:?}", languages);
    println!("Number of threads: {}", threads);

    let pool = CpuPool::new(threads);
    let (tx, rx) = channel();
    if let Ok(streamer) = Streamer::new(&input_file) {
        let langs = Arc::new(languages.clone()); // clone of langs to be moved into the other thread
        thread::spawn(move || {
            let tx = tx.clone();
            let t00 = precise_time_ns();
            for line in streamer {
                let l = line.clone();
                let langs = langs.clone(); // a new clone of langs for each future. Inefficient
                let _ = tx.send(pool.spawn_fn(move || parse_item(&l, &langs)));
            }
            let t10 = precise_time_ns();
            println!("Main loop: {} us", (t10 - t00) / 1000);
        });
    }

    let t000 = precise_time_ns();
    let mut same = 0;
    let mut different = 0;

    for v in rx {
        if let Some(v) = v.wait().ok().and_then(|v| v) {
            if v[&languages[0]] == v[&languages[1]] {
                same += 1
            } else {
                different += 1
            };
        }
    }

    let t100 = precise_time_ns();
    println!("Counting: {} us", (t100 - t000) / 1000);

    let t1 = precise_time_ns();
    println!("Results - Same: {}, Different: {}", same, different);
    println!("Time: {} us", (t1 - t0) / 1000)
}
