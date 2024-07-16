use super::attributes::AttStart;
use super::attributes::Attributes;
use super::bytecode::ByteCode;
use super::constant_pool::ConstantPool;
use super::file_reader::FileReader;
use anyhow::Result;

#[derive(Debug)]
pub struct CodeAttribute {
    attribute_name_index: u16,
    attribute_length: u32,
    max_stack: u16,
    max_locals: u16,
    code_length: u32,
    code: Vec<ByteCode>,
    exception_table: Vec<ExceptionTable>,
    attributes: Attributes,
}

#[derive(Debug)]
pub struct ExceptionTable {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl CodeAttribute {
    pub fn parse(
        file: &mut FileReader,
        att_start: &AttStart,
        cp: &ConstantPool,
    ) -> Result<CodeAttribute> {
        let max_stack = file.read_u2_to_u16()?;
        let max_locals = file.read_u2_to_u16()?;
        let code_length = file.read_u4_to_u32()?;

        let mut code = Vec::new();
        let mut curr_code = 0;
        while curr_code < code_length {
            let (byte_code, len) = ByteCode::parse(file)?;
            curr_code += len;
            code.push(byte_code);
        }

        let exception_table_length = file.read_u2_to_u16()?;
        let mut exception_table = Vec::new();
        for _i in 0..exception_table_length {
            let start_pc = file.read_u2_to_u16()?;
            let end_pc = file.read_u2_to_u16()?;
            let handler_pc = file.read_u2_to_u16()?;
            let catch_type = file.read_u2_to_u16()?;
            exception_table.push(ExceptionTable {
                start_pc,
                end_pc,
                handler_pc,
                catch_type,
            });
        }

        let attributes = Attributes::from(file, cp)?;

        Ok(CodeAttribute {
            attribute_name_index: att_start.attribute_name_index,
            attribute_length: att_start.attribute_length,
            max_stack,
            max_locals,
            code_length,
            code,
            exception_table,
            attributes,
        })
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "name {}\n",
            cp.get_to_string(self.attribute_name_index)
        ));
        s.push_str(&format!("length {}\n", self.attribute_length));

        s.push_str(&format!(
            "Code: max_stack: {}, max_locals: {}, code_length: {}\n",
            self.max_stack, self.max_locals, self.code_length
        ));

        s.push_str(&format!("Code: {}\n", self.code.len()));
        for c in &self.code {
            s.push_str(&format!("- {}\n", c.to_string()));
        }

        s.push_str(&format!(
            "Exception table: {}\n",
            self.exception_table.len()
        ));
        for et in &self.exception_table {
            s.push_str(&format!(
                "start_pc: {}, end_pc: {}, handler_pc: {}, catch_type: {}\n",
                et.start_pc, et.end_pc, et.handler_pc, et.catch_type
            ));
        }

        s.push_str(&self.attributes.to_string(cp));

        s
    }
}
