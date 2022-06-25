use crate::{
    opcode,
    register::{self, Register},
};

pub(crate) struct Instruction {
    pub opcode: u8,
    pub regs: u8,
    pub data: u16,
}

impl Instruction {
    pub(crate) fn new(opcode: u8, regs: u8, data: u16) -> Self {
        Self { opcode, regs, data }
    }
    pub(crate) fn strip_registers(&self) -> (u8, u8) {
        (self.regs & 0b00001111, self.regs >> 4)
    }
}
impl From<Instruction> for u32 {
    fn from(val: Instruction) -> Self {
        ((val.data as u32) << 16) & ((val.regs as u32) << 8) & (val.opcode as u32)
    }
}
impl From<u32> for Instruction {
    fn from(val: u32) -> Self {
        Self {
            opcode: (val as u8),
            regs: ((val >> 8) as u8),
            data: ((val >> 16) as u16),
        }
    }
}
impl From<&str> for Instruction {
    fn from(val: &str) -> Self {
        let mut opcode = 0u8;
        let mut reg0 = 0u8;
        let mut reg1 = 0u8;
        let mut data = 0u16;
        for (k, v) in val.to_string().split(' ').enumerate() {
            if k == 0 {
                opcode = opcode::Opcode::from(v).into();
            } else if k == 1 {
                reg0 = register::Register::from(v).into();
            } else if k == 2 {
                reg1 = register::Register::from(v).into()
            } else if k == 3 {
                data = v.parse().unwrap_or(0);
            }
        }
        Instruction::new(opcode, (reg1 << 4) & reg0, data)
    }
}
impl From<Instruction> for String {
    fn from(val: Instruction) -> Self {
        format!(
            "{} {} {} {}",
            val.opcode,
            u8::from(Register::from(val.regs & 0b00001111)),
            u8::from(Register::from(val.regs >> 4)),
            val.data
        )
    }
}
