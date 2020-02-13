use clap;

use std::fs;
use std::io;
use std::process;
use std::time;

fn imain() -> i32 {
    // Define the application and its arguments.
    let app = clap::App::new("aoc")
        .version("0.1.0")
        .about("Runs solutions for the Advent of Code programming problems.")
        .arg(
            clap::Arg::with_name("year")
                .help("specifies year")
                .takes_value(true)
                .required(true)
                .possible_values(&["2015", "2016", "2017", "2018", "2019"]),
        )
        .arg(
            clap::Arg::with_name("day")
                .help("specifies day")
                .takes_value(true)
                .required(true)
                .possible_values(&[
                    "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25",
                ]),
        )
        .arg(
            clap::Arg::with_name("part")
                .help("specifies part")
                .takes_value(true)
                .required(true)
                .possible_values(&["1", "2"]),
        )
        .arg(
            clap::Arg::with_name("input")
                .short("i")
                .long("input")
                .help("specifies input file (if not specified, stdin will be used)")
                .takes_value(true)
                .required(false),
        );
    // Parse arguments and convert them to proper values.
    let matches = app.get_matches();
    let year = clap::value_t!(matches.value_of("year"), i32).unwrap();
    let day = clap::value_t!(matches.value_of("day"), i32).unwrap();
    let part = clap::value_t!(matches.value_of("part"), i32).unwrap();
    let mut r: Box<dyn io::Read> = match matches.value_of("input") {
        None => Box::new(io::stdin()),
        Some(path) => match fs::File::open(path) {
            Ok(f) => Box::new(f),
            Err(e) => {
                eprintln!("error opening input file {}: {}", path, e);
                return 1;
            }
        },
    };
    // Choose solution function based on arguments.
    let solution: Result<aoc::Solution, String> = match (year, day, part) {
        // Year 2017.
        (2017, 1, 1) => Ok(aoc::year2017::day01::part1),
        (2017, 1, 2) => Ok(aoc::year2017::day01::part2),
        (2017, 2, 1) => Ok(aoc::year2017::day02::part1),
        (2017, 2, 2) => Ok(aoc::year2017::day02::part2),
        (2017, 3, 1) => Ok(aoc::year2017::day03::part1),
        (2017, 3, 2) => Ok(aoc::year2017::day03::part2),
        (2017, 4, 1) => Ok(aoc::year2017::day04::part1),
        (2017, 4, 2) => Ok(aoc::year2017::day04::part2),
        (2017, 5, 1) => Ok(aoc::year2017::day05::part1),
        (2017, 5, 2) => Ok(aoc::year2017::day05::part2),
        (2017, 6, 1) => Ok(aoc::year2017::day06::part1),
        (2017, 6, 2) => Ok(aoc::year2017::day06::part2),
        (2017, 7, 1) => Ok(aoc::year2017::day07::part1),
        (2017, 7, 2) => Ok(aoc::year2017::day07::part2),
        (2017, 8, 1) => Ok(aoc::year2017::day08::part1),
        (2017, 8, 2) => Ok(aoc::year2017::day08::part2),
        (2017, 9, 1) => Ok(aoc::year2017::day09::part1),
        (2017, 9, 2) => Ok(aoc::year2017::day09::part2),
        (2017, 10, 1) => Ok(aoc::year2017::day10::part1),
        (2017, 10, 2) => Ok(aoc::year2017::day10::part2),
        (2017, 11, 1) => Ok(aoc::year2017::day11::part1),
        (2017, 11, 2) => Ok(aoc::year2017::day11::part2),
        (2017, 12, 1) => Ok(aoc::year2017::day12::part1),
        (2017, 12, 2) => Ok(aoc::year2017::day12::part2),
        (2017, 13, 1) => Ok(aoc::year2017::day13::part1),
        (2017, 13, 2) => Ok(aoc::year2017::day13::part2),
        (2017, 14, 1) => Ok(aoc::year2017::day14::part1),
        (2017, 14, 2) => Ok(aoc::year2017::day14::part2),
        (2017, 15, 1) => Ok(aoc::year2017::day15::part1),
        (2017, 15, 2) => Ok(aoc::year2017::day15::part2),
        (2017, 16, 1) => Ok(aoc::year2017::day16::part1),
        (2017, 16, 2) => Ok(aoc::year2017::day16::part2),
        (2017, 17, 1) => Ok(aoc::year2017::day17::part1),
        (2017, 17, 2) => Ok(aoc::year2017::day17::part2),
        (2017, 18, 1) => Ok(aoc::year2017::day18::part1),
        (2017, 18, 2) => Ok(aoc::year2017::day18::part2),
        (2017, 19, 1) => Ok(aoc::year2017::day19::part1),
        (2017, 19, 2) => Ok(aoc::year2017::day19::part2),
        // Year 2018.
        (2018, 1, 1) => Ok(aoc::year2018::day01::part1),
        (2018, 1, 2) => Ok(aoc::year2018::day01::part2),
        (2018, 2, 1) => Ok(aoc::year2018::day02::part1),
        (2018, 2, 2) => Ok(aoc::year2018::day02::part2),
        (2018, 3, 1) => Ok(aoc::year2018::day03::part1),
        (2018, 3, 2) => Ok(aoc::year2018::day03::part2),
        (2018, 4, 1) => Ok(aoc::year2018::day04::part1),
        (2018, 4, 2) => Ok(aoc::year2018::day04::part2),
        (2018, 5, 1) => Ok(aoc::year2018::day05::part1),
        (2018, 5, 2) => Ok(aoc::year2018::day05::part2),
        (2018, 6, 1) => Ok(aoc::year2018::day06::part1),
        (2018, 6, 2) => Ok(aoc::year2018::day06::part2),
        (2018, 7, 1) => Ok(aoc::year2018::day07::part1),
        (2018, 7, 2) => Ok(aoc::year2018::day07::part2),
        (2018, 8, 1) => Ok(aoc::year2018::day08::part1),
        (2018, 8, 2) => Ok(aoc::year2018::day08::part2),
        (2018, 9, 1) => Ok(aoc::year2018::day09::part1),
        (2018, 9, 2) => Ok(aoc::year2018::day09::part2),
        (2018, 10, 1) => Ok(aoc::year2018::day10::part1),
        (2018, 10, 2) => Ok(aoc::year2018::day10::part2),
        (2018, 11, 1) => Ok(aoc::year2018::day11::part1),
        (2018, 11, 2) => Ok(aoc::year2018::day11::part2),
        (2018, 12, 1) => Ok(aoc::year2018::day12::part1),
        (2018, 12, 2) => Ok(aoc::year2018::day12::part2),
        (2018, 13, 1) => Ok(aoc::year2018::day13::part1),
        (2018, 13, 2) => Ok(aoc::year2018::day13::part2),
        (2018, 14, 1) => Ok(aoc::year2018::day14::part1),
        (2018, 14, 2) => Ok(aoc::year2018::day14::part2),
        (2018, 15, 1) => Ok(aoc::year2018::day15::part1),
        (2018, 15, 2) => Ok(aoc::year2018::day15::part2),
        (2018, 16, 1) => Ok(aoc::year2018::day16::part1),
        (2018, 16, 2) => Ok(aoc::year2018::day16::part2),
        (2018, 17, 1) => Ok(aoc::year2018::day17::part1),
        (2018, 17, 2) => Ok(aoc::year2018::day17::part2),
        (2018, 18, 1) => Ok(aoc::year2018::day18::part1),
        (2018, 18, 2) => Ok(aoc::year2018::day18::part2),
        (2018, 19, 1) => Ok(aoc::year2018::day19::part1),
        (2018, 19, 2) => Ok(aoc::year2018::day19::part2),
        (2018, 20, 1) => Ok(aoc::year2018::day20::part1),
        (2018, 20, 2) => Ok(aoc::year2018::day20::part2),
        _ => Err(format!(
            "no solution for year {} day {} part {}",
            year, day, part
        )),
    };
    if let Err(e) = solution {
        eprintln!("error finding solution: {}", e);
        return 1;
    }
    // Run and time solution.
    let timer = time::Instant::now();
    match solution.unwrap()(&mut r) {
        Ok(answer) => {
            let elapsed = timer.elapsed();
            println!("{}", answer);
            eprintln!(
                "{} ms ({} ns)",
                elapsed.as_nanos() as f64 / 1e+6,
                elapsed.as_nanos()
            );
        }
        Err(err) => {
            eprintln!("error in solution: {}", err);
            return 2;
        }
    };
    0
}

fn main() {
    process::exit(imain());
}
