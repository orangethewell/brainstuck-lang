#![allow(dead_code, non_snake_case)]
pub mod instruction;
pub mod error_handler;
pub mod parser;

pub use parser::parse_source;

pub const ARRAY_LEN: usize = 30_000;
pub const NEWLINE: u8 = 10;
pub const EOF: u8 = 0;