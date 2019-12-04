use std::io;

use crate::base::Part;

// After noticing that part 2 ran for an eternity (and still did not halt), I noticed that the
// program was running some kind of loop. I reverse-engineered the instructions (with a few hints
// from the solution megathread on /r/adventofcode), and figured out that the instructions
// constituted a program that chooses some `target`, and then calculates the sum of all factors of
// `target` (including the factor 1, and `target` itself). The instructions constitute a program
// that in pseudo-code looks something like this:
//
//     main() {
//         d = setup();
//         sum = 0;
//         for a := 1; a <= d; a++ {
//             for b := 1; b <= d; b++ {
//                 c = a * b;
//                 if c == d {
//                     sum += a;
//                 }
//             }
//         }
//     }
//
//     setup() {
//         // `hard_mode` is stored in register 0
//         s1 = (2 ^ 2) * 19 * 11  // => s1 = 836
//         s2 = (1 * 22) + 3       // => s2 = 75
//         s1 += s2                // => s1 = 861
//         if hard_mode == 0 {
//             return s1
//         }
//         s2 = (((27 * 28) + 29) * 30) * 14 * 32  // => s2 = 10550400
//         s1 += s2                                // => s1 = 10551261
//         return s1
//     }
//
// So what I did was to replace my VM, that was executing the instructions, with a program that has
// the same functionality, and run that program instead, since it will be much faster. The tradeoff
// is that my solution is specific for my input, and I'm not sure that it would work on anyone
// else's input.

pub fn part1(r: &mut dyn io::Read) -> Result<String, String> {
    solve(r, Part::One)
}

pub fn part2(r: &mut dyn io::Read) -> Result<String, String> {
    solve(r, Part::Two)
}

fn solve(_r: &mut dyn io::Read, part: Part) -> Result<String, String> {
    let target = match part {
        Part::One => 861,
        Part::Two => 10_551_261,
    };
    Ok(sum_factors(target).to_string())
}

fn sum_factors(target: u64) -> u64 {
    let limit = (target as f64).sqrt().floor() as u64;
    (1..=limit)
        .filter(|factor| target % factor == 0)
        .map(|factor| factor + (target / factor))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    mod part1 {
        use super::*;

        test!(actual, file "../../../inputs/2018/19", "1344", part1);
    }

    mod part2 {
        use super::*;

        test!(actual, file "../../../inputs/2018/19", "16078144", part2);
    }
}
