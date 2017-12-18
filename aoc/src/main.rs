extern crate base;
extern crate clap;

use base::Part;
use clap::{App, Arg, ArgMatches};

fn main() {
    let app = App::new("adventofcode2017")
        .version("0.1.0")
        .author("Christian Persson <saser@live.se>")
        .about("Runs solutions for the Advent of Code 2017 programming problems")
        .arg(Arg::with_name("day")
            .help("Specifies which day (1-25) to run")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("part")
            .help("Specifies which part of the problem to run (both if unspecified)")
            .takes_value(true)
            .required(false)
            .possible_values(&["1", "2"]));
    let matches = app.get_matches();

    let day = parse_day(&matches).unwrap_or_else(|msg| {
        println!("{}", msg);
        std::process::exit(1);
    });
    let parts = parse_part(&matches);

    println!("Day: {}", day);
    println!("Part(s): {:?}", parts);
}

fn parse_day(matches: &ArgMatches) -> Result<u8, String> {
    let day = matches.value_of("day").unwrap();
    let day = day.parse::<u8>().unwrap();
    match day {
        1...25 => Ok(day),
        _ => Err(format!("invalid day: {}", day)),
    }
}

fn parse_part(matches: &ArgMatches) -> Vec<Part> {
    let part = matches.value_of("part");
    match part {
        Some(p) => {
            vec![match p {
                     "1" => Part::One,
                     "2" => Part::Two,
                     _ => unreachable!(),
                 }]
        }
        None => vec![Part::One, Part::Two],
    }
}
