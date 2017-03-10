extern crate futures;
extern crate futures_cpupool;
extern crate time;
extern crate wikidata;

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
    let mut futs = Vec::new();
    if let Ok(streamer) = Streamer::new(&input_file) {
        let t00 = precise_time_ns();
        for line in streamer {
            let l = line.clone();
            let langs = languages.clone();
            futs.push(pool.spawn_fn(move || parse_item(&l, &langs)));
        }
        let t10 = precise_time_ns();
        println!("Main loop: {} us", (t10 - t00) / 1000);
    }

    let t000 = precise_time_ns();
    let mut same = 0;
    let mut different = 0;
    for f in futs {
        f.wait().ok().and_then(|v| v).map(|v| {
            if v[&languages[0]] == v[&languages[1]] {
                same += 1
            } else {
                different += 1
            };
        });
    }
    let t100 = precise_time_ns();
    println!("Counting: {} us", (t100 - t000) / 1000);

    let t1 = precise_time_ns();
    println!("Results - Same: {}, Different: {}", same, different);
    println!("Time: {} us", (t1 - t0) / 1000)
}
