use super::file_reader::FileReader;
use anyhow::Result;

#[derive(Debug)]
pub enum ByteCode {
    IConst(u8),
    IReturn,
    New(u16),
    InvokeSpecial(u16),
    Duplicate,
    AReturn,
    ALoad(u8),
    Lload(u8),
    PutStatic(u16),
    Ldc(u8),
    Return,
    LReturn,
    LAdd,
    Generic(u8),
}

impl ByteCode {
    pub fn parse(file: &mut FileReader) -> Result<(ByteCode, u32)> {
        let opcode = file.read_u1()?;
        let (code, len) = match opcode {
            0x02..=0x08 => (ByteCode::IConst(opcode - 0x03), 1),
            0xac => (ByteCode::IReturn, 1),
            0xbb => (ByteCode::New(file.read_u2_to_u16()?), 3),
            0x59 => (ByteCode::Duplicate, 1),
            0xb7 => (ByteCode::InvokeSpecial(file.read_u2_to_u16()?), 3),
            0xb0 => (ByteCode::AReturn, 1),
            0xb1 => (ByteCode::Return, 1),
            0xad => (ByteCode::LReturn, 1),
            0x12 => (ByteCode::Ldc(file.read_u1()?), 2),
            0xb3 => (ByteCode::PutStatic(file.read_u2_to_u16()?), 3),
            0x2a..=0x2d => (ByteCode::ALoad(opcode - 0x2a), 1),
            0x1e..=0x21 => (ByteCode::Lload(opcode - 0x1e), 1),
            0x61 => (ByteCode::LAdd, 1),
            _ => (ByteCode::Generic(opcode), 1),
        };
        Ok((code, len))
    }

    pub fn to_string(&self) -> String {
        match self {
            ByteCode::IConst(u8) => format!("IConst({})", u8),
            ByteCode::IReturn => "IReturn".to_string(),
            ByteCode::New(u16) => format!("New(0x{:x?})", u16),
            ByteCode::Duplicate => "Duplicate".to_string(),
            ByteCode::InvokeSpecial(u16) => format!("InvokeSpecial(0x{:x?})", u16),
            ByteCode::AReturn => "Reference Return (areturn)".to_string(),
            ByteCode::Return => "Return void (return)".to_string(),
            ByteCode::LReturn => "Long Return (lreturn)".to_string(),
            ByteCode::Ldc(u8) => format!("Ldc({})", u8),
            ByteCode::ALoad(u8) => format!("ALoad({})", u8),
            ByteCode::Lload(u8) => format!("Lload({})", u8),
            ByteCode::PutStatic(u16) => format!("PutStatic(0x{:x?})", u16),
            ByteCode::LAdd => "LAdd".to_string(),
            ByteCode::Generic(u8) => format!("Generic(0x{:x?})", u8),
        }
    }
}
