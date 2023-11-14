#[derive(Debug)]
pub enum Register {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
    Dt,
    St,
    I,
}

#[derive(Debug)]
pub enum Data {
    Reg(Register),
    Int(u16),
}

#[derive(Debug)]
pub enum Ops {
    Move(Register, Data),
    Draw(Register, Register, u8),
    Jump(&'static str),
    Add(Register, Data),
}

#[derive(Debug)]
pub enum Assembly {
    Instruction(Ops),
    Label(&'static str),
}
