//! receives instruction string and input string and executes matching
use super::Instruction;
use crate::helper::safe_add;
use std::{
    collections::VecDeque,
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum EvalError {
    PCOverFlow,
    SPOverFlow,
    InvalidPC,
}

impl Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CodeGenError: {:?}", self)
    }
}

impl Error for EvalError {}

/// match by DFS
fn eval_depth(
    inst: &[Instruction],
    line: &[char],
    mut pc: usize,
    mut sp: usize,
) -> Result<bool, EvalError> {
    loop {
        let next_i = if let Some(i) = inst.get(pc) {
            i
        } else {
            return Err(EvalError::InvalidPC);
        };

        match next_i {
            Instruction::Char(c) => {
                if let Some(sp_c) = line.get(sp) {
                    if sp_c == c {
                        safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
                        safe_add(&mut sp, &1, || EvalError::SPOverFlow)?;
                    } else {
                        return Ok(false);
                    }
                } else {
                    return Ok(false);
                }
            }
            Instruction::Jump(addr) => {
                pc = *addr;
            }
            Instruction::Match => {
                return Ok(true);
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

/// match by bfs
fn eval_width(inst: &[Instruction], line: &[char]) -> Result<bool, EvalError> {
    let mut ctx_dq = VecDeque::new();

    let mut pc = 0usize;
    let mut sp = 0usize;
    ctx_dq.push_back((pc, sp));

    loop {
        if let Some((p, s)) = ctx_dq.pop_front() {
            pc = p;
            sp = s;
        } else {
            return Ok(false);
        }
        let next_i = if let Some(i) = inst.get(pc) {
            i
        } else {
            return Err(EvalError::InvalidPC);
        };

        match next_i {
            Instruction::Char(c) => {
                if let Some(sp_c) = line.get(sp) {
                    if sp_c == c {
                        safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
                        safe_add(&mut sp, &1, || EvalError::SPOverFlow)?;
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            Instruction::Jump(addr) => {
                pc = *addr;
            }
            Instruction::Match => {
                return Ok(true);
            }
            Instruction::Split(addr1, addr2) => {
                ctx_dq.push_back((*addr1, sp));
                pc = *addr2;
            }
        }

        ctx_dq.push_back((pc, sp));
    }
}

/// function to evaluate a sequence of instructions.
///
/// inst becomes an instruction string, and matches the input string line using that instruction string.
/// if is_depth is true, depth-first search is performed; if is_depth is false, width-first search is performed.
///
/// returns Err if a runtime error occurs.
/// returns Ok(true) if the match succeeds, Ok(false) if it fails.
pub fn eval(inst: &[Instruction], line: &[char], use_dfs: bool) -> Result<bool, EvalError> {
    if use_dfs {
        eval_depth(inst, line, 0, 0)
    } else {
        eval_width(inst, line)
    }
}
