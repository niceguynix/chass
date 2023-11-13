use crate::instructions::{Assembly, Data, Ops, Register};

pub struct Linker {
    m_code: Vec<u16>,
    ops: Vec<Assembly>,
}

impl Linker {
    pub fn new(asm: Vec<Assembly>) -> Self {
        Self {
            m_code: Vec::new(),
            ops: asm,
        }
    }

    pub fn link(&mut self) {
        for i in self.ops.iter() {
            match i {
                Assembly::Instruction(op) => self.m_code.push(Self::covert_to_code(op)),
                _ => (),
            }
        }
    }

    fn covert_to_code(asm: &Ops) -> u16 {
        let c = match asm {
            Ops::Move(reg, data) => Self::encode_move(reg, data),
            Ops::Draw(r1, r2, l) => Self::encode_draw(r1, r2, l),
            _ => panic!("??"),
        };

        Self::convert(c)
    }

    fn encode_draw(r1: &Register, r2: &Register, i: &u8) -> [u8; 4] {
        [
            0xD,
            Self::get_register_code(r1),
            Self::get_register_code(r2),
            *i,
        ]
    }

    fn convert(arr: [u8; 4]) -> u16 {
        let mut n = 0_u16;
        n = arr[0] as u16;
        n <<= 4;
        n |= arr[1] as u16;
        n <<= 4;
        n |= arr[2] as u16;
        n <<= 4;
        n |= arr[3] as u16;
        n
    }

    fn encode_move(reg: &Register, data: &Data) -> [u8; 4] {
        match (reg, data) {
            (Register::I, Data::Int(i)) => {
                let x = [
                    0xA,
                    ((i & 0xF00) >> 8) as u8,
                    ((i & 0xF0) >> 4) as u8,
                    (i & 0xF) as u8,
                ];
                return x;
            }
            _ => (),
        };

        let int = match data {
            Data::Int(literal) => [
                
                6,
                Self::get_register_code(reg),
                (literal >> 4) as u8,
                (literal & 0xF) as u8,
            ],
            Data::Reg(reg2) => [
                8,
                Self::get_register_code(reg),
                Self::get_register_code(reg2),
                0,
            ],
        };
        int
    }

    fn get_register_code(reg: &Register) -> u8 {
        match reg {
            Register::V0 => 0,
            Register::V1 => 1,
            Register::V2 => 2,
            Register::V3 => 3,
            Register::V4 => 4,
            Register::V5 => 5,
            Register::V6 => 6,
            Register::V7 => 7,
            Register::V8 => 8,
            Register::V9 => 9,
            Register::VA => 10,
            Register::VB => 11,
            Register::VC => 12,
            Register::VD => 13,
            Register::VE => 13,
            Register::VF => 14,
            _ => panic!("cant"),
        }
    }

    pub fn get_code(&self) -> Vec<u8> {
        let mut code = Vec::new();
        for i in self.m_code.iter() {
            let mut t = i.clone();
            let mut fh = t & 0xFF00;
            fh >>= 8;
            let t = i & 0x00FF;
            println!("{:0x?}", i);
            code.push(fh as u8);
            code.push((i & 0xFF) as u8);
        }

        code
    }
}