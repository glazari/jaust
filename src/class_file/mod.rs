/*
 * Java class file format
 * from the Oracle documentation


ClassFile {
    u4             magic;
    u2             minor_version;
    u2             major_version;
    u2             constant_pool_count;
    cp_info        constant_pool[constant_pool_count-1];
    u2             access_flags;
    u2             this_class;
    u2             super_class;
    u2             interfaces_count;
    u2             interfaces[interfaces_count];
    u2             fields_count;
    field_info     fields[fields_count];
    u2             methods_count;
    method_info    methods[methods_count];
    u2             attributes_count;
    attribute_info attributes[attributes_count];
}

 *
 *
 */

mod file_reader;

use file_reader::FileReader;
use std::io::Result;

pub fn read_class_file(filename: &str) -> Result<()> {

    let mut file = FileReader::new(filename)?;

    let magic = file.read_u4()?;
    println!("magic: {:?}", magic);
    println!("magic: {:x?}", magic);
    // magic number is 0xCAFEBABE
    assert_eq!(magic, [0xCA, 0xFE, 0xBA, 0xBE]);

    let minor_version = file.read_u2()?;
    let major_version = file.read_u2()?;
    println!("versions");
    println!("minor, major: {:?}, {:?}", minor_version, major_version);

    let constant_pool_count = file.read_u2_to_u16()?;
    println!("constant_pool_count: {}", constant_pool_count);

    let mut constant_pool = Vec::new();
    for _i in 1..constant_pool_count {
        let tag = file.read_u1()?;
        println!("tag: {:?}", tag);

        let constant = match tag {
            METHOD_REF => ConstantPoolInfo::MethodRefInfo(MethodRefInfo {
                class_index: file.read_u2_to_u16()?,
                name_and_type_index: file.read_u2_to_u16()?,
            }),
            CLASS => ConstantPoolInfo::ClassInfo(ClassInfo {
                name_index: file.read_u2_to_u16()?,
            }),
            NAME_AND_TYPE => ConstantPoolInfo::NameAndTypeInfo(NameAndTypeInfo {
                name_index: file.read_u2_to_u16()?,
                descriptor_index: file.read_u2_to_u16()?,
            }),
            UTF8 => ConstantPoolInfo::Utf8Info(file.read_string()?),
            FIELD_REF => ConstantPoolInfo::FieldRefInfo(FieldRefInfo {
                class_index: file.read_u2_to_u16()?,
                name_and_type_index: file.read_u2_to_u16()?,
            }),
            STRING => parse_string_info(&mut file)?,
            _ => {
                println!("tag not implemented: {}", tag);
                panic!();
            }
        };

        constant_pool.push(constant);
    }
    //println!("constant_pool: {:#?}", constant_pool);

    println!("constant pool info");
    for (i, info) in constant_pool.iter().enumerate() {
        println!("{}: {}", i, print_constant_pool_info(info, &constant_pool));
    }

    Ok(())
}

const UTF8: u8 = 1;
const METHOD_REF: u8 = 10;
const CLASS: u8 = 7;
const NAME_AND_TYPE: u8 = 12;
const FIELD_REF: u8 = 9;
const STRING: u8 = 8;

fn print_constant_pool_info(info: &ConstantPoolInfo, pool: &Vec<ConstantPoolInfo>) -> String {
    match info {
        ConstantPoolInfo::Utf8Info(s) => s.clone(),
        ConstantPoolInfo::NameAndTypeInfo(n) => {
            let name = get_from_constant_pool(n.name_index, pool);
            let descriptor = get_from_constant_pool(n.descriptor_index, pool);
            let name = print_constant_pool_info(name, pool);
            let descriptor = print_constant_pool_info(descriptor, pool);
            format!(
                "name: {}
description: {}",
                name, descriptor
            )
        }
        ConstantPoolInfo::ClassInfo(c) => {
            let name = get_from_constant_pool(c.name_index, pool);
            let name = print_constant_pool_info(name, pool);
            format!("class {} ", name)
        }
        ConstantPoolInfo::MethodRefInfo(m) => {
            let class = get_from_constant_pool(m.class_index, pool);
            let name_and_type = get_from_constant_pool(m.name_and_type_index, pool);
            let class = print_constant_pool_info(class, pool);
            let name_and_type = print_constant_pool_info(name_and_type, pool);
            format!(
                "Method:
- class: {}
- name_and_type: {}",
                class, name_and_type
            )
        }
        ConstantPoolInfo::FieldRefInfo(f) => {
            let class = get_from_constant_pool(f.class_index, pool);
            let name_and_type = get_from_constant_pool(f.name_and_type_index, pool);
            let class = print_constant_pool_info(class, pool);
            let name_and_type = print_constant_pool_info(name_and_type, pool);
            format!(
                "Field:
- class: {}
- name_and_type {}",
                class, name_and_type
            )
        }
        ConstantPoolInfo::StringInfo(s) => {
            let string = get_from_constant_pool(s.string_index, pool);
            let string = print_constant_pool_info(string, pool);
            format!("String: {}", string)
        }
    }
}

fn get_from_constant_pool(index: u16, pool: &Vec<ConstantPoolInfo>) -> &ConstantPoolInfo {
    &pool[index as usize - 1]
}

#[derive(Debug)]
enum ConstantPoolInfo {
    Utf8Info(String),
    NameAndTypeInfo(NameAndTypeInfo),
    ClassInfo(ClassInfo),
    MethodRefInfo(MethodRefInfo),
    FieldRefInfo(FieldRefInfo),
    StringInfo(StringInfo),
}

#[derive(Debug)]
struct MethodRefInfo {
    class_index: u16,
    name_and_type_index: u16,
}

#[derive(Debug)]
struct ClassInfo {
    name_index: u16,
}

#[derive(Debug)]
struct NameAndTypeInfo {
    name_index: u16,
    descriptor_index: u16,
}

#[derive(Debug)]
struct FieldRefInfo {
    class_index: u16,
    name_and_type_index: u16,
}

#[derive(Debug)]
struct StringInfo {
    string_index: u16,
}

fn parse_string_info(file: &mut FileReader) -> Result<ConstantPoolInfo> {
    let string_index = file.read_u2_to_u16()?;
    Ok(ConstantPoolInfo::StringInfo(StringInfo { string_index }))
}
