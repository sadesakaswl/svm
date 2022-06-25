use crate::{instruction::Instruction, opcode::Opcode, register::Register};
#[derive(Default)]
pub(crate) struct S64 {
    zr: u64, //Zero Register
    pub r0: u64,
    pub r1: u64,
    pub r2: u64,
    pub f0: u64, //FP
    pub f1: u64, //FP
    pub f2: u64, //FP
    pub p0: u64,
    pub p1: u64,
    pub p2: u64,
    pub s0: [u64; 2], //SIMD128
    pub s1: [u64; 2], //SIMD128
    pub s2: [u64; 2], //SIMD128
    pub x0: [u64; 4], //SIMD256
    pub x1: [u64; 4], //SIMD256
    pub x2: [u64; 4], //SIMD256
    pc: u64,
}

impl S64 {
    pub(crate) fn execute_instruction(&mut self, instruction: Instruction) {
        let (reg0, reg1) = instruction.strip_registers();
        let data = instruction.data;
        match Opcode::from(instruction.opcode) {
            Opcode::Nop => self.nop(reg0, reg1, data),
            Opcode::Set => self.set(reg0, reg1, data),
            Opcode::Get => self.get(reg0, reg1, data),
            Opcode::Update => self.update(reg0, reg1, data),
            Opcode::Delete => self.delete(reg0, reg1, data),
            Opcode::And => self.and(reg0, reg1, data),
            Opcode::Or => self.or(reg0, reg1, data),
            Opcode::Xor => self.xor(reg0, reg1, data),
            Opcode::Not => self.not(reg0, reg1, data),
            Opcode::Shl => self.shl(reg0, reg1, data),
            Opcode::Shr => self.shr(reg0, reg1, data),
            Opcode::Dand => self.dand(reg0, reg1, data),
            Opcode::Dor => self.dor(reg0, reg1, data),
            Opcode::Dxor => self.dxor(reg0, reg1, data),
            Opcode::Dnot => self.dnot(reg0, reg1, data),
            Opcode::Dshl => self.dshl(reg0, reg1, data),
            Opcode::Dshr => self.dshr(reg0, reg1, data),
            Opcode::Add => self.add(reg0, reg1, data),
            Opcode::Sub => self.sub(reg0, reg1, data),
            Opcode::Mul => self.mul(reg0, reg1, data),
            Opcode::Div => self.div(reg0, reg1, data),
            Opcode::Mod => self.r#mod(reg0, reg1, data),
            Opcode::Inc => self.inc(reg0, reg1, data),
            Opcode::Dec => self.dec(reg0, reg1, data),
            Opcode::Neg => self.neg(reg0, reg1, data),
            Opcode::Dadd => self.dadd(reg0, reg1, data),
            Opcode::Dsub => self.dsub(reg0, reg1, data),
            Opcode::Dmul => self.dmul(reg0, reg1, data),
            Opcode::Ddiv => self.ddiv(reg0, reg1, data),
            Opcode::Dmod => self.dmod(reg0, reg1, data),
            Opcode::Dinc => self.dinc(reg0, reg1, data),
            Opcode::Ddec => self.ddec(reg0, reg1, data),
            Opcode::Dneg => self.dneg(reg0, reg1, data),
            Opcode::Swap => self.swap(reg0, reg1, data),
        }
    }
    pub(crate) fn execute_code(&mut self, code: Vec<u32>) {
        // code.par_iter().map(|ins| Instruction::from(*ins)).collect();
    }
    fn nop(&mut self, reg0: u8, reg1: u8, data: u16) {
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 = 0,
            Register::R1 => self.r1 = 0,
            Register::R2 => self.r2 = 0,
            Register::F0 => self.f0 = 0,
            Register::F1 => self.f1 = 0,
            Register::F2 => self.f2 = 0,
            Register::P0 => self.p0 = 0,
            Register::P1 => self.p1 = 0,
            Register::P2 => self.p2 = 0,
            Register::S0 => self.s0 = [0, 0],
            Register::S1 => self.s1 = [0, 0],
            Register::S2 => self.s2 = [0, 0],
            Register::X0 => self.x0 = [0, 0, 0, 0],
            Register::X1 => self.x1 = [0, 0, 0, 0],
            Register::X2 => self.x2 = [0, 0, 0, 0],
        };
        match Register::from(reg1) {
            Register::ZR => {}
            Register::R0 => self.r0 = 0,
            Register::R1 => self.r1 = 0,
            Register::R2 => self.r2 = 0,
            Register::F0 => self.f0 = 0,
            Register::F1 => self.f1 = 0,
            Register::F2 => self.f2 = 0,
            Register::P0 => self.p0 = 0,
            Register::P1 => self.p1 = 0,
            Register::P2 => self.p2 = 0,
            Register::S0 => self.s0 = [0, 0],
            Register::S1 => self.s1 = [0, 0],
            Register::S2 => self.s2 = [0, 0],
            Register::X0 => self.x0 = [0, 0, 0, 0],
            Register::X1 => self.x1 = [0, 0, 0, 0],
            Register::X2 => self.x2 = [0, 0, 0, 0],
        };
        self.jump_op(data)
    }
    fn set(&mut self, reg0: u8, reg1: u8, data: u16) {
        match Register::from(reg0) {
            Register::ZR => return,
            Register::R0 => {
                self.r0 &= !(0xFFFF << (16 * if reg1 < 4 { reg1 } else { 0 }));
                self.r0 |= (data as u64) << (16 * if reg1 < 4 { reg1 } else { 0 });
            }
            Register::R1 => {
                self.r1 &= !(0xFFFF << (16 * if reg1 < 4 { reg1 } else { 0 }));
                self.r1 |= (data as u64) << (16 * if reg1 < 4 { reg1 } else { 0 });
            }
            Register::R2 => {
                self.r2 &= !(0xFFFF << (16 * if reg1 < 4 { reg1 } else { 0 }));
                self.r2 |= (data as u64) << (16 * if reg1 < 4 { reg1 } else { 0 });
            }
            Register::F0 => {
                self.f0 &= !(0xFFFF << (16 * if reg1 < 4 { reg1 } else { 0 }));
                self.f0 |= (data as u64) << (16 * if reg1 < 4 { reg1 } else { 0 });
            }
            Register::F1 => {
                self.f1 &= !(0xFFFF << (16 * if reg1 < 4 { reg1 } else { 0 }));
                self.f1 |= (data as u64) << (16 * if reg1 < 4 { reg1 } else { 0 });
            }
            Register::F2 => {
                self.f2 &= !(0xFFFF << (16 * if reg1 < 4 { reg1 } else { 0 }));
                self.f2 |= (data as u64) << (16 * if reg1 < 4 { reg1 } else { 0 });
            }
            Register::P0 => {
                self.p0 &= !(0xFFFF << (16 * if reg1 < 4 { reg1 } else { 0 }));
                self.p0 |= (data as u64) << (16 * if reg1 < 4 { reg1 } else { 0 });
            }
            Register::P1 => {
                self.p1 &= !(0xFFFF << (16 * if reg1 < 4 { reg1 } else { 0 }));
                self.p1 |= (data as u64) << (16 * if reg1 < 4 { reg1 } else { 0 });
            }
            Register::P2 => {
                self.p2 &= !(0xFFFF << (16 * if reg1 < 4 { reg1 } else { 0 }));
                self.p2 |= (data as u64) << (16 * if reg1 < 4 { reg1 } else { 0 });
            }
            Register::S0 => {
                if reg1 < 4 {
                    self.s0[0] &= !(0xFFFF << (16 * reg1));
                    self.s0[0] |= (data as u64) << (16 * reg1);
                } else if reg1 < 8 {
                    self.s0[1] &= !(0xFFFF << (16 * (reg1 - 4)));
                    self.s0[1] |= (data as u64) << (16 * (reg1 - 4));
                } else {
                    self.s0[0] &= !(0xFFFF << (0));
                    self.s0[0] |= data as u64;
                }
            }
            Register::S1 => {
                if reg1 < 4 {
                    self.s1[0] &= !(0xFFFF << (16 * reg1));
                    self.s1[0] |= (data as u64) << (16 * reg1);
                } else if reg1 < 8 {
                    self.s1[1] &= !(0xFFFF << (16 * (reg1 - 4)));
                    self.s1[1] |= (data as u64) << (16 * (reg1 - 4));
                } else {
                    self.s1[0] &= !(0xFFFF << (0));
                    self.s1[0] |= data as u64;
                }
            }
            Register::S2 => {
                if reg1 < 4 {
                    self.s2[0] &= !(0xFFFF << (16 * reg1));
                    self.s2[0] |= (data as u64) << (16 * reg1);
                } else if reg1 < 8 {
                    self.s2[1] &= !(0xFFFF << (16 * (reg1 - 4)));
                    self.s2[1] |= (data as u64) << (16 * (reg1 - 4));
                } else {
                    self.s2[0] &= !(0xFFFF << 0);
                    self.s2[0] |= data as u64;
                }
            }
            Register::X0 => {
                if reg1 < 4 {
                    self.x0[0] &= !(0xFFFF << (16 * reg1));
                    self.x0[0] &= (data as u64) << (16 * reg1 as u64);
                } else if reg1 < 8 {
                    self.x0[1] &= !(0xFFFF << (16 * (reg1 - 4)));
                    self.x0[1] &= (data as u64) << ((16 * reg1 - 4) as u64);
                } else if reg1 < 12 {
                    self.x0[2] &= !(0xFFFF << (16 * (reg1 - 8)));
                    self.x0[2] &= (data as u64) << ((16 * reg1 - 8) as u64);
                } else if reg1 < 16 {
                    self.s2[3] &= !(0xFFFF << (16 * (reg1 - 12)));
                    self.x0[3] &= (data as u64) << ((16 * reg1 - 12) as u64);
                } else {
                    self.x0[0] &= !(0xFFFF << 0);
                    self.x0[0] &= data as u64;
                }
            }
            Register::X1 => {
                if reg1 < 4 {
                    self.x1[0] &= !(0xFFFF << (16 * reg1));
                    self.x1[0] &= (data as u64) << (16 * reg1 as u64);
                } else if reg1 < 8 {
                    self.x1[1] &= !(0xFFFF << (16 * (reg1 - 4)));
                    self.x1[1] &= (data as u64) << ((16 * reg1 - 4) as u64);
                } else if reg1 < 12 {
                    self.x1[2] &= !(0xFFFF << (16 * (reg1 - 8)));
                    self.x1[2] &= (data as u64) << ((16 * reg1 - 8) as u64);
                } else if reg1 < 16 {
                    self.x1[3] &= !(0xFFFF << (16 * (reg1 - 12)));
                    self.x1[3] &= (data as u64) << ((16 * reg1 - 12) as u64);
                } else {
                    self.x1[0] &= !(0xFFFF << 0);
                    self.x1[0] &= data as u64;
                }
            }
            Register::X2 => {
                if reg1 < 4 {
                    self.x2[0] &= !(0xFFFF << (16 * reg1));
                    self.x2[0] &= (data as u64) << (16 * reg1 as u64);
                } else if reg1 < 8 {
                    self.x2[1] &= !(0xFFFF << (16 * (reg1 - 4)));
                    self.x2[1] &= (data as u64) << ((16 * reg1 - 4) as u64);
                } else if reg1 < 12 {
                    self.x2[2] &= !(0xFFFF << (16 * (reg1 - 8)));
                    self.x2[2] &= (data as u64) << ((16 * reg1 - 8) as u64);
                } else if reg1 < 16 {
                    self.x2[3] &= !(0xFFFF << (16 * (reg1 - 12)));
                    self.x2[3] &= (data as u64) << ((16 * reg1 - 12) as u64);
                } else {
                    self.x2[0] &= !(0xFFFF << 0);
                    self.x2[0] &= data as u64;
                }
            }
        };
    }
    fn get(&mut self, reg0: u8, reg1: u8, data: u16) {
        let val = match Register::from(reg0) {
            //Get value
            Register::ZR => 0u64,
            Register::R0 => self.r0 & (0xFFFF << if reg1 < 4 { reg1 } else { 0 }),
            Register::R1 => self.r1 & (0xFFFF << if reg1 < 4 { reg1 } else { 0 }),
            Register::R2 => self.r2 & (0xFFFF << if reg1 < 4 { reg1 } else { 0 }),
            Register::F0 => self.f0 & (0xFFFF << if reg1 < 4 { reg1 } else { 0 }),
            Register::F1 => self.f1 & (0xFFFF << if reg1 < 4 { reg1 } else { 0 }),
            Register::F2 => self.f2 & (0xFFFF << if reg1 < 4 { reg1 } else { 0 }),
            Register::P0 => self.p0 & (0xFFFF << if reg1 < 4 { reg1 } else { 0 }),
            Register::P1 => self.p1 & (0xFFFF << if reg1 < 4 { reg1 } else { 0 }),
            Register::P2 => self.p2 & (0xFFFF << if reg1 < 4 { reg1 } else { 0 }),
            Register::S0 => {
                if reg1 < 4 {
                    self.s0[0] & (0xFFFF << reg1)
                } else if reg1 < 8 {
                    self.s0[1] & (0xFFFF << (reg1 - 4))
                } else {
                    self.s0[0]
                }
            }
            Register::S1 => {
                if reg1 < 4 {
                    self.s1[0] & (0xFFFF << reg1)
                } else if reg1 < 8 {
                    self.s1[1] & (0xFFFF << (reg1 - 4))
                } else {
                    self.s1[0]
                }
            }
            Register::S2 => {
                if reg1 < 4 {
                    self.s2[0] & (0xFFFF << reg1)
                } else if reg1 < 8 {
                    self.s2[1] & (0xFFFF << (reg1 - 4))
                } else {
                    self.s2[0]
                }
            }
            Register::X0 => {
                if reg1 < 4 {
                    self.x0[0] & (0xFFFF << reg1)
                } else if reg1 < 8 {
                    self.x0[1] & (0xFFFF << (reg1 - 4))
                } else if reg1 < 12 {
                    self.x0[2] & (0xFFFF << (reg1 - 8))
                } else if reg1 < 16 {
                    self.x0[3] & (0xFFFF << (reg1 - 12))
                } else {
                    self.x0[0]
                }
            }
            Register::X1 => {
                if reg1 < 4 {
                    self.x1[0] & (0xFFFF << reg1)
                } else if reg1 < 8 {
                    self.x1[1] & (0xFFFF << (reg1 - 4))
                } else if reg1 < 12 {
                    self.x1[2] & (0xFFFF << (reg1 - 8))
                } else if reg1 < 16 {
                    self.x1[3] & (0xFFFF << (reg1 - 12))
                } else {
                    self.x1[0]
                }
            }
            Register::X2 => {
                if reg1 < 4 {
                    self.x2[0] & (0xFFFF << reg1)
                } else if reg1 < 8 {
                    self.x2[1] & (0xFFFF << (reg1 - 4))
                } else if reg1 < 12 {
                    self.x2[2] & (0xFFFF << (reg1 - 8))
                } else if reg1 < 16 {
                    self.x2[3] & (0xFFFF << (reg1 - 12))
                } else {
                    self.x2[0]
                }
            }
        };
        self.r0 &= !(0xFFFF << (16 * (reg1 % 4)));
        self.r0 &= val;
        self.jump_op(data);
    }
    fn update(&mut self, reg0: u8, reg1: u8, data: u16) {
        let (val, val2, val3, val4) = self.get_vals(reg1);
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 = val,
            Register::R1 => self.r1 = val,
            Register::R2 => self.r2 = val,
            Register::F0 => self.f0 = val,
            Register::F1 => self.f1 = val,
            Register::F2 => self.f2 = val,
            Register::P0 => self.p0 = val,
            Register::P1 => self.p1 = val,
            Register::P2 => self.p2 = val,
            Register::S0 => {
                self.s0[0] = val;
                self.s0[1] = val2;
            }
            Register::S1 => {
                self.s1[0] = val;
                self.s1[1] = val2;
            }
            Register::S2 => {
                self.s2[0] = val;
                self.s2[1] = val2;
            }
            Register::X0 => {
                self.x0[0] = val;
                self.x0[1] = val2;
                self.x0[2] = val3;
                self.x0[3] = val4;
            }
            Register::X1 => {
                self.x1[0] = val;
                self.x1[1] = val2;
                self.x1[2] = val3;
                self.x1[3] = val4;
            }
            Register::X2 => {
                self.x2[0] = val;
                self.x2[1] = val2;
                self.x2[2] = val3;
                self.x2[3] = val4;
            }
        };
        self.jump_op(data)
    }
    fn delete(&mut self, reg0: u8, reg1: u8, data: u16) {
        self.update(reg0, reg1, 0);
        self.nop(Register::ZR.into(), reg1, 0);
        self.jump_op(data);
    }
    fn and(&mut self, reg0: u8, reg1: u8, data: u16) {
        let (val, val2, val3, val4) = self.get_vals(reg1);
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 &= val,
            Register::R1 => self.r1 &= val,
            Register::R2 => self.r2 &= val,
            Register::F0 => self.f0 &= val,
            Register::F1 => self.f1 &= val,
            Register::F2 => self.f2 &= val,
            Register::P0 => self.p0 &= val,
            Register::P1 => self.p1 &= val,
            Register::P2 => self.p2 &= val,
            Register::S0 => {
                self.s0[0] &= val;
                self.s0[1] &= val2;
            }
            Register::S1 => {
                self.s1[0] &= val;
                self.s1[1] &= val2;
            }
            Register::S2 => {
                self.s2[0] &= val;
                self.s2[1] &= val2;
            }
            Register::X0 => {
                self.x0[0] &= val;
                self.x0[1] &= val2;
                self.x0[2] &= val3;
                self.x0[3] &= val4;
            }
            Register::X1 => {
                self.x1[0] &= val;
                self.x1[1] &= val2;
                self.x1[2] &= val3;
                self.x1[3] &= val4;
            }
            Register::X2 => {
                self.x2[0] &= val;
                self.x2[1] &= val2;
                self.x2[2] &= val3;
                self.x2[3] &= val4;
            }
        };
        self.jump_op(data);
    }
    fn or(&mut self, reg0: u8, reg1: u8, data: u16) {
        let (val, val2, val3, val4) = self.get_vals(reg1);
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 |= val,
            Register::R1 => self.r1 |= val,
            Register::R2 => self.r2 |= val,
            Register::F0 => self.f0 |= val,
            Register::F1 => self.f1 |= val,
            Register::F2 => self.f2 |= val,
            Register::P0 => self.p0 |= val,
            Register::P1 => self.p1 |= val,
            Register::P2 => self.p2 |= val,
            Register::S0 => {
                self.s0[0] |= val;
                self.s0[1] |= val2;
            }
            Register::S1 => {
                self.s1[0] |= val;
                self.s1[1] |= val2;
            }
            Register::S2 => {
                self.s2[0] |= val;
                self.s2[1] |= val2;
            }
            Register::X0 => {
                self.x0[0] |= val;
                self.x0[1] |= val2;
                self.x0[2] |= val3;
                self.x0[3] |= val4;
            }
            Register::X1 => {
                self.x1[0] |= val;
                self.x1[1] |= val2;
                self.x1[2] |= val3;
                self.x1[3] |= val4;
            }
            Register::X2 => {
                self.x2[0] |= val;
                self.x2[1] |= val2;
                self.x2[2] |= val3;
                self.x2[3] |= val4;
            }
        };
        self.jump_op(data);
    }
    fn xor(&mut self, reg0: u8, reg1: u8, data: u16) {
        let (val, val2, val3, val4) = self.get_vals(reg1);
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 ^= val,
            Register::R1 => self.r1 ^= val,
            Register::R2 => self.r2 ^= val,
            Register::F0 => self.f0 ^= val,
            Register::F1 => self.f1 ^= val,
            Register::F2 => self.f2 ^= val,
            Register::P0 => self.p0 ^= val,
            Register::P1 => self.p1 ^= val,
            Register::P2 => self.p2 ^= val,
            Register::S0 => {
                self.s0[0] ^= val;
                self.s0[1] ^= val2;
            }
            Register::S1 => {
                self.s1[0] ^= val;
                self.s1[1] ^= val2;
            }
            Register::S2 => {
                self.s2[0] ^= val;
                self.s2[1] ^= val2;
            }
            Register::X0 => {
                self.x0[0] ^= val;
                self.x0[1] ^= val2;
                self.x0[2] ^= val3;
                self.x0[3] ^= val4;
            }
            Register::X1 => {
                self.x1[0] ^= val;
                self.x1[1] ^= val2;
                self.x1[2] ^= val3;
                self.x1[3] ^= val4;
            }
            Register::X2 => {
                self.x2[0] ^= val;
                self.x2[1] ^= val2;
                self.x2[2] ^= val3;
                self.x2[3] ^= val4;
            }
        };
        self.jump_op(data);
    }
    fn not(&mut self, reg0: u8, reg1: u8, data: u16) {
        let (val, val2, val3, val4) = self.get_vals(reg1);
        let val = !val;
        let val2 = !val2;
        let val3 = !val3;
        let val4 = !val4;
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 = val,
            Register::R1 => self.r1 = val,
            Register::R2 => self.r2 = val,
            Register::F0 => self.f0 = val,
            Register::F1 => self.f1 = val,
            Register::F2 => self.f2 = val,
            Register::P0 => self.p0 = val,
            Register::P1 => self.p1 = val,
            Register::P2 => self.p2 = val,
            Register::S0 => {
                self.s0[0] = val;
                self.s0[1] = val2;
            }
            Register::S1 => {
                self.s1[0] = val;
                self.s1[1] = val2;
            }
            Register::S2 => {
                self.s2[0] = val;
                self.s2[1] = val2;
            }
            Register::X0 => {
                self.x0[0] = val;
                self.x0[1] = val2;
                self.x0[2] = val3;
                self.x0[3] = val4;
            }
            Register::X1 => {
                self.x1[0] = val;
                self.x1[1] = val2;
                self.x1[2] = val3;
                self.x1[3] = val4;
            }
            Register::X2 => {
                self.x2[0] = val;
                self.x2[1] = val2;
                self.x2[2] = val3;
                self.x2[3] = val4;
            }
        };
        self.jump_op(data);
    }
    fn shl(&mut self, reg0: u8, reg1: u8, data: u16) {
        let (val, val2, val3, val4) = self.get_vals(reg1);
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 <<= val,
            Register::R1 => self.r1 <<= val,
            Register::R2 => self.r2 <<= val,
            Register::F0 => self.f0 <<= val,
            Register::F1 => self.f1 <<= val,
            Register::F2 => self.f2 <<= val,
            Register::P0 => self.p0 <<= val,
            Register::P1 => self.p1 <<= val,
            Register::P2 => self.p2 <<= val,
            Register::S0 => {
                if val >= 64 {
                    self.nop(Register::S0.into(), Register::ZR.into(), 0);
                } else {
                    self.s0[0] <<= val;
                    self.s0[1] <<= val2;
                }
            }
            Register::S1 => {
                if val >= 64 {
                    self.nop(Register::S1.into(), Register::ZR.into(), 0);
                } else {
                    self.s1[0] <<= val;
                    self.s1[1] <<= val2;
                }
            }
            Register::S2 => {
                if val >= 64 {
                    self.nop(Register::S2.into(), Register::ZR.into(), 0);
                } else {
                    self.s2[0] <<= val;
                    self.s2[1] <<= val2;
                }
            }
            Register::X0 => {
                if val >= 64 {
                    self.nop(Register::X0.into(), Register::ZR.into(), 0);
                } else {
                    self.x0[0] <<= val;
                    self.x0[1] <<= val2;
                    self.x0[2] <<= val3;
                    self.x0[3] <<= val4;
                }
            }
            Register::X1 => {
                if val >= 64 {
                    self.nop(Register::X1.into(), Register::ZR.into(), 0);
                } else {
                    self.x1[0] <<= val;
                    self.x1[1] <<= val2;
                    self.x1[2] <<= val3;
                    self.x1[3] <<= val4;
                }
            }
            Register::X2 => {
                if val >= 64 {
                    self.nop(Register::X2.into(), Register::ZR.into(), 0);
                } else {
                    self.x2[0] <<= val;
                    self.x2[1] <<= val2;
                    self.x2[2] <<= val3;
                    self.x2[3] <<= val4;
                }
            }
        };
        self.jump_op(data);
    }
    fn shr(&mut self, reg0: u8, reg1: u8, data: u16) {
        let (val, val2, val3, val4) = self.get_vals(reg1);
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 >>= val,
            Register::R1 => self.r1 >>= val,
            Register::R2 => self.r2 >>= val,
            Register::F0 => self.f0 >>= val,
            Register::F1 => self.f1 >>= val,
            Register::F2 => self.f2 >>= val,
            Register::P0 => self.p0 >>= val,
            Register::P1 => self.p1 >>= val,
            Register::P2 => self.p2 >>= val,
            Register::S0 => {
                if val >= 64 {
                    self.nop(Register::S0.into(), Register::ZR.into(), 0);
                } else {
                    self.s0[0] >>= val;
                    self.s0[1] >>= val2;
                }
            }
            Register::S1 => {
                if val >= 64 {
                    self.nop(Register::S1.into(), Register::ZR.into(), 0);
                } else {
                    self.s1[0] >>= val;
                    self.s1[1] >>= val2;
                }
            }
            Register::S2 => {
                if val >= 64 {
                    self.nop(Register::S2.into(), Register::ZR.into(), 0);
                } else {
                    self.s2[0] >>= val;
                    self.s2[1] >>= val2;
                }
            }
            Register::X0 => {
                if val >= 64 {
                    self.nop(Register::X0.into(), Register::ZR.into(), 0);
                } else {
                    self.x0[0] >>= val;
                    self.x0[1] >>= val2;
                    self.x0[2] >>= val3;
                    self.x0[3] >>= val4;
                }
            }
            Register::X1 => {
                if val >= 64 {
                    self.nop(Register::X1.into(), Register::ZR.into(), 0);
                } else {
                    self.x1[0] >>= val;
                    self.x1[1] >>= val2;
                    self.x1[2] >>= val3;
                    self.x1[3] >>= val4;
                }
            }
            Register::X2 => {
                if val >= 64 {
                    self.nop(Register::X2.into(), Register::ZR.into(), 0);
                } else {
                    self.x2[0] >>= val;
                    self.x2[1] >>= val2;
                    self.x2[2] >>= val3;
                    self.x2[3] >>= val4;
                }
            }
        };
        self.jump_op(data);
    }
    fn dand(&mut self, reg0: u8, reg1: u8, data: u16) {
        let (val, val2, val3, val4) = self.get_vals(reg1);
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 = val & (data as u64),
            Register::R1 => self.r1 = val & (data as u64),
            Register::R2 => self.r2 = val & (data as u64),
            Register::F0 => self.f0 = val & (data as u64),
            Register::F1 => self.f1 = val & (data as u64),
            Register::F2 => self.f2 = val & (data as u64),
            Register::P0 => self.p0 = val & (data as u64),
            Register::P1 => self.p1 = val & (data as u64),
            Register::P2 => self.p2 = val & (data as u64),
            Register::S0 => {
                self.s0[0] = val & (data as u64);
                self.s0[1] = val2 & (data as u64);
            }
            Register::S1 => {
                self.s1[0] = val & (data as u64);
                self.s1[1] = val2 & (data as u64);
            }
            Register::S2 => {
                self.s2[0] = val & (data as u64);
                self.s2[1] = val2 & (data as u64);
            }
            Register::X0 => {
                self.x0[0] = val & (data as u64);
                self.x0[1] = val2 & (data as u64);
                self.x0[2] = val3 & (data as u64);
                self.x0[3] = val4 & (data as u64);
            }
            Register::X1 => {
                self.x1[0] = val & (data as u64);
                self.x1[1] = val2 & (data as u64);
                self.x1[2] = val3 & (data as u64);
                self.x1[3] = val4 & (data as u64);
            }
            Register::X2 => {
                self.x2[0] = val & (data as u64);
                self.x2[1] = val2 & (data as u64);
                self.x2[2] = val3 & (data as u64);
                self.x2[3] = val4 & (data as u64);
            }
        };
    }
    fn dor(&mut self, reg0: u8, reg1: u8, data: u16) {
        let (val, val2, val3, val4) = self.get_vals(reg1);
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 = val | (data as u64),
            Register::R1 => self.r1 = val | (data as u64),
            Register::R2 => self.r2 = val | (data as u64),
            Register::F0 => self.f0 = val | (data as u64),
            Register::F1 => self.f1 = val | (data as u64),
            Register::F2 => self.f2 = val | (data as u64),
            Register::P0 => self.p0 = val | (data as u64),
            Register::P1 => self.p1 = val | (data as u64),
            Register::P2 => self.p2 = val | (data as u64),
            Register::S0 => {
                self.s0[0] = val | (data as u64);
                self.s0[1] = val2 | (data as u64);
            }
            Register::S1 => {
                self.s1[0] = val | (data as u64);
                self.s1[1] = val2 | (data as u64);
            }
            Register::S2 => {
                self.s2[0] = val | (data as u64);
                self.s2[1] = val2 | (data as u64);
            }
            Register::X0 => {
                self.x0[0] = val | (data as u64);
                self.x0[1] = val2 | (data as u64);
                self.x0[2] = val3 | (data as u64);
                self.x0[3] = val4 | (data as u64);
            }
            Register::X1 => {
                self.x1[0] = val | (data as u64);
                self.x1[1] = val2 | (data as u64);
                self.x1[2] = val3 | (data as u64);
                self.x1[3] = val4 | (data as u64);
            }
            Register::X2 => {
                self.x2[0] = val | (data as u64);
                self.x2[1] = val2 | (data as u64);
                self.x2[2] = val3 | (data as u64);
                self.x2[3] = val4 | (data as u64);
            }
        };
    }
    fn dxor(&mut self, reg0: u8, reg1: u8, data: u16) {
        let (val, val2, val3, val4) = self.get_vals(reg1);
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 = val ^ (data as u64),
            Register::R1 => self.r1 = val ^ (data as u64),
            Register::R2 => self.r2 = val ^ (data as u64),
            Register::F0 => self.f0 = val ^ (data as u64),
            Register::F1 => self.f1 = val ^ (data as u64),
            Register::F2 => self.f2 = val ^ (data as u64),
            Register::P0 => self.p0 = val ^ (data as u64),
            Register::P1 => self.p1 = val ^ (data as u64),
            Register::P2 => self.p2 = val ^ (data as u64),
            Register::S0 => {
                self.s0[0] = val ^ (data as u64);
                self.s0[1] = val2 ^ (data as u64);
            }
            Register::S1 => {
                self.s1[0] = val ^ (data as u64);
                self.s1[1] = val2 ^ (data as u64);
            }
            Register::S2 => {
                self.s2[0] = val ^ (data as u64);
                self.s2[1] = val2 ^ (data as u64);
            }
            Register::X0 => {
                self.x0[0] = val ^ (data as u64);
                self.x0[1] = val2 ^ (data as u64);
                self.x0[2] = val3 ^ (data as u64);
                self.x0[3] = val4 ^ (data as u64);
            }
            Register::X1 => {
                self.x1[0] = val ^ (data as u64);
                self.x1[1] = val2 ^ (data as u64);
                self.x1[2] = val3 ^ (data as u64);
                self.x1[3] = val4 ^ (data as u64);
            }
            Register::X2 => {
                self.x2[0] = val ^ (data as u64);
                self.x2[1] = val2 ^ (data as u64);
                self.x2[2] = val3 ^ (data as u64);
                self.x2[3] = val4 ^ (data as u64);
            }
        };
    }
    fn dnot(&mut self, reg0: u8, reg1: u8, data: u16) {
        match Register::from(reg1) {
            Register::ZR => {}
            Register::R0 => self.r0 = !self.r0,
            Register::R1 => self.r1 = !self.r1,
            Register::R2 => self.r2 = !self.r2,
            Register::F0 => self.f0 = !self.f0,
            Register::F1 => self.f1 = !self.f1,
            Register::F2 => self.f2 = !self.f2,
            Register::P0 => self.p0 = !self.p0,
            Register::P1 => self.p1 = !self.p1,
            Register::P2 => self.p2 = !self.p2,
            Register::S0 => {
                self.s0[0] = !self.s0[0];
                self.s0[1] = !self.s0[1];
            }
            Register::S1 => {
                self.s1[0] = !self.s1[0];
                self.s1[1] = !self.s1[1];
            }
            Register::S2 => {
                self.s2[0] = !self.s2[0];
                self.s2[1] = !self.s2[1];
            }
            Register::X0 => {
                self.x0[0] = !self.x0[0];
                self.x0[1] = !self.x0[1];
                self.x0[2] = !self.x0[2];
                self.x0[3] = !self.x0[3];
            }
            Register::X1 => {
                self.x1[0] = !self.x1[0];
                self.x1[1] = !self.x1[1];
                self.x1[2] = !self.x1[2];
                self.x1[3] = !self.x1[3];
            }
            Register::X2 => {
                self.x2[0] = !self.x2[0];
                self.x2[1] = !self.x2[1];
                self.x2[2] = !self.x2[2];
                self.x2[3] = !self.x2[3];
            }
        };
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 = !self.r0,
            Register::R1 => self.r1 = !self.r1,
            Register::R2 => self.r2 = !self.r2,
            Register::F0 => self.f0 = !self.f0,
            Register::F1 => self.f1 = !self.f1,
            Register::F2 => self.f2 = !self.f2,
            Register::P0 => self.p0 = !self.p0,
            Register::P1 => self.p1 = !self.p1,
            Register::P2 => self.p2 = !self.p2,
            Register::S0 => {
                self.s0[0] = !self.s0[0];
                self.s0[1] = !self.s0[1];
            }
            Register::S1 => {
                self.s1[0] = !self.s1[0];
                self.s1[1] = !self.s1[1];
            }
            Register::S2 => {
                self.s2[0] = !self.s2[0];
                self.s2[1] = !self.s2[1];
            }
            Register::X0 => {
                self.x0[0] = !self.x0[0];
                self.x0[1] = !self.x0[1];
                self.x0[2] = !self.x0[2];
                self.x0[3] = !self.x0[3];
            }
            Register::X1 => {
                self.x1[0] = !self.x1[0];
                self.x1[1] = !self.x1[1];
                self.x1[2] = !self.x1[2];
                self.x1[3] = !self.x1[3];
            }
            Register::X2 => {
                self.x2[0] = !self.x2[0];
                self.x2[1] = !self.x2[1];
                self.x2[2] = !self.x2[2];
                self.x2[3] = !self.x2[3];
            }
        };
        self.jump_op(data);
    }
    fn dshl(&mut self, reg0: u8, reg1: u8, data: u16) {
        let (val, val2, val3, val4) = self.get_vals(reg1);
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 = val << (data as u64),
            Register::R1 => self.r1 = val << (data as u64),
            Register::R2 => self.r2 = val << (data as u64),
            Register::F0 => self.f0 = val << (data as u64),
            Register::F1 => self.f1 = val << (data as u64),
            Register::F2 => self.f2 = val << (data as u64),
            Register::P0 => self.p0 = val << (data as u64),
            Register::P1 => self.p1 = val << (data as u64),
            Register::P2 => self.p2 = val << (data as u64),
            Register::S0 => {
                self.s0[0] = val << data;
                self.s0[1] = val2 << data;
            }
            Register::S1 => {
                self.s1[0] = val << data;
                self.s1[1] = val2 << data;
            }
            Register::S2 => {
                self.s2[0] = val << data;
                self.s2[1] = val2 << data;
            }
            Register::X0 => {
                self.x0[0] = val << data;
                self.x0[1] = val2 << data;
                self.x0[2] = val3 << data;
                self.x0[3] = val4 << data;
            }
            Register::X1 => {
                self.x1[0] = val << data;
                self.x1[1] = val2 << data;
                self.x1[2] = val3 << data;
                self.x1[3] = val4 << data;
            }
            Register::X2 => {
                self.x0[0] = val << data;
                self.x0[1] = val2 << data;
                self.x0[2] = val3 << data;
                self.x0[3] = val4 << data;
            }
        };
    }
    fn dshr(&mut self, reg0: u8, reg1: u8, data: u16) {
        let (val, val2, val3, val4) = self.get_vals(reg1);
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 = val >> (data as u64),
            Register::R1 => self.r1 = val >> (data as u64),
            Register::R2 => self.r2 = val >> (data as u64),
            Register::F0 => self.f0 = val >> (data as u64),
            Register::F1 => self.f1 = val >> (data as u64),
            Register::F2 => self.f2 = val >> (data as u64),
            Register::P0 => self.p0 = val >> (data as u64),
            Register::P1 => self.p1 = val >> (data as u64),
            Register::P2 => self.p2 = val >> (data as u64),
            Register::S0 => {
                self.s0[0] = val >> data;
                self.s0[1] = val2 >> data;
            }
            Register::S1 => {
                self.s1[0] = val >> data;
                self.s1[1] = val2 >> data;
            }
            Register::S2 => {
                self.s2[0] = val >> data;
                self.s2[1] = val2 >> data;
            }
            Register::X0 => {
                self.x0[0] = val >> data;
                self.x0[1] = val2 >> data;
                self.x0[2] = val3 >> data;
                self.x0[3] = val4 >> data;
            }
            Register::X1 => {
                self.x1[0] = val >> data;
                self.x1[1] = val2 >> data;
                self.x1[2] = val3 >> data;
                self.x1[3] = val4 >> data;
            }
            Register::X2 => {
                self.x0[0] = val >> data;
                self.x0[1] = val2 >> data;
                self.x0[2] = val3 >> data;
                self.x0[3] = val4 >> data;
            }
        };
    }
    fn add(&mut self, reg0: u8, reg1: u8, data: u16) {
        let (val, val2, val3, val4) = self.get_vals(reg1);
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 += val,
            Register::R1 => self.r1 += val,
            Register::R2 => self.r2 += val,
            Register::F0 => self.f0 = ((val as f64) + (self.f0 as f64)).to_bits(),
            Register::F1 => self.f1 = ((val as f64) + (self.f1 as f64)).to_bits(),
            Register::F2 => self.f2 = ((val as f64) + (self.f2 as f64)).to_bits(),
            Register::P0 => self.p0 += val,
            Register::P1 => self.p1 += val,
            Register::P2 => self.p2 += val,
            Register::S0 => {
                self.s0[0] += val;
                self.s0[1] += val2;
            }
            Register::S1 => {
                self.s1[0] += val;
                self.s1[1] += val2;
            }
            Register::S2 => {
                self.s2[0] += val;
                self.s2[1] += val2;
            }
            Register::X0 => {
                self.x0[0] += val;
                self.x0[1] += val2;
                self.x0[2] += val3;
                self.x0[3] += val4;
            }
            Register::X1 => {
                self.x1[0] += val;
                self.x1[1] += val2;
                self.x1[2] += val3;
                self.x1[3] += val4;
            }
            Register::X2 => {
                self.x2[0] += val;
                self.x2[1] += val2;
                self.x2[2] += val3;
                self.x2[3] += val4;
            }
        };
        self.jump_op(data);
    }
    fn sub(&mut self, reg0: u8, reg1: u8, data: u16) {
        let (val, val2, val3, val4) = self.get_vals(reg1);
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 -= val,
            Register::R1 => self.r1 -= val,
            Register::R2 => self.r2 -= val,
            Register::F0 => self.f0 = ((val as f64) - (self.f0 as f64)).to_bits(),
            Register::F1 => self.f1 = ((val as f64) - (self.f1 as f64)).to_bits(),
            Register::F2 => self.f2 = ((val as f64) - (self.f2 as f64)).to_bits(),
            Register::P0 => self.p0 -= val,
            Register::P1 => self.p1 -= val,
            Register::P2 => self.p2 -= val,
            Register::S0 => {
                self.s0[0] -= val;
                self.s0[1] -= val2;
            }
            Register::S1 => {
                self.s1[0] -= val;
                self.s1[1] -= val2;
            }
            Register::S2 => {
                self.s2[0] -= val;
                self.s2[1] -= val2;
            }
            Register::X0 => {
                self.x0[0] -= val;
                self.x0[1] -= val2;
                self.x0[2] -= val3;
                self.x0[3] -= val4;
            }
            Register::X1 => {
                self.x1[0] -= val;
                self.x1[1] -= val2;
                self.x1[2] -= val3;
                self.x1[3] -= val4;
            }
            Register::X2 => {
                self.x2[0] -= val;
                self.x2[1] -= val2;
                self.x2[2] -= val3;
                self.x2[3] -= val4;
            }
        };
        self.jump_op(data);
    }
    fn mul(&mut self, reg0: u8, reg1: u8, data: u16) {
        let (val, val2, val3, val4) = self.get_vals(reg1);
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 *= val,
            Register::R1 => self.r1 *= val,
            Register::R2 => self.r2 *= val,
            Register::F0 => self.f0 = ((val as f64) * (self.f0 as f64)).to_bits(),
            Register::F1 => self.f1 = ((val as f64) * (self.f1 as f64)).to_bits(),
            Register::F2 => self.f2 = ((val as f64) * (self.f2 as f64)).to_bits(),
            Register::P0 => self.p0 *= val,
            Register::P1 => self.p1 *= val,
            Register::P2 => self.p2 *= val,
            Register::S0 => {
                self.s0[0] *= val;
                self.s0[1] *= val2;
            }
            Register::S1 => {
                self.s1[0] *= val;
                self.s1[1] *= val2;
            }
            Register::S2 => {
                self.s2[0] *= val;
                self.s2[1] *= val2;
            }
            Register::X0 => {
                self.x0[0] *= val;
                self.x0[1] *= val2;
                self.x0[2] *= val3;
                self.x0[3] *= val4;
            }
            Register::X1 => {
                self.x1[0] *= val;
                self.x1[1] *= val2;
                self.x1[2] *= val3;
                self.x1[3] *= val4;
            }
            Register::X2 => {
                self.x2[0] *= val;
                self.x2[1] *= val2;
                self.x2[2] *= val3;
                self.x2[3] *= val4;
            }
        };
        self.jump_op(data);
    }
    fn div(&mut self, reg0: u8, reg1: u8, data: u16) {
        let (val, val2, val3, val4) = self.get_vals(reg1);
        if val == 0 || val2 == 0 || val3 == 0 || val4 == 0 {
            return;
        }
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 /= val,
            Register::R1 => self.r1 /= val,
            Register::R2 => self.r2 /= val,
            Register::F0 => self.f0 = ((val as f64) / (self.f0 as f64)).to_bits(),
            Register::F1 => self.f1 = ((val as f64) / (self.f1 as f64)).to_bits(),
            Register::F2 => self.f2 = ((val as f64) / (self.f2 as f64)).to_bits(),
            Register::P0 => self.p0 /= val,
            Register::P1 => self.p1 /= val,
            Register::P2 => self.p2 /= val,
            Register::S0 => {
                self.s0[0] /= val;
                self.s0[1] /= val2;
            }
            Register::S1 => {
                self.s1[0] /= val;
                self.s1[1] /= val2;
            }
            Register::S2 => {
                self.s2[0] /= val;
                self.s2[1] /= val2;
            }
            Register::X0 => {
                self.x0[0] /= val;
                self.x0[1] /= val2;
                self.x0[2] /= val3;
                self.x0[3] /= val4;
            }
            Register::X1 => {
                self.x1[0] /= val;
                self.x1[1] /= val2;
                self.x1[2] /= val3;
                self.x1[3] /= val4;
            }
            Register::X2 => {
                self.x2[0] /= val;
                self.x2[1] /= val2;
                self.x2[2] /= val3;
                self.x2[3] /= val4;
            }
        };
        self.jump_op(data);
    }
    fn r#mod(&mut self, reg0: u8, reg1: u8, data: u16) {
        let (val, val2, val3, val4) = self.get_vals(reg1);
        if val == 0 || val2 == 0 || val3 == 0 || val4 == 0 {
            return;
        }
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 %= val,
            Register::R1 => self.r1 %= val,
            Register::R2 => self.r2 %= val,
            Register::F0 => self.f0 = ((val as f64) % (self.f0 as f64)).to_bits(),
            Register::F1 => self.f1 = ((val as f64) % (self.f1 as f64)).to_bits(),
            Register::F2 => self.f2 = ((val as f64) % (self.f2 as f64)).to_bits(),
            Register::P0 => self.p0 %= val,
            Register::P1 => self.p1 %= val,
            Register::P2 => self.p2 %= val,
            Register::S0 => {
                self.s0[0] %= val;
                self.s0[1] %= val2;
            }
            Register::S1 => {
                self.s1[0] %= val;
                self.s1[1] %= val2;
            }
            Register::S2 => {
                self.s2[0] %= val;
                self.s2[1] %= val2;
            }
            Register::X0 => {
                self.x0[0] %= val;
                self.x0[1] %= val2;
                self.x0[2] %= val3;
                self.x0[3] %= val4;
            }
            Register::X1 => {
                self.x1[0] %= val;
                self.x1[1] %= val2;
                self.x1[2] %= val3;
                self.x1[3] %= val4;
            }
            Register::X2 => {
                self.x2[0] %= val;
                self.x2[1] %= val2;
                self.x2[2] %= val3;
                self.x2[3] %= val4;
            }
        };
        self.jump_op(data);
    }
    fn inc(&mut self, reg0: u8, reg1: u8, data: u16) {
        match Register::from(reg1) {
            Register::ZR => {}
            Register::R0 => self.r0 = self.r0 + 1,
            Register::R1 => self.r1 = self.r1 + 1,
            Register::R2 => self.r2 = self.r2 + 1,
            Register::F0 => self.f0 = ((self.f0 as f64) + 1.0).to_bits(),
            Register::F1 => self.f1 = ((self.f1 as f64) + 1.0).to_bits(),
            Register::F2 => self.f2 = ((self.f0 as f64) + 1.0).to_bits(),
            Register::P0 => self.p0 = self.p0 + 1,
            Register::P1 => self.p1 = self.p1 + 1,
            Register::P2 => self.p2 = self.p2 + 1,
            Register::S0 => {
                self.s0[0] += 1;
                self.s0[1] += 1;
            }
            Register::S1 => {
                self.s1[0] += 1;
                self.s1[1] += 1;
            }
            Register::S2 => {
                self.s2[0] += 1;
                self.s2[1] += 1;
            }
            Register::X0 => {
                self.x0[0] += 1;
                self.x0[1] += 1;
                self.x0[2] += 1;
                self.x0[3] += 1;
            }
            Register::X1 => {
                self.x1[0] += 1;
                self.x1[1] += 1;
                self.x1[2] += 1;
                self.x1[3] += 1;
            }
            Register::X2 => {
                self.x2[0] += 1;
                self.x2[1] += 1;
                self.x2[2] += 1;
                self.x2[3] += 1;
            }
        };
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 = self.r0 + 1,
            Register::R1 => self.r1 = self.r1 + 1,
            Register::R2 => self.r2 = self.r2 + 1,
            Register::F0 => self.f0 = ((self.f0 as f64) + 1.0).to_bits(),
            Register::F1 => self.f1 = ((self.f1 as f64) + 1.0).to_bits(),
            Register::F2 => self.f2 = ((self.f0 as f64) + 1.0).to_bits(),
            Register::P0 => self.p0 = self.p0 + 1,
            Register::P1 => self.p1 = self.p1 + 1,
            Register::P2 => self.p2 = self.p2 + 1,
            Register::S0 => {
                self.s0[0] += 1;
                self.s0[1] += 1;
            }
            Register::S1 => {
                self.s1[0] += 1;
                self.s1[1] += 1;
            }
            Register::S2 => {
                self.s2[0] += 1;
                self.s2[1] += 1;
            }
            Register::X0 => {
                self.x0[0] += 1;
                self.x0[1] += 1;
                self.x0[2] += 1;
                self.x0[3] += 1;
            }
            Register::X1 => {
                self.x1[0] += 1;
                self.x1[1] += 1;
                self.x1[2] += 1;
                self.x1[3] += 1;
            }
            Register::X2 => {
                self.x2[0] += 1;
                self.x2[1] += 1;
                self.x2[2] += 1;
                self.x2[3] += 1;
            }
        };
        self.jump_op(data);
    }
    fn dec(&mut self, reg0: u8, reg1: u8, data: u16) {
        match Register::from(reg1) {
            Register::ZR => {}
            Register::R0 => self.r0 = self.r0 + 1,
            Register::R1 => self.r1 = self.r1 + 1,
            Register::R2 => self.r2 = self.r2 + 1,
            Register::F0 => self.f0 = ((self.f0 as f64) + 1.0).to_bits(),
            Register::F1 => self.f1 = ((self.f1 as f64) + 1.0).to_bits(),
            Register::F2 => self.f2 = ((self.f0 as f64) + 1.0).to_bits(),
            Register::P0 => self.p0 = self.p0 + 1,
            Register::P1 => self.p1 = self.p1 + 1,
            Register::P2 => self.p2 = self.p2 + 1,
            Register::S0 => {
                self.s0[0] += 1;
                self.s0[1] += 1;
            }
            Register::S1 => {
                self.s1[0] += 1;
                self.s1[1] += 1;
            }
            Register::S2 => {
                self.s2[0] += 1;
                self.s2[1] += 1;
            }
            Register::X0 => {
                self.x0[0] += 1;
                self.x0[1] += 1;
                self.x0[2] += 1;
                self.x0[3] += 1;
            }
            Register::X1 => {
                self.x1[0] += 1;
                self.x1[1] += 1;
                self.x1[2] += 1;
                self.x1[3] += 1;
            }
            Register::X2 => {
                self.x2[0] += 1;
                self.x2[1] += 1;
                self.x2[2] += 1;
                self.x2[3] += 1;
            }
        };
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 = self.r0 - 1,
            Register::R1 => self.r1 = self.r1 - 1,
            Register::R2 => self.r2 = self.r2 - 1,
            Register::F0 => self.f0 = ((self.f0 as f64) - 1.0).to_bits(),
            Register::F1 => self.f1 = ((self.f1 as f64) - 1.0).to_bits(),
            Register::F2 => self.f2 = ((self.f0 as f64) - 1.0).to_bits(),
            Register::P0 => self.p0 = self.p0 - 1,
            Register::P1 => self.p1 = self.p1 - 1,
            Register::P2 => self.p2 = self.p2 - 1,
            Register::S0 => {
                self.s0[0] -= 1;
                self.s0[1] -= 1;
            }
            Register::S1 => {
                self.s1[0] -= 1;
                self.s1[1] -= 1;
            }
            Register::S2 => {
                self.s2[0] -= 1;
                self.s2[1] -= 1;
            }
            Register::X0 => {
                self.x0[0] -= 1;
                self.x0[1] -= 1;
                self.x0[2] -= 1;
                self.x0[3] -= 1;
            }
            Register::X1 => {
                self.x1[0] -= 1;
                self.x1[1] -= 1;
                self.x1[2] -= 1;
                self.x1[3] -= 1;
            }
            Register::X2 => {
                self.x2[0] -= 1;
                self.x2[1] -= 1;
                self.x2[2] -= 1;
                self.x2[3] -= 1;
            }
        };
        self.jump_op(data);
    }
    fn neg(&mut self, reg0: u8, reg1: u8, data: u16) {
        match Register::from(reg1) {
            Register::ZR => {}
            Register::R0 => self.r0 = self.r0.wrapping_neg(),
            Register::R1 => self.r1 = self.r1.wrapping_neg(),
            Register::R2 => self.r2 = self.r2.wrapping_neg(),
            Register::F0 => self.f0 = (-(self.f0 as f64)).to_bits(),
            Register::F1 => self.f1 = (-(self.f1 as f64)).to_bits(),
            Register::F2 => self.f2 = (-(self.f0 as f64)).to_bits(),
            Register::P0 => self.p0 = self.p0.wrapping_neg(),
            Register::P1 => self.p1 = self.p1.wrapping_neg(),
            Register::P2 => self.p2 = self.p2.wrapping_neg(),
            Register::S0 => {
                self.s0[0] = self.s0[0].wrapping_neg();
                self.s0[1] = self.s0[1].wrapping_neg();
            }
            Register::S1 => {
                self.s1[0] = self.s1[0].wrapping_neg();
                self.s1[1] = self.s1[1].wrapping_neg();
            }
            Register::S2 => {
                self.s2[0] = self.s2[0].wrapping_neg();
                self.s2[1] = self.s2[1].wrapping_neg();
            }
            Register::X0 => {
                self.x0[0] = self.x0[0].wrapping_neg();
                self.x0[1] = self.x0[1].wrapping_neg();
                self.x0[2] = self.x0[2].wrapping_neg();
                self.x0[3] = self.x0[3].wrapping_neg();
            }
            Register::X1 => {
                self.x1[0] = self.x1[0].wrapping_neg();
                self.x1[1] = self.x1[1].wrapping_neg();
                self.x1[2] = self.x1[2].wrapping_neg();
                self.x1[3] = self.x1[3].wrapping_neg();
            }
            Register::X2 => {
                self.x2[0] = self.x2[0].wrapping_neg();
                self.x2[1] = self.x2[1].wrapping_neg();
                self.x2[2] = self.x2[2].wrapping_neg();
                self.x2[3] = self.x2[3].wrapping_neg();
            }
        };
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 = self.r0.wrapping_neg(),
            Register::R1 => self.r1 = self.r1.wrapping_neg(),
            Register::R2 => self.r2 = self.r2.wrapping_neg(),
            Register::F0 => self.f0 = (-(self.f0 as f64)).to_bits(),
            Register::F1 => self.f1 = (-(self.f1 as f64)).to_bits(),
            Register::F2 => self.f2 = (-(self.f0 as f64)).to_bits(),
            Register::P0 => self.p0 = self.p0.wrapping_neg(),
            Register::P1 => self.p1 = self.p1.wrapping_neg(),
            Register::P2 => self.p2 = self.p2.wrapping_neg(),
            Register::S0 => {
                self.s0[0] = self.s0[0].wrapping_neg();
                self.s0[1] = self.s0[1].wrapping_neg();
            }
            Register::S1 => {
                self.s1[0] = self.s1[0].wrapping_neg();
                self.s1[1] = self.s1[1].wrapping_neg();
            }
            Register::S2 => {
                self.s2[0] = self.s2[0].wrapping_neg();
                self.s2[1] = self.s2[1].wrapping_neg();
            }
            Register::X0 => {
                self.x0[0] = self.x0[0].wrapping_neg();
                self.x0[1] = self.x0[1].wrapping_neg();
                self.x0[2] = self.x0[2].wrapping_neg();
                self.x0[3] = self.x0[3].wrapping_neg();
            }
            Register::X1 => {
                self.x1[0] = self.x1[0].wrapping_neg();
                self.x1[1] = self.x1[1].wrapping_neg();
                self.x1[2] = self.x1[2].wrapping_neg();
                self.x1[3] = self.x1[3].wrapping_neg();
            }
            Register::X2 => {
                self.x2[0] = self.x2[0].wrapping_neg();
                self.x2[1] = self.x2[1].wrapping_neg();
                self.x2[2] = self.x2[2].wrapping_neg();
                self.x2[3] = self.x2[3].wrapping_neg();
            }
        };
        self.jump_op(data);
    }
    fn dadd(&mut self, reg0: u8, reg1: u8, data: u16) {
        let (val, val2, val3, val4) = self.get_vals(reg1);
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 = val + (data as u64),
            Register::R1 => self.r1 = val + (data as u64),
            Register::R2 => self.r2 = val + (data as u64),
            Register::F0 => self.f0 = ((val as f64) + (data as f64)).to_bits(),
            Register::F1 => self.f1 = ((val as f64) + (data as f64)).to_bits(),
            Register::F2 => self.f2 = ((val as f64) + (data as f64)).to_bits(),
            Register::P0 => self.p0 = val + (data as u64),
            Register::P1 => self.p1 = val + (data as u64),
            Register::P2 => self.p2 = val + (data as u64),
            Register::S0 => {
                self.s0[0] = val + (data as u64);
                self.s0[1] = val2 + (data as u64);
            }
            Register::S1 => {
                self.s1[0] = val + (data as u64);
                self.s1[1] = val2 + (data as u64);
            }
            Register::S2 => {
                self.s2[0] = val + (data as u64);
                self.s2[1] = val2 + (data as u64);
            }
            Register::X0 => {
                self.x0[0] = val + (data as u64);
                self.x0[1] = val2 + (data as u64);
                self.x0[2] = val3 + (data as u64);
                self.x0[3] = val4 + (data as u64);
            }
            Register::X1 => {
                self.x1[0] = val + (data as u64);
                self.x1[1] = val2 + (data as u64);
                self.x1[2] = val3 + (data as u64);
                self.x1[3] = val4 + (data as u64);
            }
            Register::X2 => {
                self.x2[0] = val + (data as u64);
                self.x2[1] = val2 + (data as u64);
                self.x2[2] = val3 + (data as u64);
                self.x2[3] = val4 + (data as u64);
            }
        };
    }
    fn dsub(&mut self, reg0: u8, reg1: u8, data: u16) {
        let (val, val2, val3, val4) = self.get_vals(reg1);
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 = val - (data as u64),
            Register::R1 => self.r1 = val - (data as u64),
            Register::R2 => self.r2 = val - (data as u64),
            Register::F0 => self.f0 = ((val as f64) - (data as f64)).to_bits(),
            Register::F1 => self.f1 = ((val as f64) - (data as f64)).to_bits(),
            Register::F2 => self.f2 = ((val as f64) - (data as f64)).to_bits(),
            Register::P0 => self.p0 = val - (data as u64),
            Register::P1 => self.p1 = val - (data as u64),
            Register::P2 => self.p2 = val - (data as u64),
            Register::S0 => {
                self.s0[0] = val - (data as u64);
                self.s0[1] = val2 - (data as u64);
            }
            Register::S1 => {
                self.s1[0] = val - (data as u64);
                self.s1[1] = val2 - (data as u64);
            }
            Register::S2 => {
                self.s2[0] = val - (data as u64);
                self.s2[1] = val2 - (data as u64);
            }
            Register::X0 => {
                self.x0[0] = val - (data as u64);
                self.x0[1] = val2 - (data as u64);
                self.x0[2] = val3 - (data as u64);
                self.x0[3] = val4 - (data as u64);
            }
            Register::X1 => {
                self.x1[0] = val - (data as u64);
                self.x1[1] = val2 - (data as u64);
                self.x1[2] = val3 - (data as u64);
                self.x1[3] = val4 - (data as u64);
            }
            Register::X2 => {
                self.x2[0] = val - (data as u64);
                self.x2[1] = val2 - (data as u64);
                self.x2[2] = val3 - (data as u64);
                self.x2[3] = val4 - (data as u64);
            }
        };
    }
    fn dmul(&mut self, reg0: u8, reg1: u8, data: u16) {
        let (val, val2, val3, val4) = self.get_vals(reg1);
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 = val * (data as u64),
            Register::R1 => self.r1 = val * (data as u64),
            Register::R2 => self.r2 = val * (data as u64),
            Register::F0 => self.f0 = ((val as f64) * (data as f64)).to_bits(),
            Register::F1 => self.f1 = ((val as f64) * (data as f64)).to_bits(),
            Register::F2 => self.f2 = ((val as f64) * (data as f64)).to_bits(),
            Register::P0 => self.p0 = val * (data as u64),
            Register::P1 => self.p1 = val * (data as u64),
            Register::P2 => self.p2 = val * (data as u64),
            Register::S0 => {
                self.s0[0] = val * (data as u64);
                self.s0[1] = val2 * (data as u64);
            }
            Register::S1 => {
                self.s1[0] = val * (data as u64);
                self.s1[1] = val2 * (data as u64);
            }
            Register::S2 => {
                self.s2[0] = val * (data as u64);
                self.s2[1] = val2 * (data as u64);
            }
            Register::X0 => {
                self.x0[0] = val * (data as u64);
                self.x0[1] = val2 * (data as u64);
                self.x0[2] = val3 * (data as u64);
                self.x0[3] = val4 * (data as u64);
            }
            Register::X1 => {
                self.x1[0] = val * (data as u64);
                self.x1[1] = val2 * (data as u64);
                self.x1[2] = val3 * (data as u64);
                self.x1[3] = val4 * (data as u64);
            }
            Register::X2 => {
                self.x2[0] = val * (data as u64);
                self.x2[1] = val2 * (data as u64);
                self.x2[2] = val3 * (data as u64);
                self.x2[3] = val4 * (data as u64);
            }
        };
    }
    fn ddiv(&mut self, reg0: u8, reg1: u8, data: u16) {
        if data == 0 {
            return;
        }
        let (val, val2, val3, val4) = self.get_vals(reg1);
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 = val / (data as u64),
            Register::R1 => self.r1 = val / (data as u64),
            Register::R2 => self.r2 = val / (data as u64),
            Register::F0 => self.f0 = ((val as f64) / (data as f64)).to_bits(),
            Register::F1 => self.f1 = ((val as f64) / (data as f64)).to_bits(),
            Register::F2 => self.f2 = ((val as f64) / (data as f64)).to_bits(),
            Register::P0 => self.p0 = val / (data as u64),
            Register::P1 => self.p1 = val / (data as u64),
            Register::P2 => self.p2 = val / (data as u64),
            Register::S0 => {
                self.s0[0] = val / (data as u64);
                self.s0[1] = val2 / (data as u64);
            }
            Register::S1 => {
                self.s1[0] = val / (data as u64);
                self.s1[1] = val2 / (data as u64);
            }
            Register::S2 => {
                self.s2[0] = val / (data as u64);
                self.s2[1] = val2 / (data as u64);
            }
            Register::X0 => {
                self.x0[0] = val / (data as u64);
                self.x0[1] = val2 / (data as u64);
                self.x0[2] = val3 / (data as u64);
                self.x0[3] = val4 / (data as u64);
            }
            Register::X1 => {
                self.x1[0] = val / (data as u64);
                self.x1[1] = val2 / (data as u64);
                self.x1[2] = val3 / (data as u64);
                self.x1[3] = val4 / (data as u64);
            }
            Register::X2 => {
                self.x2[0] = val / (data as u64);
                self.x2[1] = val2 / (data as u64);
                self.x2[2] = val3 / (data as u64);
                self.x2[3] = val4 / (data as u64);
            }
        };
    }
    fn dmod(&mut self, reg0: u8, reg1: u8, data: u16) {
        if data == 0 {
            return;
        }
        let (val, val2, val3, val4) = self.get_vals(reg1);
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 = val % (data as u64),
            Register::R1 => self.r1 = val % (data as u64),
            Register::R2 => self.r2 = val % (data as u64),
            Register::F0 => self.f0 = ((val as f64) % (data as f64)).to_bits(),
            Register::F1 => self.f1 = ((val as f64) % (data as f64)).to_bits(),
            Register::F2 => self.f2 = ((val as f64) % (data as f64)).to_bits(),
            Register::P0 => self.p0 = val % (data as u64),
            Register::P1 => self.p1 = val % (data as u64),
            Register::P2 => self.p2 = val % (data as u64),
            Register::S0 => {
                self.s0[0] = val % (data as u64);
                self.s0[1] = val2 % (data as u64);
            }
            Register::S1 => {
                self.s1[0] = val % (data as u64);
                self.s1[1] = val2 % (data as u64);
            }
            Register::S2 => {
                self.s2[0] = val % (data as u64);
                self.s2[1] = val2 % (data as u64);
            }
            Register::X0 => {
                self.x0[0] = val % (data as u64);
                self.x0[1] = val2 % (data as u64);
                self.x0[2] = val3 % (data as u64);
                self.x0[3] = val4 % (data as u64);
            }
            Register::X1 => {
                self.x1[0] = val % (data as u64);
                self.x1[1] = val2 % (data as u64);
                self.x1[2] = val3 % (data as u64);
                self.x1[3] = val4 % (data as u64);
            }
            Register::X2 => {
                self.x2[0] = val % (data as u64);
                self.x2[1] = val2 % (data as u64);
                self.x2[2] = val3 % (data as u64);
                self.x2[3] = val4 % (data as u64);
            }
        };
    }
    fn dinc(&mut self, reg0: u8, reg1: u8, data: u16) {
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 = (data as u64) + 1,
            Register::R1 => self.r1 = (data as u64) + 1,
            Register::R2 => self.r2 = (data as u64) + 1,
            Register::F0 => self.f0 = ((data as f64) + 1.0).to_bits(),
            Register::F1 => self.f1 = ((data as f64) + 1.0).to_bits(),
            Register::F2 => self.f2 = ((data as f64) + 1.0).to_bits(),
            Register::P0 => self.p0 = (data as u64) + 1,
            Register::P1 => self.p1 = (data as u64) + 1,
            Register::P2 => self.p2 = (data as u64) + 1,
            Register::S0 => {
                self.s0[0] = (data as u64) + 1;
                self.s0[1] = (data as u64) + 1;
            }
            Register::S1 => {
                self.s1[0] = (data as u64) + 1;
                self.s1[1] = (data as u64) + 1;
            }
            Register::S2 => {
                self.s2[0] = (data as u64) + 1;
                self.s2[1] = (data as u64) + 1;
            }
            Register::X0 => {
                self.x0[0] = (data as u64) + 1;
                self.x0[1] = (data as u64) + 1;
                self.x0[2] = (data as u64) + 1;
                self.x0[3] = (data as u64) + 1;
            }
            Register::X1 => {
                self.x1[0] = (data as u64) + 1;
                self.x1[1] = (data as u64) + 1;
                self.x1[2] = (data as u64) + 1;
                self.x1[3] = (data as u64) + 1;
            }
            Register::X2 => {
                self.x2[0] = (data as u64) + 1;
                self.x2[1] = (data as u64) + 1;
                self.x2[2] = (data as u64) + 1;
                self.x2[3] = (data as u64) + 1;
            }
        };
        match Register::from(reg1) {
            Register::ZR => {}
            Register::R0 => self.r0 = data as u64,
            Register::R1 => self.r1 = data as u64,
            Register::R2 => self.r2 = data as u64,
            Register::F0 => self.f0 = (data as f64).to_bits(),
            Register::F1 => self.f1 = (data as f64).to_bits(),
            Register::F2 => self.f2 = (data as f64).to_bits(),
            Register::P0 => self.p0 = data as u64,
            Register::P1 => self.p1 = data as u64,
            Register::P2 => self.p2 = data as u64,
            Register::S0 => {
                self.s0[0] = data as u64;
                self.s0[1] = data as u64;
            }
            Register::S1 => {
                self.s1[0] = data as u64;
                self.s1[1] = data as u64;
            }
            Register::S2 => {
                self.s2[0] = data as u64;
                self.s2[1] = data as u64;
            }
            Register::X0 => {
                self.x0[0] = data as u64;
                self.x0[1] = data as u64;
                self.x0[2] = data as u64;
                self.x0[3] = data as u64;
            }
            Register::X1 => {
                self.x1[0] = data as u64;
                self.x1[1] = data as u64;
                self.x1[2] = data as u64;
                self.x1[3] = data as u64;
            }
            Register::X2 => {
                self.x2[0] = data as u64;
                self.x2[1] = data as u64;
                self.x2[2] = data as u64;
                self.x2[3] = data as u64;
            }
        };
    }
    fn ddec(&mut self, reg0: u8, reg1: u8, data: u16) {
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 = (data as u64) - 1,
            Register::R1 => self.r1 = (data as u64) - 1,
            Register::R2 => self.r2 = (data as u64) - 1,
            Register::F0 => self.f0 = ((data as f64) - 1.0).to_bits(),
            Register::F1 => self.f1 = ((data as f64) - 1.0).to_bits(),
            Register::F2 => self.f2 = ((data as f64) - 1.0).to_bits(),
            Register::P0 => self.p0 = (data as u64) - 1,
            Register::P1 => self.p1 = (data as u64) - 1,
            Register::P2 => self.p2 = (data as u64) - 1,
            Register::S0 => {
                self.s0[0] = (data as u64) - 1;
                self.s0[1] = (data as u64) - 1;
            }
            Register::S1 => {
                self.s1[0] = (data as u64) - 1;
                self.s1[1] = (data as u64) - 1;
            }
            Register::S2 => {
                self.s2[0] = (data as u64) - 1;
                self.s2[1] = (data as u64) - 1;
            }
            Register::X0 => {
                self.x0[0] = (data as u64) - 1;
                self.x0[1] = (data as u64) - 1;
                self.x0[2] = (data as u64) - 1;
                self.x0[3] = (data as u64) - 1;
            }
            Register::X1 => {
                self.x1[0] = (data as u64) - 1;
                self.x1[1] = (data as u64) - 1;
                self.x1[2] = (data as u64) - 1;
                self.x1[3] = (data as u64) - 1;
            }
            Register::X2 => {
                self.x2[0] = (data as u64) - 1;
                self.x2[1] = (data as u64) - 1;
                self.x2[2] = (data as u64) - 1;
                self.x2[3] = (data as u64) - 1;
            }
        };
        match Register::from(reg1) {
            Register::ZR => {}
            Register::R0 => self.r0 = data as u64,
            Register::R1 => self.r1 = data as u64,
            Register::R2 => self.r2 = data as u64,
            Register::F0 => self.f0 = (data as f64).to_bits(),
            Register::F1 => self.f1 = (data as f64).to_bits(),
            Register::F2 => self.f2 = (data as f64).to_bits(),
            Register::P0 => self.p0 = data as u64,
            Register::P1 => self.p1 = data as u64,
            Register::P2 => self.p2 = data as u64,
            Register::S0 => {
                self.s0[0] = data as u64;
                self.s0[1] = data as u64;
            }
            Register::S1 => {
                self.s1[0] = data as u64;
                self.s1[1] = data as u64;
            }
            Register::S2 => {
                self.s2[0] = data as u64;
                self.s2[1] = data as u64;
            }
            Register::X0 => {
                self.x0[0] = data as u64;
                self.x0[1] = data as u64;
                self.x0[2] = data as u64;
                self.x0[3] = data as u64;
            }
            Register::X1 => {
                self.x1[0] = data as u64;
                self.x1[1] = data as u64;
                self.x1[2] = data as u64;
                self.x1[3] = data as u64;
            }
            Register::X2 => {
                self.x2[0] = data as u64;
                self.x2[1] = data as u64;
                self.x2[2] = data as u64;
                self.x2[3] = data as u64;
            }
        };
    }
    fn dneg(&mut self, reg0: u8, reg1: u8, data: u16) {
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 = data.wrapping_neg() as u64,
            Register::R1 => self.r1 = data.wrapping_neg() as u64,
            Register::R2 => self.r2 = data.wrapping_neg() as u64,
            Register::F0 => self.f0 = (data.wrapping_neg() as f64).to_bits(),
            Register::F1 => self.f1 = (data.wrapping_neg() as f64).to_bits(),
            Register::F2 => self.f2 = (data.wrapping_neg() as f64).to_bits(),
            Register::P0 => self.p0 = data.wrapping_neg() as u64,
            Register::P1 => self.p1 = data.wrapping_neg() as u64,
            Register::P2 => self.p2 = data.wrapping_neg() as u64,
            Register::S0 => {
                self.s0[0] = data.wrapping_neg() as u64;
                self.s0[1] = data.wrapping_neg() as u64;
            }
            Register::S1 => {
                self.s1[0] = data.wrapping_neg() as u64;
                self.s1[1] = data.wrapping_neg() as u64;
            }
            Register::S2 => {
                self.s2[0] = data.wrapping_neg() as u64;
                self.s2[1] = data.wrapping_neg() as u64;
            }
            Register::X0 => {
                self.x0[0] = data.wrapping_neg() as u64;
                self.x0[1] = data.wrapping_neg() as u64;
                self.x0[2] = data.wrapping_neg() as u64;
                self.x0[3] = data.wrapping_neg() as u64;
            }
            Register::X1 => {
                self.x1[0] = data.wrapping_neg() as u64;
                self.x1[1] = data.wrapping_neg() as u64;
                self.x1[2] = data.wrapping_neg() as u64;
                self.x1[3] = data.wrapping_neg() as u64;
            }
            Register::X2 => {
                self.x2[0] = data.wrapping_neg() as u64;
                self.x2[1] = data.wrapping_neg() as u64;
                self.x2[2] = data.wrapping_neg() as u64;
                self.x2[3] = data.wrapping_neg() as u64;
            }
        };
        match Register::from(reg1) {
            Register::ZR => {}
            Register::R0 => self.r0 = data as u64,
            Register::R1 => self.r1 = data as u64,
            Register::R2 => self.r2 = data as u64,
            Register::F0 => self.f0 = (data as f64).to_bits(),
            Register::F1 => self.f1 = (data as f64).to_bits(),
            Register::F2 => self.f2 = (data as f64).to_bits(),
            Register::P0 => self.p0 = data as u64,
            Register::P1 => self.p1 = data as u64,
            Register::P2 => self.p2 = data as u64,
            Register::S0 => {
                self.s0[0] = data as u64;
                self.s0[1] = data as u64;
            }
            Register::S1 => {
                self.s1[0] = data as u64;
                self.s1[1] = data as u64;
            }
            Register::S2 => {
                self.s2[0] = data as u64;
                self.s2[1] = data as u64;
            }
            Register::X0 => {
                self.x0[0] = data as u64;
                self.x0[1] = data as u64;
                self.x0[2] = data as u64;
                self.x0[3] = data as u64;
            }
            Register::X1 => {
                self.x1[0] = data as u64;
                self.x1[1] = data as u64;
                self.x1[2] = data as u64;
                self.x1[3] = data as u64;
            }
            Register::X2 => {
                self.x2[0] = data as u64;
                self.x2[1] = data as u64;
                self.x2[2] = data as u64;
                self.x2[3] = data as u64;
            }
        };
    }
    fn swap(&mut self, reg0: u8, reg1: u8, data: u16) {
        let (val, val2, val3, val4) = self.get_vals(reg0);
        let (val5, val6, val7, val8) = self.get_vals(reg0);
        match Register::from(reg0) {
            Register::ZR => {}
            Register::R0 => self.r0 = val5,
            Register::R1 => self.r1 = val5,
            Register::R2 => self.r2 = val5,
            Register::F0 => self.f0 = val5,
            Register::F1 => self.f1 = val5,
            Register::F2 => self.f2 = val5,
            Register::P0 => self.p0 = val5,
            Register::P1 => self.p1 = val5,
            Register::P2 => self.p2 = val5,
            Register::S0 => {
                self.s0[0] = val5;
                self.s0[1] = val6;
            }
            Register::S1 => {
                self.s1[0] = val5;
                self.s1[1] = val6;
            }
            Register::S2 => {
                self.s2[0] = val5;
                self.s2[1] = val6;
            }
            Register::X0 => {
                self.x0[0] = val5;
                self.x0[1] = val6;
                self.x0[2] = val7;
                self.x0[3] = val8;
            }
            Register::X1 => {
                self.x1[0] = val5;
                self.x1[1] = val6;
                self.x1[2] = val7;
                self.x1[3] = val8;
            }
            Register::X2 => {
                self.x2[0] = val5;
                self.x2[1] = val6;
                self.x2[2] = val7;
                self.x2[3] = val8;
            }
        };
        match Register::from(reg1) {
            Register::ZR => {}
            Register::R0 => self.r0 = val,
            Register::R1 => self.r1 = val,
            Register::R2 => self.r2 = val,
            Register::F0 => self.f0 = val,
            Register::F1 => self.f1 = val,
            Register::F2 => self.f2 = val,
            Register::P0 => self.p0 = val,
            Register::P1 => self.p1 = val,
            Register::P2 => self.p2 = val,
            Register::S0 => {
                self.s0[0] = val;
                self.s0[1] = val2;
            }
            Register::S1 => {
                self.s1[0] = val;
                self.s1[1] = val2;
            }
            Register::S2 => {
                self.s2[0] = val;
                self.s2[1] = val2;
            }
            Register::X0 => {
                self.x0[0] = val;
                self.x0[1] = val2;
                self.x0[2] = val3;
                self.x0[3] = val4;
            }
            Register::X1 => {
                self.x1[0] = val;
                self.x1[1] = val2;
                self.x1[2] = val3;
                self.x1[3] = val4;
            }
            Register::X2 => {
                self.x2[0] = val;
                self.x2[1] = val2;
                self.x2[2] = val3;
                self.x2[3] = val4;
            }
        };
    }
    fn jump_op(&mut self, data: u16) {
        if data == 0 {
            return;
        } else {
            self.pc = self.pc.wrapping_add_signed((data as i16) as i64);
        }
    }
    fn get_vals(&self, reg0: u8) -> (u64, u64, u64, u64) {
        let val = match Register::from(reg0) {
            Register::ZR => 0,
            Register::R0 => self.r0,
            Register::R1 => self.r1,
            Register::R2 => self.r2,
            Register::F0 => self.f0,
            Register::F1 => self.f1,
            Register::F2 => self.f2,
            Register::P0 => self.p0,
            Register::P1 => self.p1,
            Register::P2 => self.p2,
            Register::S0 => self.s0[0],
            Register::S1 => self.s1[0],
            Register::S2 => self.s2[0],
            Register::X0 => self.x0[0],
            Register::X1 => self.x1[0],
            Register::X2 => self.x2[0],
        };
        let val2 = match Register::from(reg0) {
            Register::ZR => 0,
            Register::R0 => 0,
            Register::R1 => 0,
            Register::R2 => 0,
            Register::F0 => 0,
            Register::F1 => 0,
            Register::F2 => 0,
            Register::P0 => 0,
            Register::P1 => 0,
            Register::P2 => 0,
            Register::S0 => self.s0[1],
            Register::S1 => self.s1[1],
            Register::S2 => self.s2[1],
            Register::X0 => self.x0[1],
            Register::X1 => self.x1[1],
            Register::X2 => self.x2[1],
        };
        let val3 = match Register::from(reg0) {
            Register::ZR => 0,
            Register::R0 => 0,
            Register::R1 => 0,
            Register::R2 => 0,
            Register::F0 => 0,
            Register::F1 => 0,
            Register::F2 => 0,
            Register::P0 => 0,
            Register::P1 => 0,
            Register::P2 => 0,
            Register::S0 => 0,
            Register::S1 => 0,
            Register::S2 => 0,
            Register::X0 => self.x0[2],
            Register::X1 => self.x1[2],
            Register::X2 => self.x2[2],
        };
        let val4 = match Register::from(reg0) {
            Register::ZR => 0,
            Register::R0 => 0,
            Register::R1 => 0,
            Register::R2 => 0,
            Register::F0 => 0,
            Register::F1 => 0,
            Register::F2 => 0,
            Register::P0 => 0,
            Register::P1 => 0,
            Register::P2 => 0,
            Register::S0 => 0,
            Register::S1 => 0,
            Register::S2 => 0,
            Register::X0 => self.x0[3],
            Register::X1 => self.x1[3],
            Register::X2 => self.x2[3],
        };
        (val, val2, val3, val4)
    }
}
