extern crate base;

use base::{Part, Solver};

struct Day02;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day02)
}

impl Solver for Day02 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        Err("not implemented yet".to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
