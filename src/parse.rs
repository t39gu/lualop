use crate::bytecode::ByteCode;
use crate::lex::{Lex, Token};
use crate::value::Value;
use std::fs::File;

#[derive(Debug)]
pub struct ParseProto {
    pub byte_codes: Vec<ByteCode>,
    pub constants: Vec<Value>,
}

pub fn load(input: File) -> ParseProto {
    let mut byte_codes = Vec::new();
    let mut constants = Vec::new();
    let mut lex = Lex::new(input);

    loop {
        match lex.next() {
            Token::Name(name) => {
                constants.push(Value::String(name));
                byte_codes.push(ByteCode::GetGlobal(0, (constants.len() - 1) as u8));

                if let Token::String(s) = lex.next() {
                    constants.push(Value::String(s));
                    byte_codes.push(ByteCode::Load(1, (constants.len() - 1) as u8));
                    byte_codes.push(ByteCode::Call(0, 1));
                } else {
                    panic!("expected string");
                }
            }
            Token::Eos => break,
            t => panic!("unexpected token: {t:?}"),
        }
    }

    dbg!(&byte_codes);
    dbg!(&constants);
    ParseProto {
        byte_codes,
        constants,
    }
}
