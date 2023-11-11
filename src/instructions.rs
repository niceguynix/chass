enum Register {
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
    VF,
    Dt,
    St,
    I,
}

enum Data {
    Reg(Register),
    Int(i8),
}

enum Ops {
    Move(Register,Data),
}
