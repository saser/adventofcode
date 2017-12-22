extern crate base;
#[macro_use]
extern crate clap;
extern crate day01;
extern crate day02;

use base::{Part, Solver};
use clap::{App, Arg, ArgMatches};
use std::fs::File;
use std::io::{self, Read};
use std::process;
use std::time::{Duration, Instant};

static APP_NAME: &'static str = "aoc";
static APP_VERSION: &'static str = "0.1.0";
static APP_AUTHOR: &'static str = "Christian Persson <saser@live.se>";
static APP_ABOUT: &'static str = "Runs solutions for the Advent of Code 2017 programming problems";

macro_rules! eprintln {
    ($($arg:tt)*) => (
        use std::io::Write;
        let _ = writeln!(&mut ::std::io::stderr(), $($arg)* );
    )
}

fn main() {
    let app = create_app();
    let matches = app.get_matches();

    let (day, part, input_path) = parse_arguments(&matches).unwrap_or_else(|e| {
        eprintln!("Unable to parse arguments: {}", e);
        process::exit(1);
    });
    let input = read_input(&input_path).unwrap_or_else(|e| {
        eprintln!("Unable to read input file: {}", e);
        process::exit(1);
    });
    let solver = get_solver(day).unwrap_or_else(|e| {
        eprintln!("Unable to get solver: {}", e);
        process::exit(1);
    });

    solve(solver, day, part, &input);
}

fn solve(solver: Box<Solver>, day: u8, part: Part, input: &str) {
    let timer = Instant::now();
    let solution = solver.solve(part, input).unwrap_or_else(|e| {
        eprintln!("Unable to acquire solution for day {} part {}: {}",
                  day,
                  part,
                  e);
        process::exit(1);
    });
    let solution_time = timer.elapsed();
    println!("Solution for day {} part {}: {}", day, part, solution);
    println!("Time to solve: {}", format_duration(solution_time));
}

fn format_duration(duration: Duration) -> String {
    let total_ns: u64 = duration.as_secs() * 1_000_000_000 + duration.subsec_nanos() as u64;
    let total_ms: f64 = total_ns as f64 / 1e+6;
    format!("{:.3} ms ({} ns)", total_ms, total_ns)
}

fn create_app() -> App<'static, 'static> {
    App::new(APP_NAME)
        .version(APP_VERSION)
        .author(APP_AUTHOR)
        .about(APP_ABOUT)
        .arg(Arg::with_name("day")
            .help("Specifies which day (1-25) to run")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("part")
            .help("Specifies which part of the problem to run")
            .takes_value(true)
            .required(true)
            .possible_values(&["1", "2"]))
        .arg(Arg::with_name("input_file")
            .help("Path to file containing input to problem")
            .takes_value(true)
            .required(true))
}

fn parse_arguments(matches: &ArgMatches) -> Result<(u8, Part, String), String> {
    let day = value_t!(matches.value_of("day"), u8).unwrap();
    if day < 1 || day > 25 {
        return Err("day must be 1-25 (inclusive)".to_string());
    }
    let part = value_t!(matches.value_of("part"), Part).unwrap();
    let input_path = matches.value_of("input_file").unwrap().to_string();
    Ok((day, part, input_path))
}

fn read_input(path: &str) -> io::Result<String> {
    let mut input = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut input)?;
    Ok(input)
}

fn get_solver(day: u8) -> Result<Box<Solver>, String> {
    match day {
        1 => Ok(day01::get_solver()),
        2 => Ok(day02::get_solver()),
        _ => Err(format!("no solver for day {}", day)),
    }
}
