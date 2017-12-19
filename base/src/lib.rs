use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Part {
    One,
    Two,
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Part::One),
            "2" => Ok(Part::Two),
            _ => Err(format!("part must be 1 or 2; was: {}", s)),
        }
    }
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Part::One => "1",
            Part::Two => "2",
        };
        write!(f, "{}", s)
    }
}

pub trait Solver {
    fn solve(&self, part: Part, input: &str) -> Result<String, String>;
}
