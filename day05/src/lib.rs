extern crate base;

use base::{Part, Solver};

pub fn get_solver() -> Box<Solver> {
    Box::new(Day05)
}

struct Day05;

impl Solver for Day05 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        Err("no solutions for day 5 implemented yet".to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
