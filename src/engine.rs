//! Regex Engine
mod codegen;
mod evaluator;
mod parser;

use crate::helper::DynError;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Instruction {
    Char(char),
    Match,
    Jump(usize),
    Split(usize, usize),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Char(c) => write!(f, "char {}", c),
            Instruction::Match => write!(f, "match"),
            Instruction::Jump(addr) => write!(f, "jump {:>04}", addr),
            Instruction::Split(addr1, addr2) => write!(f, "split {:>04}, {:>04}", addr1, addr2),
        }
    }
}

/// Match regex and strings
/// # Example
///
/// ```
/// use regex_zero;
/// regex_zero::do_matching("abc|(de|cd)+", "decddede", true);
/// ```
///
/// # Arguments
///
/// expr: regex, line: input string to match, is_depth: use depth first search
///
/// # Return Values
///
/// Ok(true) if there are no errors and succeeded to match
/// Ok(false) if there are no errors and failed to match
///
/// Err if there are some errors
pub fn do_matching(expr: &str, line: &str, is_depth: bool) -> Result<bool, DynError> {
    let ast = parser::parse(&expr)?;
    let code = codegen::get_code(&ast)?;
    let line = line.chars().collect::<Vec<char>>();
    Ok(evaluator::eval(&code, &line, is_depth)?)
}

pub fn do_matching_with_cache(expr: &str, line: &str, is_depth: bool) -> Result<bool, DynError> {
    let ast = parser::parse(&expr)?;
    let code = codegen::get_code(&ast)?;
    let line = line.chars().collect::<Vec<char>>();
    Ok(evaluator::eval_with_cache(&code, &line, is_depth)?)
}

pub fn print(expr: &str) -> Result<(), DynError> {
    println!("expr: {expr}");
    let ast = parser::parse(expr)?;
    println!("AST: {:?}", ast);

    println!();
    println!("code:");
    let code = codegen::get_code(&ast)?;
    for (n, c) in code.iter().enumerate() {
        println!("{:>04}: {c}", n);
    }
    Ok(())
}
