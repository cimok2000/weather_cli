use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Weather CLI")
        .version("0.1.0")
        .author("Connor O'Keefe <cimok2000@gmail.com>")
        .about("Pulls weather data from <replace with API name>")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .takes_value(true)
                .help("An input file."),
        )
        .arg(
            Arg::new("num")
                .short('n')
                .long("number")
                .takes_value(true)
                .help("A number"),
        )
        .get_matches();

    let input_file = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", input_file);
}
