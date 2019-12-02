use std::io;

pub mod base;
pub mod year2016;
pub mod year2017;
pub mod year2018;

pub type Solution = fn(r: &mut dyn io::Read) -> Result<String, String>;

#[macro_export]
macro_rules! test {
    ($name:ident, $input:expr, $output:expr, $solution:expr) => {
        #[test]
        fn $name() {
            let input: &str = $input;
            let output: &str = $output;
            let solution: crate::Solution = $solution;
            assert_eq!(output, solution(&mut Box::new(input.as_bytes())).unwrap());
        }
    };
}
