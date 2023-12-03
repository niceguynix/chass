use crate::instructions::{Assembly, Data, Ops, Register};
use std::collections::HashMap;

const START_ADDR: i32 = 0x200;

#[derive(Debug)]
pub struct Linker {
    m_code: Vec<u16>,
    ops: Vec<Assembly>,
    label_addr: HashMap<&'static str, i32>,
}

impl Linker {
    pub fn new(asm: Vec<Assembly>) -> Self {
        Self {
            m_code: Vec::new(),
            ops: asm,
            label_addr: HashMap::new(),
        }
    }

    pub fn link(&mut self) {
        for i in self.ops.iter() {
            match i {
                Assembly::Instruction(op) => self.m_code.push(self.covert_to_code(op)),
                _ => (),
            }
        }
    }

    fn covert_to_code(&self, asm: &Ops) -> u16 {
        println!("{asm:?}");
        let c = match asm {
            Ops::Move(reg, data) => Self::encode_move(reg, data),
            Ops::Draw(r1, r2, l) => Self::encode_draw(r1, r2, &(*l as u8)),
            Ops::Jump(label) => self.encode_jump(label),
            Ops::Add(reg, data) => Self::encode_add(reg, data),
            Ops::SkipIfEqual(reg, data) => Self::encode_skip_if_eq(reg, data),
            Ops::ClearScreen => [0, 0, 0xE, 0],
            Ops::LoadFontAddress(reg) => Self::encode_load_font_address(reg),
            Ops::Call(d) => Self::encode_call(self, *d),
            Ops::Rand(reg, literal) => Self::encode_rand(reg, literal),
            Ops::SkipIfKeyNotPress(reg) => Self::encode_skip_if_key_not_pressed(reg),
            Ops::And(r1, r2) => Self::encode_and(r1, r2),
            Ops::SkipIfNotEqual(reg, data) => Self::encode_skip_if_not_equal(reg, data),
            Ops::Sub(reg1, reg2) => Self::encode_sub(reg1, reg2),
            Ops::Bcd(reg) => Self::encode_bcd(reg),
            Ops::Store(reg) => Self::encode_store(reg),
            Ops::Return => Self::encode_return(),
            Ops::NoOp => [0, 0, 0, 0],
        };

        Self::convert(c)
    }

    fn encode_return() -> [u8; 4] {
        [0, 0, 0xE, 0xE]
    }

    fn encode_store(reg: &Register) -> [u8; 4] {
        [0xF, Self::get_register_code(reg), 6, 5]
    }

    fn encode_bcd(reg: &Register) -> [u8; 4] {
        [0xF, Self::get_register_code(reg), 3, 3]
    }

    fn encode_sub(reg1: &Register, reg2: &Register) -> [u8; 4] {
        [
            8,
            Self::get_register_code(reg1),
            Self::get_register_code(reg2),
            5,
        ]
    }

    fn encode_skip_if_not_equal(reg: &Register, data: &Data) -> [u8; 4] {
        match data {
            Data::Int(i) => [
                4,
                Self::get_register_code(reg),
                ((i & 0xF0) >> 4) as u8,
                (i & 0xF) as u8,
            ],
            _ => unimplemented!(),
        }
    }

    fn encode_and(reg1: &Register, reg2: &Register) -> [u8; 4] {
        [
            8,
            Self::get_register_code(reg1),
            Self::get_register_code(reg2),
            2,
        ]
    }

    fn encode_skip_if_key_not_pressed(reg: &Register) -> [u8; 4] {
        [0xE, Self::get_register_code(reg), 0xA, 1]
    }

    fn encode_rand(reg: &Register, data: &u8) -> [u8; 4] {
        [
            0xC,
            Self::get_register_code(reg),
            (data & 0xF0) >> 4,
            (data & 0xF),
        ]
    }

    fn encode_call(&self, d: &str) -> [u8; 4] {
        let d = self.label_addr.get(d).expect("unexpected label found");
        [
            2,
            ((d & 0xF00) >> 8) as u8,
            ((d & 0xF0) >> 4) as u8,
            (d & 0xF) as u8,
        ]
    }

    fn encode_load_font_address(reg: &Register) -> [u8; 4] {
        [0xF, Self::get_register_code(reg), 2, 9]
    }

    fn encode_skip_if_eq(reg: &Register, data: &Data) -> [u8; 4] {
        match data {
            Data::Int(l) => [
                3,
                Self::get_register_code(reg),
                ((l & 0xF0) >> 4) as u8,
                (l & 0xF) as u8,
            ],
            Data::Reg(r2) => [
                5,
                Self::get_register_code(reg),
                Self::get_register_code(r2),
                0,
            ],
        }
    }

    fn encode_add(reg: &Register, data: &Data) -> [u8; 4] {
        if let (Register::I, Data::Reg(r2)) = (reg, data) {
            return [0xF, Self::get_register_code(r2), 1, 0xE];
        }

        match data {
            Data::Int(literal) => [
                7,
                Self::get_register_code(reg),
                ((literal & 0xF0) >> 4) as u8,
                (literal & 0xF) as u8,
            ],
            Data::Reg(r) => [
                8,
                Self::get_register_code(reg),
                Self::get_register_code(r),
                4,
            ],
        }
    }

    fn encode_jump(&self, label: &str) -> [u8; 4] {
        let addr = self.label_addr.get(label).expect("Not a balid label");
        let i = *addr;
        [
            1,
            ((i & 0xF00) >> 8) as u8,
            ((i & 0xF0) >> 4) as u8,
            (i & 0xF) as u8,
        ]
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
        let mut n;
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
            (Register::Dt, Data::Reg(reg)) => {
                return [0xF, Self::get_register_code(reg), 1, 5];
            }
            (Register::St, Data::Reg(reg)) => {
                return [0xF, Self::get_register_code(reg), 1, 8];
            }
            _ => (),
        };

        if let Data::Reg(Register::Dt) = data {
            return [0xF, Self::get_register_code(reg), 0, 7];
        }

        match data {
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
        }
    }

    fn get_register_code(reg: &Register) -> u8 {
        println!("{reg:?}");
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
            Register::VE => 14,
            Register::VF => 15,
            _ => panic!("cant"),
        }
    }

    pub fn get_code(&self) -> Vec<u8> {
        let mut code = Vec::new();
        for i in self.m_code.iter() {
            let t = *i;
            let mut fh = t & 0xFF00;
            fh >>= 8;
            let _t = i & 0x00FF;
            println!("{:0x?}", i);
            code.push(fh as u8);
            code.push((i & 0xFF) as u8);
        }

        code
    }

    pub fn set_up_labels(&mut self) {
        let mut lables = HashMap::new();
        let mut addr = 0;
        for i in self.ops.iter() {
            if let Assembly::Label(label) = *i {
                if lables.contains_key(label) {
                    panic!("Duplicate Label found");
                }
                lables.insert(label, START_ADDR + addr);
            } else {
                addr += 2;
            }
        }
        self.label_addr = lables;
    }
}
