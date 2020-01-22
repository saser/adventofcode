use std::io;

use crate::base::Part;

pub fn part1(r: &mut dyn io::Read) -> Result<String, String> {
    solve(r, Part::One)
}

pub fn part2(r: &mut dyn io::Read) -> Result<String, String> {
    solve(r, Part::Two)
}

fn solve(r: &mut dyn io::Read, part: Part) -> Result<String, String> {
    Err("not implemented yet".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    mod part1 {
        use super::*;

        test!(example, file "testdata/day18/ex", "4", part1);
        test!(actual, file "../../../inputs/2017/18", "3188", part1);
    }

    // mod part2 {
    //     use super::*;

    //     test!(example, "", "", part2);
    //     test!(actual, file "../../../inputs/2017/18", "", part2);
    // }
}
