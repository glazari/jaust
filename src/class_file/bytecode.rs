use super::file_reader::FileReader;
use anyhow::Result;

#[derive(Debug)]
pub enum ByteCode {
    IConstn(u8),        // Push int constant
    LConstn(u8),        // Push long constant
    Lcmp,               // Compare long
    IReturn,            // Return int from method
    New(u16),           // create new object
    InvokeSpecial(u16), // Invoke instance method; special handling for superclass, private, and instance initialization method invocations
    InvokeVirtual(u16), // Invoke instance method; dispatch based on class
    InvokeDynamic(u16), // Invoke dynamic method
    Duplicate,          // Duplicate the top operand stack value
    AReturn,            // Return reference from method
    ALoad(u8),          // Load reference from local variable
    Lload(u8),          // Load long from local variable
    ILoad(u8),          // Load int from local variable
    AStore(u8),         // Store reference into local variable
    LStore(u8),         // Store long into local variable
    PutStatic(u16),     // Set static field in class
    Ldc(u8),            // Push item from run-time constant pool
    Return,             // Return void
    LReturn,            // Return long
    LAdd,               // Add long
    L2i,                // Convert long to int
    I2L,                // Convert int to long
    Athrow,             // Throw exception or error
    Ifeq(i16),          // Branch if int value = 0
    Ifne(i16),          // Branch if int value != 0
    Iflt(i16),          // Branch if int value < 0
    Ifge(i16),          // Branch if int value >= 0
    Ifgt(i16),          // Branch if int value > 0
    Ifle(i16),          // Branch if int value <= 0
    PutField(u16),      // Set field in object
    GetField(u16),      // Fetch field from object


    Generic(u8),
}

impl ByteCode {
    pub fn parse(file: &mut FileReader) -> Result<(ByteCode, u32)> {
        let opcode = file.read_u1()?;
        let (code, len) = match opcode {
            0x02..=0x08 => (ByteCode::IConstn(opcode - 0x03), 1),
            0x09..=0x0a => (ByteCode::IConstn(opcode - 0x09), 1),
            0x94 => (ByteCode::Lcmp, 1),
            0xac => (ByteCode::IReturn, 1),
            0xbb => (ByteCode::New(file.read_u2_to_u16()?), 3),
            0x59 => (ByteCode::Duplicate, 1),
            0xb7 => (ByteCode::InvokeSpecial(file.read_u2_to_u16()?), 3),
            0xb6 => (ByteCode::InvokeVirtual(file.read_u2_to_u16()?), 3),
            0xba => {
                let method_index = file.read_u2_to_u16()?;
                assert_eq!(file.read_u1()?, 0);
                assert_eq!(file.read_u1()?, 0);
                (ByteCode::InvokeDynamic(method_index), 5)
            }, 
            0xb0 => (ByteCode::AReturn, 1),
            0xb1 => (ByteCode::Return, 1),
            0xad => (ByteCode::LReturn, 1),
            0x1a..=0x1d => (ByteCode::ILoad(opcode - 0x1a), 1),
            0x12 => (ByteCode::Ldc(file.read_u1()?), 2),
            0xb3 => (ByteCode::PutStatic(file.read_u2_to_u16()?), 3),
            0x2a..=0x2d => (ByteCode::ALoad(opcode - 0x2a), 1),
            0x1e..=0x21 => (ByteCode::Lload(opcode - 0x1e), 1),
            0x16 => (ByteCode::Lload(file.read_u1()?), 2),
            0x61 => (ByteCode::LAdd, 1),
            0x88 => (ByteCode::L2i, 1),
            0x85 => (ByteCode::I2L, 1),
            0x4b..=0x4e => (ByteCode::AStore(opcode - 0x4b), 1),
            0x37 => (ByteCode::LStore(file.read_u1()?), 2),
            0xbf => (ByteCode::Athrow, 1),
            0x99 => (ByteCode::Ifeq(file.read_i16()?), 3),
            0x9a => (ByteCode::Ifne(file.read_i16()?), 3),
            0x9b => (ByteCode::Iflt(file.read_i16()?), 3),
            0x9c => (ByteCode::Ifge(file.read_i16()?), 3),
            0x9d => (ByteCode::Ifgt(file.read_i16()?), 3),
            0x9e => (ByteCode::Ifle(file.read_i16()?), 3),
            0xb5 => (ByteCode::PutField(file.read_u2_to_u16()?), 3),
            0xb4 => (ByteCode::GetField(file.read_u2_to_u16()?), 3),
            _ => (ByteCode::Generic(opcode), 1),
        };
        Ok((code, len))
    }

    pub fn to_string(&self) -> String {
        match self {
            ByteCode::IConstn(u8) => format!("IConst({})", u8),
            ByteCode::LConstn(u8) => format!("LConst({})", u8),
            ByteCode::Lcmp => "Lcmp".to_string(),
            ByteCode::IReturn => "IReturn".to_string(),
            ByteCode::New(u16) => format!("New(0x{:x?})", u16),
            ByteCode::Duplicate => "Duplicate".to_string(),
            ByteCode::InvokeSpecial(u16) => format!("InvokeSpecial(0x{:x?})", u16),
            ByteCode::InvokeVirtual(u16) => format!("InvokeVirtual(0x{:x?})", u16),
            ByteCode::InvokeDynamic(u16) => format!("InvokeDynamic(0x{:x?})", u16),
            ByteCode::AReturn => "Reference Return (areturn)".to_string(),
            ByteCode::Return => "Return void (return)".to_string(),
            ByteCode::LReturn => "Long Return (lreturn)".to_string(),
            ByteCode::Ldc(u8) => format!("Ldc({})", u8),
            ByteCode::ALoad(u8) => format!("ALoad({})", u8),
            ByteCode::Lload(u8) => format!("Lload({})", u8),
            ByteCode::ILoad(u8) => format!("ILoad({})", u8),
            ByteCode::AStore(u8) => format!("AStore({})", u8),
            ByteCode::LStore(u8) => format!("LStore({})", u8),
            ByteCode::PutStatic(u16) => format!("PutStatic(0x{:x?})", u16),
            ByteCode::LAdd => "LAdd".to_string(),
            ByteCode::L2i => "L2i".to_string(),
            ByteCode::I2L => "I2L".to_string(),
            ByteCode::Athrow => "Athrow".to_string(),
            ByteCode::Ifeq(i16) => format!("Ifeq({})", i16),
            ByteCode::Ifne(i16) => format!("Ifne({})", i16),
            ByteCode::Iflt(i16) => format!("Iflt({})", i16),
            ByteCode::Ifge(i16) => format!("Ifge({})", i16),
            ByteCode::Ifgt(i16) => format!("Ifgt({})", i16),
            ByteCode::Ifle(i16) => format!("Ifle({})", i16),
            ByteCode::PutField(u16) => format!("PutField(0x{:x?})", u16),
            ByteCode::GetField(u16) => format!("GetField(0x{:x?})", u16),
            ByteCode::Generic(u8) => format!("Generic(0x{:x?})", u8),
        }
    }
}
