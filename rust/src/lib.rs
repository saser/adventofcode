use std::io;

pub mod base;
pub mod year2016;
pub mod year2017;
pub mod year2018;

pub type Solution = fn(r: &mut dyn io::Read) -> Result<String, String>;
