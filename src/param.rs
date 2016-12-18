use clap::{App,Arg};

pub struct Parameters {
    pub input_file: String,
    pub languages: Vec<String>,
    pub threads: usize,
}

pub fn read_params() -> Parameters {
    let matches = App::new("wikidata-rs")
        .version("0.1.0")
        .author("Radu Popescu <mail@radupopescu.net>")
        .about("Process the Wikidata JSON dump with Rust")
        .arg(Arg::with_name("file")
             .short("f")
             .long("file")
             .takes_value(true)
             .value_name("FILE")
             .required(true)
             .help("Wikidump data file"))
        .arg(Arg::with_name("languages")
             .short("l")
             .long("languages")
             .takes_value(true)
             .value_name("LANGUAGES")
             .required(true)
             .multiple(true)
             .help("List of languages to use"))
        .arg(Arg::with_name("threads")
             .short("t")
             .long("threads")
             .takes_value(true)
             .value_name("THREADS")
             .default_value("1")
             .help("Number of worker threads"))
        .get_matches();

    let input = matches.value_of("file").unwrap().to_string(); // safe <= required parameter

    let langs = matches
        .values_of("languages")
        .unwrap()
        .collect::<Vec<_>>(); // safe <= required parameters
    let languages = langs.iter().map(|l| l.to_string()).collect();

    let threads = matches.value_of("threads").unwrap().parse::<usize>().unwrap();

    Parameters {input_file: input, languages: languages, threads: threads}
}
