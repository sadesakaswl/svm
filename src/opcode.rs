pub(crate) enum Opcode {
    Nop,    //nop reg0 reg1 jump_index(Clears selected registers)
    Set,    //set reg0 index data
    Get,    //get reg0 index jump_index (gets from reg0+1)
    Update, //update reg0 reg1 jump_index
    Delete, //delete reg0 reg1 jump_index (clears reg1)

    Swap, //swap reg0 reg1 jump_index

    And,  //and reg0 reg1 jump_index (reg0&=reg1)
    Or,   //or reg0 reg1 jump_index (reg0|=reg1)
    Xor,  //xor reg0 reg1 jump_index (reg0^=reg1)
    Not,  //not reg0 reg1 jump_index (reg0=~reg1)
    Shl,  //shl reg0 reg1 jump_index (reg0<<=reg1)
    Shr,  //shr reg0 reg1 jump_index (reg0>>=reg1)
    Dand, //dand reg0 reg1 data (reg0=reg1&data)
    Dor,  //dor reg0 reg1 data (reg0=reg1|data)
    Dxor, //dxor reg0 reg1 data (reg0=reg1^data)
    Dnot, //dnot reg0 reg1 jump_index (reg0=~reg0) (reg1=~reg1)
    Dshl, //dshl reg0 reg1 data (reg0=reg1<<data)
    Dshr, //dshr reg0 reg1 data (reg0=reg1>>data)

    Add,  //add reg0 reg1 jump_index (reg0+=reg1)
    Sub,  //sub reg0 reg1 jump_index (reg0-=reg1)
    Mul,  //mul reg0 reg1 jump_index (reg0*=reg1)
    Div,  //div reg0 reg1 jump_index (reg0/=reg1)
    Mod,  //mod reg0 reg1 jump_index (reg0%=reg1)
    Inc,  //inc reg0 reg1 jump_index (reg0=reg0+1) (reg1=reg1-1)
    Dec,  //dec reg0 reg1 jump_index (reg0=reg0-1) (reg1=reg1-1)
    Neg,  //neg reg0 reg1 jump_index (reg0=-reg0) (reg1=-reg1)
    Dadd, //dadd reg0 reg1 data (reg0=reg1+data)
    Dsub, //dsub reg0 reg1 data (reg0=reg1-data)
    Dmul, //dmul reg0 reg1 data (reg0=reg1*data)
    Ddiv, //ddiv reg0 reg1 data (reg0=reg1/data)
    Dmod, //dmod reg0 reg1 data (reg0=reg1%data)
    Dinc, //dinc reg0 reg1 data (reg0=data+1) (reg1=data)
    Ddec, //ddec reg0 reg1 data (reg0=data-1) (reg1=data)
    Dneg, //dneg reg0 reg1 data (reg0=-data) (reg1=data)
}
impl From<u8> for Opcode {
    fn from(_: u8) -> Self {
        todo!()
    }
}
impl From<Opcode> for u8 {
    fn from(val: Opcode) -> Self {
        val as u8
    }
}
impl From<&str> for Opcode {
    fn from(_: &str) -> Self {
        todo!()
    }
}
impl From<Opcode> for &str {
    fn from(_: Opcode) -> Self {
        todo!()
    }
}
