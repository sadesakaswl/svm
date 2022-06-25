/*
Syntax:
opcode reg0 reg1 data
*/

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::instruction::Instruction;

pub(crate) fn compile_line(line: &str) -> u32 {
    Instruction::from(line).into()
}
pub(crate) fn compile_lines(lines: Vec<&str>) -> Vec<u32> {
    lines.iter().map(|v| compile_line(v)).collect()
}
pub(crate) fn compile_lines_par(lines: Vec<&str>) -> Vec<u32> {
    lines.par_iter().map(|v| compile_line(*v)).collect()
}
pub(crate) fn decompile_line(line: u32) -> String {
    String::from(Instruction::from(line))
}
