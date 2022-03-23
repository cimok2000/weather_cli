use std::{fs::File, io::Write};

use clap::{Arg, ArgMatches, Command};

const CONFIG_FILE: &str = "config.json";

fn main() {
    let matches = Command::new("Weather CLI")
        .version("0.1.0")
        .author("Connor O'Keefe <cimok2000@gmail.com>")
        .about("Pulls weather data from <replace with API name>")
        .arg(
            Arg::new("setup")
                .short('s')
                .long("setup")
                .takes_value(false)
                .help("Setup the CLI tool."),
        )
        .arg(
            Arg::new("num")
                .short('n')
                .long("number")
                .takes_value(true)
                .help("A number"),
        )
        .get_matches();

    // let input_file = matches.value_of("file").unwrap_or("input.txt");
    // println!("The file passed is: {}", input_file);
    parse_args(matches);
}

fn parse_args(matches: ArgMatches) {
    if matches.occurrences_of("setup") == 1 {
        println!("Setup Project");
        setup_project();
    }
}

fn setup_project() {
    let config: &str = "{}";

    let mut output = File::create(CONFIG_FILE).unwrap();
    output.write(b"{:?}", config);
}
