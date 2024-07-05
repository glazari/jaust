use crate::class_file::file_reader::FileReader;
use std::io::Result;

pub struct ConstantPool {
    constants: Vec<Info>,
}

const UTF8: u8 = 1;
const METHOD_REF: u8 = 10;
const CLASS: u8 = 7;
const NAME_AND_TYPE: u8 = 12;
const FIELD_REF: u8 = 9;
const STRING: u8 = 8;

#[derive(Debug)]
pub enum Info {
    Utf8Info(String),
    NameAndTypeInfo(NameAndTypeInfo),
    ClassInfo(ClassInfo),
    MethodRefInfo(MethodRefInfo),
    FieldRefInfo(FieldRefInfo),
    StringInfo(StringInfo),
}

#[derive(Debug)]
pub struct MethodRefInfo {
    class_index: u16,
    name_and_type_index: u16,
}

#[derive(Debug)]
pub struct ClassInfo {
    name_index: u16,
}

#[derive(Debug)]
pub struct NameAndTypeInfo {
    name_index: u16,
    descriptor_index: u16,
}

#[derive(Debug)]
pub struct FieldRefInfo {
    class_index: u16,
    name_and_type_index: u16,
}

#[derive(Debug)]
pub struct StringInfo {
    string_index: u16,
}

impl ConstantPool {
    pub fn from(file: &mut FileReader) -> Result<ConstantPool> {
        let mut constant_pool = Vec::new();

        let constant_pool_count = file.read_u2_to_u16()?;
        for _i in 1..constant_pool_count {
            let tag = file.read_u1()?;

            let constant = match tag {
                METHOD_REF => Info::MethodRefInfo(MethodRefInfo {
                    class_index: file.read_u2_to_u16()?,
                    name_and_type_index: file.read_u2_to_u16()?,
                }),
                CLASS => Info::ClassInfo(ClassInfo {
                    name_index: file.read_u2_to_u16()?,
                }),
                NAME_AND_TYPE => Info::NameAndTypeInfo(NameAndTypeInfo {
                    name_index: file.read_u2_to_u16()?,
                    descriptor_index: file.read_u2_to_u16()?,
                }),
                UTF8 => Info::Utf8Info(file.read_string()?),
                FIELD_REF => Info::FieldRefInfo(FieldRefInfo {
                    class_index: file.read_u2_to_u16()?,
                    name_and_type_index: file.read_u2_to_u16()?,
                }),
                STRING => Info::StringInfo(StringInfo {
                    string_index: file.read_u2_to_u16()?,
                }),
                _ => {
                    println!("tag not implemented: {}", tag);
                    panic!();
                }
            };
            constant_pool.push(constant);
        }

        Ok(ConstantPool {
            constants: constant_pool,
        })
    }

    pub fn get(&self, index: u16) -> &Info {
        &self.constants[index as usize - 1]
    }


    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for (i, info) in self.constants.iter().enumerate() {
            s.push_str(&format!("{}: {}\n", i, self.info_to_string(info)));
        }
        s
    }

    pub fn info_to_string(&self, info: &Info) -> String {
        match info {
            Info::Utf8Info(s) => s.clone(),
            Info::NameAndTypeInfo(n) => {
                let name = self.get(n.name_index);
                let descriptor = self.get(n.descriptor_index);
                let name = self.info_to_string(name);
                let descriptor = self.info_to_string(descriptor);
                format!("{}[{}]", name, descriptor)
            },
            Info::ClassInfo(c) => {
                let name = self.get(c.name_index);
                self.info_to_string(name)
            },
            Info::MethodRefInfo(m) => {
                let class = self.get(m.class_index);
                let name_and_type = self.get(m.name_and_type_index);
                let class = self.info_to_string(class);
                let name_and_type = self.info_to_string(name_and_type);
                format!("{}.{}()", class, name_and_type)
            },
            Info::FieldRefInfo(f) => {
                let class = self.get(f.class_index);
                let name_and_type = self.get(f.name_and_type_index);
                let class = self.info_to_string(class);
                let name_and_type = self.info_to_string(name_and_type);
                format!("{}.{}", class, name_and_type)
            }
            Info::StringInfo(s) => {
                let string = self.get(s.string_index);
                self.info_to_string(string)
            }
        }
    }


}
