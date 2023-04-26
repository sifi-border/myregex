//! regular expression engine

use crate::helper::DynError;
use std::fmt::Display;

mod codegen;
mod evaluator;
mod parser;

#[derive(Debug)]
pub enum Instruction {
    Char(char),
    Match,
    Jump(usize),
    Split(usize, usize),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Char(c) => write!(f, "char {}", c),
            Instruction::Match => write!(f, "match"),
            Instruction::Jump(addr) => write!(f, "jump {:>04}", addr),
            Instruction::Split(addr1, addr2) => write!(f, "split {:>04}, {:>04}", addr1, addr2),
        }
    }
}

/// match a regular expression with a string.
///
/// # example usage
///
/// ```
/// use regex;
/// regex::do_matching("abc|(de|cd)+", "decddede", true);
/// ```
///
/// # arguments
///
/// expr is the regular expression, line is the string to match.
/// if is_depth is true, depth-first search is used; if false, width-first search is used.
///
/// # return value
///
/// returns Ok(true) if executed without error and matching is **successful**,
/// returns Ok(false) if executed without error and matching **fails**.
///
/// returns Err if there is an error in the input regular expression or an internal implementation error.
pub fn do_matching(expr: &str, line: &str, use_dfs: bool) -> Result<bool, DynError> {
    let ast = parser::parse(expr)?;
    let code = codegen::get_code(&ast)?;
    let line = line.chars().collect::<Vec<char>>();
    Ok(evaluator::eval(&code, &line, use_dfs)?)
}
