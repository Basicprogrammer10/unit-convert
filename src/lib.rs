#![feature(iter_intersperse)]
#![feature(decl_macro)]

pub mod args;
pub mod dimension;
pub mod input;
pub mod misc;
pub mod prefix;
#[cfg(test)]
mod test;
pub mod units;

type Num = f64;
