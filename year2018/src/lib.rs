extern crate base;
extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate rayon;
extern crate regex;

use base::{Solver, YearDispatcher};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

pub struct Year2018;

pub fn get_dispatcher() -> Box<dyn YearDispatcher> {
    Box::new(Year2018)
}

impl YearDispatcher for Year2018 {
    fn get_solver(&self, day: u8) -> Result<Box<dyn Solver>, String> {
        match day {
            01 => Ok(day01::get_solver()),
            02 => Ok(day02::get_solver()),
            03 => Ok(day03::get_solver()),
            04 => Ok(day04::get_solver()),
            05 => Ok(day05::get_solver()),
            _ => Err(format!("no solver for day {}", day)),
        }
    }
}
