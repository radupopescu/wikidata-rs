pub struct Parameters {
    pub input_file: String,
    pub languages: Vec<String>,
}

pub fn read_params() -> Parameters {
    let matches = clap_app!(myapp =>
                            (version: "0.1.0")
                            (author: "Radu Popescu <mail@radupopescu.net>")
                            (about: "Process the Wikidata JSON dump with Rust")
                            (@arg FILE: -f --file +takes_value +required "Wikidump data file")
                            (@arg LANGUAGES: -l --languages +takes_value +required +multiple "List of languages to use")
    ).get_matches();

    let input = matches.value_of("FILE").unwrap().to_string(); // safe <= required parameter

    let langs = matches
        .values_of("LANGUAGES")
        .unwrap()
        .collect::<Vec<_>>(); // safe <= required parameters
    let languages = langs.iter().map(|l| l.to_string()).collect();

    Parameters {input_file: input, languages: languages}
}
