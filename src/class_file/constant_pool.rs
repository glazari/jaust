use crate::class_file::file_reader::FileReader;
use anyhow::Result;

pub struct ConstantPool {
    constants: Vec<Info>,
}

const UTF8: u8 = 1;
const METHOD_REF: u8 = 10;
const CLASS: u8 = 7;
const NAME_AND_TYPE: u8 = 12;
const FIELD_REF: u8 = 9;
const STRING: u8 = 8;
const INVOKEDYNAMIC: u8 = 18;
const METHOD_HANDLE: u8 = 15;

#[derive(Debug)]
pub enum Info {
    Utf8Info(String),
    NameAndTypeInfo(NameAndTypeInfo),
    ClassInfo(ClassInfo),
    MethodRefInfo(MethodRefInfo),
    FieldRefInfo(FieldRefInfo),
    StringInfo(StringInfo),
    InvokeDynamicInfo(InvokeDynamicInfo),
    MethodHandleInfo(MethodHandleInfo),
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

#[derive(Debug)]
pub struct InvokeDynamicInfo {
    bootstrap_method_attr_index: u16,
    name_and_type_index: u16,
}

#[derive(Debug)]
pub struct MethodHandleInfo {
    reference_kind: MethodHandleReferenceKind,
    reference_index: u16,
}

#[derive(Debug)]
pub enum MethodHandleReferenceKind {
    REF_getField,         // 1     getfield C.f:T
    REF_getStatic,        // 2     getstatic C.f:T
    REF_putField,         // 3     putfield C.f:T
    REF_putStatic,        // 4     putstatic C.f:T
    REF_invokeVirtual,    // 5     invokevirtual C.m:(A*)T
    REF_invokeStatic,     // 6     invokestatic C.m:(A*)T
    REF_invokeSpecial,    // 7     invokespecial C.m:(A*)T
    REF_newInvokeSpecial, // 8     new C; dup; invokespecial C.<init>:(A*)void
    REF_invokeInterface,  // 9     invokeinterface C.m:(A*)T
}

impl MethodHandleReferenceKind {
    pub fn from_u8(value: u8) -> MethodHandleReferenceKind {
        match value {
            1 => MethodHandleReferenceKind::REF_getField,
            2 => MethodHandleReferenceKind::REF_getStatic,
            3 => MethodHandleReferenceKind::REF_putField,
            4 => MethodHandleReferenceKind::REF_putStatic,
            5 => MethodHandleReferenceKind::REF_invokeVirtual,
            6 => MethodHandleReferenceKind::REF_invokeStatic,
            7 => MethodHandleReferenceKind::REF_invokeSpecial,
            8 => MethodHandleReferenceKind::REF_newInvokeSpecial,
            9 => MethodHandleReferenceKind::REF_invokeInterface,
            _ => panic!("invalid reference kind {}", value),
        }
    }

    pub fn to_string(&self) -> String {
        let out = match self {
            MethodHandleReferenceKind::REF_getField => "getField",
            MethodHandleReferenceKind::REF_getStatic => "getStatic",
            MethodHandleReferenceKind::REF_putField => "putField",
            MethodHandleReferenceKind::REF_putStatic => "putStatic",
            MethodHandleReferenceKind::REF_invokeVirtual => "invokeVirtual",
            MethodHandleReferenceKind::REF_invokeStatic => "invokeStatic",
            MethodHandleReferenceKind::REF_invokeSpecial => "invokeSpecial",
            MethodHandleReferenceKind::REF_newInvokeSpecial => "newInvokeSpecial",
            MethodHandleReferenceKind::REF_invokeInterface => "invokeInterface",
        };
        out.to_string()
    }
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
                INVOKEDYNAMIC => Info::InvokeDynamicInfo(InvokeDynamicInfo {
                    bootstrap_method_attr_index: file.read_u2_to_u16()?,
                    name_and_type_index: file.read_u2_to_u16()?,
                }),
                METHOD_HANDLE => Info::MethodHandleInfo(MethodHandleInfo {
                    reference_kind: MethodHandleReferenceKind::from_u8(file.read_u1()?),
                    reference_index: file.read_u2_to_u16()?,
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

    pub fn get_to_string(&self, index: u16) -> String {
        self.info_to_string(self.get(index))
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
            }
            Info::ClassInfo(c) => {
                let name = self.get(c.name_index);
                self.info_to_string(name)
            }
            Info::MethodRefInfo(m) => {
                let class = self.get(m.class_index);
                let name_and_type = self.get(m.name_and_type_index);
                let class = self.info_to_string(class);
                let name_and_type = self.info_to_string(name_and_type);
                format!("{}.{}()", class, name_and_type)
            }
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
            Info::InvokeDynamicInfo(i) => {
                let name_and_type = self.get(i.name_and_type_index);
                let name_and_type = self.info_to_string(name_and_type);
                format!(
                    "InvokeDynamic(bootstrap_index{})[{}]",
                    i.bootstrap_method_attr_index, name_and_type
                )
            }
            Info::MethodHandleInfo(m) => {
                let reference = self.get(m.reference_index);
                let reference = self.info_to_string(reference);
                format!("MethodHandle({})[{}]", m.reference_kind.to_string(), reference)
            }
        }
    }
}
