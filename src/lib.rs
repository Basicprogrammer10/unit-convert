#![feature(iter_intersperse)]
#![feature(decl_macro)]

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod args;
pub mod dimension;
pub mod input;
pub mod misc;
pub mod prefix;
pub mod shorthand;
#[cfg(test)]
mod test;
pub mod units;

type Num = f64;
