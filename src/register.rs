pub(crate) enum Register {
    ZR, //Zero Register
    R0, //General Purpose Register
    R1, //General Purpose Register
    R2, //General Purpose Register
    F0, //Fp Register
    F1, //Fp Register
    F2, //Fp Register
    P0, //Pointer Register
    P1, //Pointer Register
    P2, //Pointer Register
    S0, //SIMD Register(128 bit)
    S1, //SIMD Register(128 bit)
    S2, //SIMD Register(128 bit)
    X0, //SIMD Register(256 bit)
    X1, //SIMD Register(256 bit)
    X2, //SIMD Register(256 bit)
}

impl From<Register> for u8 {
    fn from(val: Register) -> Self {
        val as u8
    }
}
impl From<u8> for Register {
    fn from(val: u8) -> Self {
        match val {
            0 => Register::ZR,
            1 => Register::R0,
            2 => Register::R1,
            3 => Register::R2,
            4 => Register::F0,
            5 => Register::F1,
            6 => Register::F2,
            7 => Register::P0,
            8 => Register::P1,
            9 => Register::P2,
            10 => Register::S0,
            11 => Register::S1,
            12 => Register::S2,
            13 => Register::X0,
            14 => Register::X1,
            15 => Register::X2,
            _ => Register::ZR,
        }
    }
}
impl From<Register> for &str {
    fn from(val: Register) -> Self {
        match val {
            Register::ZR => "zr",
            Register::R0 => "r0",
            Register::R1 => "r1",
            Register::R2 => "r2",
            Register::F0 => "f0",
            Register::F1 => "f1",
            Register::F2 => "f2",
            Register::P0 => "p0",
            Register::P1 => "p1",
            Register::P2 => "p2",
            Register::S0 => "s0",
            Register::S1 => "s1",
            Register::S2 => "s2",
            Register::X0 => "x0",
            Register::X1 => "x1",
            Register::X2 => "x2",
        }
    }
}
impl From<&str> for Register {
    fn from(val: &str) -> Self {
        match val {
            "zr" => Register::ZR,
            "r0" => Register::R0,
            "r1" => Register::R1,
            "r2" => Register::R2,
            "f0" => Register::F0,
            "f1" => Register::F1,
            "f2" => Register::F2,
            "p0" => Register::P0,
            "p1" => Register::P1,
            "p2" => Register::P2,
            "s0" => Register::S0,
            "s1" => Register::S1,
            "s2" => Register::S2,
            "x0" => Register::X0,
            "x1" => Register::X1,
            "x2" => Register::X2,
            _ => Register::ZR,
        }
    }
}
