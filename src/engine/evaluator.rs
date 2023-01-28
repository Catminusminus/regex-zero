use super::Instruction;
use crate::helper::safe_add;
use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum EvalError {
    PCOverflow,
    SPOverflow,
    InvalidPC,
    InvalidContext,
}

impl Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CodeGenError: {:?}", self)
    }
}

impl Error for EvalError {}

fn eval_depth(
    inst: &[Instruction],
    line: &[char],
    mut pc: usize,
    mut sp: usize,
) -> Result<bool, EvalError> {
    loop {
        let next = if let Some(i) = inst.get(pc) {
            i
        } else {
            return Err(EvalError::InvalidPC);
        };
        match next {
            Instruction::Char(c) => {
                if let Some(sp_c) = line.get(sp) {
                    if c == sp_c {
                        safe_add(&mut pc, &1, || EvalError::PCOverflow)?;
                        safe_add(&mut sp, &1, || EvalError::SPOverflow)?;
                    } else {
                        return Ok(false);
                    }
                } else {
                    return Ok(false);
                }
            }
            Instruction::Match => {
                return Ok(true);
            }
            Instruction::Jump(addr) => {
                pc = *addr;
            }
            Instruction::Split(addr1, addr2) => {
                if eval_depth(inst, line, *addr1, sp)? || eval_depth(inst, line, *addr2, sp)? {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            }
        }
    }
}

fn eval_depth_with_cache(
    inst: &[Instruction],
    line: &[char],
    mut pc: usize,
    mut sp: usize,
    cache: &mut HashSet<(usize, usize)>,
) -> Result<bool, EvalError> {
    loop {
        let next = if let Some(i) = inst.get(pc) {
            i
        } else {
            return Err(EvalError::InvalidPC);
        };
        match next {
            Instruction::Char(c) => {
                if let Some(sp_c) = line.get(sp) {
                    if c == sp_c {
                        safe_add(&mut pc, &1, || EvalError::PCOverflow)?;
                        safe_add(&mut sp, &1, || EvalError::SPOverflow)?;
                    } else {
                        return Ok(false);
                    }
                } else {
                    return Ok(false);
                }
            }
            Instruction::Match => {
                return Ok(true);
            }
            Instruction::Jump(addr) => {
                if cache.contains(&(*addr, sp)) {
                    return Ok(false);
                }
                cache.insert((*addr, sp));
                pc = *addr;
            }
            Instruction::Split(addr1, addr2) => {
                if eval_depth_with_cache(inst, line, *addr1, sp, cache)?
                    || eval_depth_with_cache(inst, line, *addr2, sp, cache)?
                {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            }
        }
    }
}

/// Evaluate insts seq
///
/// inst: insts seq, line: input string, is_depth: depth first search or not
///
/// returns Err if some errors occur
/// returns Ok(true) if the matching is succeeded, returns Ok(false) if not.
pub fn eval_with_cache(
    inst: &[Instruction],
    line: &[char],
    is_depth: bool,
) -> Result<bool, EvalError> {
    let mut cache = HashSet::new();
    if is_depth {
        eval_depth_with_cache(inst, line, 0, 0, &mut cache)
    } else {
        eval_depth_with_cache(inst, line, 0, 0, &mut cache)
    }
}

pub fn eval(inst: &[Instruction], line: &[char], is_depth: bool) -> Result<bool, EvalError> {
    if is_depth {
        eval_depth(inst, line, 0, 0)
    } else {
        eval_depth(inst, line, 0, 0)
    }
}
