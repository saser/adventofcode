use crate::base::{Part, Solver};

pub fn get_solver() -> Box<dyn Solver> {
    Box::new(Day20)
}

struct Day20;

impl Solver for Day20 {
    fn solve(&self, _part: Part, _input: &str) -> Result<String, String> {
        Err("day 20 not yet implemented".to_string())
    }
}
