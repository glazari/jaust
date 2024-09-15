use super::attributes::AttStart;

use crate::class_file::{constant_pool::ConstantPool, file_reader::FileReader};

use anyhow::Result;

#[derive(Debug)]
pub struct RuntimeVisibleAnnotationsAttribute {
    attribute_name_index: u16,
    attribute_length: u32,
    annotations: Vec<Annotation>,
}

#[derive(Debug)]
pub struct Annotation {
    type_index: u16,
    element_value_pairs: Vec<ElementValuePair>,
}

#[derive(Debug)]
pub struct ElementValuePair {
    element_name_index: u16,
    value: ElementValue,
}

#[derive(Debug)]
pub enum ElementValue {
    ConstValueIndex(u16),
    EnumConstValue {
        type_name_index: u16,
        const_name_index: u16,
    },
    ClassInfoIndex(u16),
    AnnotationValue(Annotation),
    ArrayValue(Vec<ElementValue>),
}

impl RuntimeVisibleAnnotationsAttribute {
    pub fn parse(
        file: &mut FileReader,
        att_start: &AttStart,
    ) -> Result<RuntimeVisibleAnnotationsAttribute> {
        let num_annotations = file.read_u2_to_u16()?;
        let mut annotations = Vec::new();
        for _i in 0..num_annotations {
            let annotation = Annotation::parse(file)?;
            annotations.push(annotation);
        }
        Ok(RuntimeVisibleAnnotationsAttribute {
            attribute_name_index: att_start.attribute_name_index,
            attribute_length: att_start.attribute_length,
            annotations,
        })
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let mut s = String::new();
        s.push_str("RuntimeVisibleAnnotations: ");
        for annotation in &self.annotations {
            s.push_str(&annotation.to_string(cp));
            s.push_str(", ");
        }
        s
    }
}

impl Annotation {
    pub fn parse(file: &mut FileReader) -> Result<Annotation> {
        let type_index = file.read_u2_to_u16()?;
        let num_pairs = file.read_u2_to_u16()?;
        let mut element_value_pairs = Vec::new();
        for _i in 0..num_pairs {
            let element_name_index = file.read_u2_to_u16()?;
            let value = ElementValue::parse(file)?;
            element_value_pairs.push(ElementValuePair {
                element_name_index,
                value,
            });
        }
        Ok(Annotation {
            type_index,
            element_value_pairs,
        })
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let mut s = String::new();
        s.push_str("@");
        s.push_str(&cp.get_to_string(self.type_index));
        s.push_str("(");
        for pair in &self.element_value_pairs {
            s.push_str(&cp.get_to_string(pair.element_name_index));
            s.push_str(" = ");
            s.push_str(&pair.value.to_string(cp));
            s.push_str(", ");
        }
        s.push_str(")");
        s
    }
}

impl ElementValue {
    pub fn parse(file: &mut FileReader) -> Result<ElementValue> {
        let tag = file.read_u1()?;
        match tag {
            b'B' => Ok(ElementValue::ConstValueIndex(file.read_u2_to_u16()?)),
            b'C' => Ok(ElementValue::ConstValueIndex(file.read_u2_to_u16()?)),
            b'D' => Ok(ElementValue::ConstValueIndex(file.read_u2_to_u16()?)),
            b'F' => Ok(ElementValue::ConstValueIndex(file.read_u2_to_u16()?)),
            b'I' => Ok(ElementValue::ConstValueIndex(file.read_u2_to_u16()?)),
            b'J' => Ok(ElementValue::ConstValueIndex(file.read_u2_to_u16()?)),
            b'S' => Ok(ElementValue::ConstValueIndex(file.read_u2_to_u16()?)),
            b'Z' => Ok(ElementValue::ConstValueIndex(file.read_u2_to_u16()?)),
            b's' => Ok(ElementValue::ConstValueIndex(file.read_u2_to_u16()?)),
            b'e' => {
                let type_name_index = file.read_u2_to_u16()?;
                let const_name_index = file.read_u2_to_u16()?;
                Ok(ElementValue::EnumConstValue {
                    type_name_index,
                    const_name_index,
                })
            }
            b'c' => Ok(ElementValue::ClassInfoIndex(file.read_u2_to_u16()?)),
            b'@' => {
                let annotation = Annotation::parse(file)?;
                Ok(ElementValue::AnnotationValue(annotation))
            }
            b'[' => {
                let num_values = file.read_u2_to_u16()?;
                let mut values = Vec::new();
                for _i in 0..num_values {
                    let value = ElementValue::parse(file)?;
                    values.push(value);
                }
                Ok(ElementValue::ArrayValue(values))
            }
            _ => anyhow::bail!("Invalid tag: {}", tag),
        }
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        match self {
            ElementValue::ConstValueIndex(index) => cp.get_to_string(*index),
            ElementValue::EnumConstValue {
                type_name_index,
                const_name_index,
            } => {
                format!(
                    "{}::{}",
                    cp.get_to_string(*type_name_index),
                    cp.get_to_string(*const_name_index)
                )
            }
            ElementValue::ClassInfoIndex(index) => cp.get_to_string(*index),
            ElementValue::AnnotationValue(annotation) => annotation.to_string(cp),
            ElementValue::ArrayValue(values) => {
                let mut s = String::new();
                s.push_str("{");
                for value in values {
                    s.push_str(&value.to_string(cp));
                    s.push_str(", ");
                }
                s.push_str("}");
                s
            }
        }
    }
}
