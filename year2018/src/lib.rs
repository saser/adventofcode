use base::{Solver, YearDispatcher};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;

pub struct Year2018;

pub fn get_dispatcher() -> Box<dyn YearDispatcher> {
    Box::new(Year2018)
}

impl YearDispatcher for Year2018 {
    #[allow(clippy::zero_prefixed_literal)]
    fn get_solver(&self, day: u8) -> Result<Box<dyn Solver>, String> {
        match day {
            01 => Ok(day01::get_solver()),
            02 => Ok(day02::get_solver()),
            03 => Ok(day03::get_solver()),
            04 => Ok(day04::get_solver()),
            05 => Ok(day05::get_solver()),
            06 => Ok(day06::get_solver()),
            07 => Ok(day07::get_solver()),
            08 => Ok(day08::get_solver()),
            09 => Ok(day09::get_solver()),
            10 => Ok(day10::get_solver()),
            11 => Ok(day11::get_solver()),
            _ => Err(format!("no solver for day {}", day)),
        }
    }
}
