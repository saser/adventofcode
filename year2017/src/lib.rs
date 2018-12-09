#[macro_use]
extern crate lazy_static;

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
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;

pub struct Year2017;

pub fn get_dispatcher() -> Box<dyn YearDispatcher> {
    Box::new(Year2017)
}

impl YearDispatcher for Year2017 {
    fn get_solver(&self, day: u8) -> Result<Box<dyn Solver>, String> {
        match day {
            1 => Ok(day01::get_solver()),
            2 => Ok(day02::get_solver()),
            3 => Ok(day03::get_solver()),
            4 => Ok(day04::get_solver()),
            5 => Ok(day05::get_solver()),
            6 => Ok(day06::get_solver()),
            7 => Ok(day07::get_solver()),
            8 => Ok(day08::get_solver()),
            9 => Ok(day09::get_solver()),
            10 => Ok(day10::get_solver()),
            11 => Ok(day11::get_solver()),
            12 => Ok(day12::get_solver()),
            13 => Ok(day13::get_solver()),
            14 => Ok(day14::get_solver()),
            15 => Ok(day15::get_solver()),
            16 => Ok(day16::get_solver()),
            17 => Ok(day17::get_solver()),
            18 => Ok(day18::get_solver()),
            19 => Ok(day19::get_solver()),
            20 => Ok(day20::get_solver()),
            _ => Err(format!("no solver for day {}", day)),
        }
    }
}
