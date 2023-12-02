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
    Draw(Register, Register, u16),
    Jump(&'static str),
    Add(Register, Data),
    SkipIfEqual(Register, Data),
    ClearScreen,
    LoadFontAddress(Register),
    Call(&'static str),
    Rand(Register, u8),
    SkipIfKeyNotPress(Register),
    And(Register,Register),
    SkipIfNotEqual(Register,Data),
    Sub(Register,Register)
}

#[derive(Debug)]
pub enum Assembly {
    Instruction(Ops),
    Label(&'static str),
}
