use std::fmt;
use crate::ast::ARRAY_LEN;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    PtrBelowZero,
    PtrAboveLimit,
    NonClosedBrackets,
    NonClosedEnvs,
    NestedEnv,
    InfiniteLoop
}

impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            PtrBelowZero => write!(f, "error: mem pointer went below zero."),
            PtrAboveLimit => write!(f, "error: mem pointer went above limit {}", ARRAY_LEN),
            NonClosedBrackets => write!(f, "error: some brackets are unclosed on source code"),
            NonClosedEnvs => write!(f, "error: some environments (parentheses) are unclosed in your source code"),
            NestedEnv => write!(f, "error: environment nesting is not allowed"),
            InfiniteLoop => write!(f, "error: potential infinite loop in source code")
        }
    }
}