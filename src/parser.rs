use crate::instructions::{self, Assembly, Data, Ops, Register};

#[derive(Debug)]
pub struct Parser {
    data: &'static str,
    p: usize,
    pub ops: Vec<Assembly>,
}

impl Parser {
    pub fn new(odata: String) -> Self {
        Self {
            data: Box::leak(odata.into_boxed_str()),
            p: 0,
            ops: Vec::new(),
        }
    }

    pub fn get_next_token(&mut self) -> Option<&'static str> {
        while let Some(i) = self.data.chars().nth(self.p) {
            if !i.is_whitespace() {
                break;
            }
            self.p += 1;
        }

        let start = self.p;

        while let Some(i) = self.data.chars().nth(self.p) {
            if i.is_whitespace() {
                break;
            }
            self.p += 1;
        }

        let end = self.p;
        if start >= self.data.len() {
            return None;
        }
        // println!("start:{start} end:{end}");
        Some(&self.data[start..end])
    }

    fn get_instrution(&mut self) -> Option<Assembly> {
        let token = self.get_next_token();

        let token = match token {
            Some(tok) => tok,
            _ => return None,
        };

        let asm = match token {
            label if label.starts_with(':') => Assembly::Label(&label[1..]),
            "mov" => Assembly::Instruction(instructions::Ops::Move(
                self.get_register(),
                self.get_data(),
            )),
            "draw" => Assembly::Instruction(instructions::Ops::Draw(
                self.get_register(),
                self.get_register(),
                self.get_literal(),
            )),
            "jump" => Assembly::Instruction(instructions::Ops::Jump(self.get_label())),
            "add" => {
                Assembly::Instruction(instructions::Ops::Add(self.get_register(), self.get_data()))
            }
            "se" => Assembly::Instruction(instructions::Ops::SkipIfEqual(
                self.get_register(),
                self.get_data(),
            )),
            "clr" => Assembly::Instruction(Ops::ClearScreen),
            "ldfadr" => Assembly::Instruction(Ops::LoadFontAddress(self.get_register())),
            "call" => Assembly::Instruction(Ops::Call(self.get_label())),
            "getrand" => {
                Assembly::Instruction(Ops::Rand(self.get_register(), self.get_literal() as u8))
            }
            "sknp" => Assembly::Instruction(Ops::SkipIfKeyNotPress(self.get_register())),
            "and" => Assembly::Instruction(Ops::And(self.get_register(), self.get_register())),
            "sne" => Assembly::Instruction(Ops::SkipIfNotEqual(self.get_register(), self.get_data())),
            "sub" => Assembly::Instruction(Ops::Sub(self.get_register(), self.get_register())),
            "bcd" => Assembly::Instruction(Ops::Bcd(self.get_register())),
            "store" => Assembly::Instruction(Ops::Store(self.get_register())),
            "ret" => Assembly::Instruction(Ops::Return),
            _ => panic!("unrecogninzed instruction"),
        };

        Some(asm)
    }

    fn get_label(&mut self) -> &'static str {
        let label = self.get_next_token().expect("Need a lable");
        if !label.starts_with('.') {
            panic!("Not a valid label");
        }
        &label[1..]
    }

    pub fn load_instructions(&mut self) {
        while let Some(instruction) = self.get_instrution() {
            self.ops.push(instruction);
        }
    }

    fn to_register(&self, token: &str) -> Option<Register> {
        let register = match token {
            "rv0" => Register::V0,
            "rv1" => Register::V1,
            "rv2" => Register::V2,
            "rv3" => Register::V3,
            "rv4" => Register::V4,
            "rv5" => Register::V5,
            "rv6" => Register::V6,
            "rv7" => Register::V7,
            "rv8" => Register::V8,
            "rv9" => Register::V9,
            "rva" => Register::VA,
            "rvb" => Register::VB,
            "rvc" => Register::VC,
            "rvd" => Register::VD,
            "rve" => Register::VE,
            "rvf" => Register::VF,
            "irg" => Register::I,
            "rdt" => Register::Dt,
            "rst" => Register::St,
            _ => return None,
        };
        Some(register)
    }

    fn to_literal(&self, token: &str) -> Option<u16> {
        token.parse().ok()
    }

    fn get_data(&mut self) -> Data {
        let token = self
            .get_next_token()
            .expect("Needed a register or integer here");

        if let Some(reg) = self.to_register(token) {
            return Data::Reg(reg);
        }

        if let Some(literal) = self.to_literal(token) {
            return Data::Int(literal as u16);
        }

        panic!("Not a valid register or literal");
    }

    fn get_register(&mut self) -> Register {
        let token = self.get_next_token().expect("Needed a register");
        self.to_register(token).expect("Not a register")
    }
    fn get_literal(&mut self) -> u16 {
        let token = self.get_next_token().expect("Needed a integer");
        self.to_literal(token).expect("Not a integer")
    }
}
