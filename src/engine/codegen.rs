use super::{parser::AST, Instruction};
use crate::helper::safe_add;

use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum CodeGenError {
    PCOverflow,
    FailStar,
    FailOr,
    FailQuestion,
}

impl Display for CodeGenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CodeGenError: {}", self)
    }
}

impl Error for CodeGenError {}

#[derive(Default, Debug)]
struct Generator {
    pc: usize,
    insts: Vec<Instruction>,
}

impl Generator {
    /// Increment the program counter
    fn inc_pc(&mut self) -> Result<(), CodeGenError> {
        safe_add(&mut self.pc, &1, || CodeGenError::PCOverflow)
    }
    /// Generate Code from AST
    fn gen_expr(&mut self, ast: &AST) -> Result<(), CodeGenError> {
        match ast {
            AST::Char(c) => self.gen_char(*c)?,
            AST::Or(e1, e2) => self.gen_or(e1, e2)?,
            AST::Plus(e) => self.gen_plus(e)?,
            AST::Star(e) => self.gen_star(e)?,
            AST::Question(e) => self.gen_question(e)?,
            AST::Seq(v) => self.gen_seq(v)?,
        }
        Ok(())
    }
    fn gen_seq(&mut self, exprs: &[AST]) -> Result<(), CodeGenError> {
        for e in exprs {
            self.gen_expr(e)?;
        }
        Ok(())
    }
    fn gen_char(&mut self, c: char) -> Result<(), CodeGenError> {
        let inst = Instruction::Char(c);
        self.insts.push(inst);
        self.inc_pc()?;
        Ok(())
    }
    /// Code Generator for OR operator
    ///
    /// Generate code like below:
    ///
    /// ```text
    ///     split L1, L2
    /// L1: code of e1
    ///     jmp L3
    /// L2: code of e2
    /// L3:
    /// ```
    fn gen_or(&mut self, e1: &AST, e2: &AST) -> Result<(), CodeGenError> {
        // split L1, L2
        let split_addr = self.pc;
        self.inc_pc()?;
        let split = Instruction::Split(self.pc, 0); // L1 = self.pc. Assume that L2 = 0
        self.insts.push(split);

        // L1: code of e1
        self.gen_expr(e1)?;

        // jmp L3
        let jmp_addr = self.pc;
        self.insts.push(Instruction::Jump(0)); // Assume that L3 = 0

        self.inc_pc()?;
        if let Some(Instruction::Split(_, l2)) = self.insts.get_mut(split_addr) {
            *l2 = self.pc;
        } else {
            return Err(CodeGenError::FailOr);
        }

        // L2: code of e2
        self.gen_expr(e2)?;

        if let Some(Instruction::Jump(l3)) = self.insts.get_mut(jmp_addr) {
            *l3 = self.pc;
        } else {
            return Err(CodeGenError::FailOr);
        }
        Ok(())
    }
    /// Code Generator for ?
    ///
    /// Generate code like below:
    ///
    /// ```text
    ///     split L1, L2
    /// L1: code of e
    /// L2:
    /// ```
    fn gen_question(&mut self, e: &AST) -> Result<(), CodeGenError> {
        // split L1, L2
        let split_addr = self.pc;
        self.inc_pc()?;
        let split = Instruction::Split(self.pc, 0); // L1 = self.pc. Assume that L2 = 0
        self.insts.push(split);

        // L1: code of e
        self.gen_expr(e)?;

        if let Some(Instruction::Split(_, l2)) = self.insts.get_mut(split_addr) {
            *l2 = self.pc;
            Ok(())
        } else {
            Err(CodeGenError::FailQuestion)
        }
    }
    /// Code Generator for plus
    ///
    /// Generate code like below:
    ///
    /// ```text
    /// L1: code of e
    ///     split L1, L2
    /// L2:
    /// ```
    fn gen_plus(&mut self, e: &AST) -> Result<(), CodeGenError> {
        // L1: code of e
        let l1 = self.pc;
        self.gen_expr(e)?;

        // split L1, L2
        self.inc_pc()?;
        let split = Instruction::Split(l1, self.pc);
        self.insts.push(split);

        Ok(())
    }
    /// Code Generator for star
    ///
    /// Generate code like below:
    ///
    /// ```text
    /// L1: split L2, L3
    /// L2: code of e
    ///     jmp L1
    /// L3:
    /// ```
    fn gen_star(&mut self, e: &AST) -> Result<(), CodeGenError> {
        // L1: split L2, L3
        let l1 = self.pc;
        self.inc_pc()?;
        let split = Instruction::Split(self.pc, 0);
        self.insts.push(split);

        // L2: code of e
        self.gen_expr(e)?;

        // jmp L1
        self.inc_pc()?;
        self.insts.push(Instruction::Jump(l1));

        if let Some(Instruction::Split(_, l3)) = self.insts.get_mut(l1) {
            *l3 = self.pc;
            Ok(())
        } else {
            Err(CodeGenError::FailStar)
        }
    }
    fn gen_code(&mut self, ast: &AST) -> Result<(), CodeGenError> {
        self.gen_expr(ast)?;
        self.inc_pc()?;
        self.insts.push(Instruction::Match);
        Ok(())
    }
}

pub fn get_code(ast: &AST) -> Result<Vec<Instruction>, CodeGenError> {
    let mut generator = Generator::default();
    generator.gen_code(ast)?;
    Ok(generator.insts)
}
