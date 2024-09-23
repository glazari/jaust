use super::{
    BootstrapMethodsAttribute, CodeAttribute, DeprecatedAttribute, ExceptionsAttribute,
    GenericAttribute, InnerClassesAttribute, LineNumberTableAttribute, MethodParametersAttribute,
    RecordAttribute, RuntimeVisibleAnnotationsAttribute, SourceFileAttribute,
    StackMapTableAttribute, SignatureAttribute
};

use crate::class_file::{constant_pool::ConstantPool, file_reader::FileReader};
use crate::print_debug as p;
use anyhow::Result;

#[derive(Debug)]
pub struct Attributes {
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub enum Attribute {
    Code(CodeAttribute),
    SourceFile(SourceFileAttribute),
    LineNumberTable(LineNumberTableAttribute),
    StackMapTable(StackMapTableAttribute),
    Deprecated(DeprecatedAttribute),
    Exceptions(ExceptionsAttribute),
    RuntimeVisibleAnnotationsAttribute(RuntimeVisibleAnnotationsAttribute),
    RecordAttribute(RecordAttribute),
    InnerClassesAttribute(InnerClassesAttribute),
    MethodParametersAttribute(MethodParametersAttribute),
    BootstrapMethodsAttribute(BootstrapMethodsAttribute),
    SignatureAttribute(SignatureAttribute),
    GenericAttribute(GenericAttribute),
}

#[derive(Debug)]
pub struct AttStart {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
}

impl Attributes {
    pub fn from(file: &mut FileReader, cp: &ConstantPool) -> Result<Attributes> {
        let mut attributes = Vec::new();

        let attributes_count = file.read_u2_to_u16()?;
        for _i in 0..attributes_count {
            let attribute_name_index = file.read_u2_to_u16()?;
            let attribute_length = file.read_u4_to_u32()?;

            let att_start = AttStart {
                attribute_name_index,
                attribute_length,
            };

            let name = cp.get_to_string(attribute_name_index);
            p!("Attribute name: {}", name);
            match name.as_str() {
                "Code" => {
                    let att = CodeAttribute::parse(file, &att_start, cp)?;
                    attributes.push(Attribute::Code(att));
                }
                "SourceFile" => {
                    let att = SourceFileAttribute::parse(file, &att_start)?;
                    attributes.push(Attribute::SourceFile(att));
                }
                "LineNumberTable" => {
                    let att = LineNumberTableAttribute::parse(file, &att_start)?;
                    attributes.push(Attribute::LineNumberTable(att));
                }
                "StackMapTable" => {
                    let att = StackMapTableAttribute::parse(file, &att_start)?;
                    attributes.push(Attribute::StackMapTable(att));
                }
                "Deprecated" => {
                    let att = DeprecatedAttribute::parse(file, &att_start)?;
                    attributes.push(Attribute::Deprecated(att));
                }
                "Exceptions" => {
                    let att = ExceptionsAttribute::parse(file, &att_start)?;
                    attributes.push(Attribute::Exceptions(att));
                }
                "RuntimeVisibleAnnotations" => {
                    let att = RuntimeVisibleAnnotationsAttribute::parse(file, &att_start)?;
                    attributes.push(Attribute::RuntimeVisibleAnnotationsAttribute(att));
                }
                "Record" => {
                    let att = RecordAttribute::parse(file, &att_start, cp)?;
                    attributes.push(Attribute::RecordAttribute(att));
                }
                "InnerClasses" => {
                    let att = InnerClassesAttribute::parse(file, &att_start)?;
                    attributes.push(Attribute::InnerClassesAttribute(att));
                }
                "MethodParameters" => {
                    let att = MethodParametersAttribute::parse(file, &att_start)?;
                    attributes.push(Attribute::MethodParametersAttribute(att));
                }
                "BootstrapMethods" => {
                    let att = BootstrapMethodsAttribute::parse(file, &att_start)?;
                    attributes.push(Attribute::BootstrapMethodsAttribute(att));
                }
                "Signature" => {
                    let att = SignatureAttribute::parse(file, &att_start)?;
                    attributes.push(Attribute::SignatureAttribute(att));
                }
                _ => {
                    let att = GenericAttribute::parse(file, &att_start)?;
                    attributes.push(Attribute::GenericAttribute(att));
                }
            }
        }

        Ok(Attributes { attributes })
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let mut s = String::new();

        s.push_str("Attributes\n");
        for att in &self.attributes {
            s.push_str(&att.to_string(cp));
        }
        s
    }

    pub fn get_source_file(&self, cp: &ConstantPool) -> Option<String> {
        for att in &self.attributes {
            match att {
                Attribute::SourceFile(att) => {
                    return Some(att.to_string(cp));
                }
                _ => {}
            }
        }
        None
    }

    pub fn get_checked_exceptions(&self, cp: &ConstantPool) -> Vec<String> {
        let mut exceptions = Vec::new();
        for att in &self.attributes {
            match att {
                Attribute::Exceptions(e) => {
                    e.exception_index_table.iter().for_each(|e_index| 
                        exceptions.push(cp.get_to_string(*e_index))
                    );
                }
                _ => {}
            }
        }
        exceptions
    }
}

impl Attribute {
    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let mut s = String::new();
        match self {
            Attribute::GenericAttribute(att) => s.push_str(&att.to_string(cp)),
            Attribute::SourceFile(att) => {
                s.push_str("SourceFile ");
                s.push_str(&att.to_string(cp));
                s.push_str("\n");
            }
            Attribute::Code(att) => s.push_str(&att.to_string(cp)),
            Attribute::LineNumberTable(att) => s.push_str(&att.to_string(cp)),
            Attribute::StackMapTable(att) => s.push_str(&att.to_string(cp)),
            Attribute::Deprecated(att) => s.push_str(&att.to_string(cp)),
            Attribute::Exceptions(att) => s.push_str(&att.to_string(cp)),
            Attribute::RuntimeVisibleAnnotationsAttribute(att) => s.push_str(&att.to_string(cp)),
            Attribute::RecordAttribute(att) => s.push_str(&att.to_string(cp)),
            Attribute::InnerClassesAttribute(att) => s.push_str(&att.to_string(cp)),
            Attribute::MethodParametersAttribute(att) => s.push_str(&att.to_string(cp)),
            Attribute::BootstrapMethodsAttribute(att) => s.push_str(&att.to_string(cp)),
            Attribute::SignatureAttribute(att) => s.push_str(&att.to_string(cp)),
        }
        s
    }
}
