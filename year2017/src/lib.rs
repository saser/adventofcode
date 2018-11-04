extern crate base;

use base::{Solver, YearDispatcher};

pub mod day01;

pub struct Year2017;

pub fn get_dispatcher() -> Box<dyn YearDispatcher> {
    Box::new(Year2017)
}

impl YearDispatcher for Year2017 {
    fn get_solver(&self, day: u8) -> Result<Box<dyn Solver>, String> {
        match day {
            1 => Ok(day01::get_solver()),
            _ => Err(format!("no solver for day {}", day)),
        }
    }
}
