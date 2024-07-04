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

use std::io::Read;
use std::io::Result;
use std::fs::File;

type U1 = u8;
type U2 = [u8; 2];
type U4 = [u8; 4];

pub fn read_class_file(filename: &str) -> Result<()> {
    let mut file = File::open(filename).unwrap();

    let magic = read_u4(&mut file);
    println!("magic: {:?}", magic);
    println!("magic: {:x?}", magic);
    // magic number is 0xCAFEBABE
    assert_eq!(magic, [0xCA, 0xFE, 0xBA, 0xBE]);

    let minor_version = read_u2(&mut file);
    let major_version = read_u2(&mut file);
    println!("versions");
    println!("minor, major: {:?}, {:?}", minor_version, major_version);


    let constant_pool_count = read_u2(&mut file);
    println!("constant_pool_count: {:?}", constant_pool_count);
    let constant_pool_count = u16::from_be_bytes(constant_pool_count);
    println!("constant_pool_count: {}", constant_pool_count);

    let mut constant_pool = Vec::new();
    for i in 1..constant_pool_count {
        let tag = read_u1(&mut file);
        println!("tag: {:?}", tag);


        let constant = match tag {
            10 => parse_methodref_info(&mut file),
            7 => parse_class_info(&mut file),
            12 => parse_name_and_type_info(&mut file),
            1 => parse_utf8_info(&mut file),    
            9 => parse_field_ref_info(&mut file),
            8 => parse_string_info(&mut file),
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


fn print_constant_pool_info(info: &ConstantPoolInfo, pool: &Vec<ConstantPoolInfo>) -> String {
    match info {
        ConstantPoolInfo::Utf8Info(s) => s.clone(),
        ConstantPoolInfo::NameAndTypeInfo(n) => {
            let name = get_from_constant_pool(n.name_index, pool); 
            let descriptor = get_from_constant_pool(n.descriptor_index, pool);
            let name = print_constant_pool_info(name, pool);
            let descriptor = print_constant_pool_info(descriptor, pool);
            format!("name: {}
description: {}", name, descriptor)
        
        },
        ConstantPoolInfo::ClassInfo(c) => {
            let name = get_from_constant_pool(c.name_index, pool);
            let name = print_constant_pool_info(name, pool);
            format!("class {} ", name)
        },
        ConstantPoolInfo::MethodRefInfo(m) => {
            let class = get_from_constant_pool(m.class_index, pool); 
            let name_and_type = get_from_constant_pool(m.name_and_type_index, pool);
            let class = print_constant_pool_info(class, pool);
            let name_and_type =  print_constant_pool_info(name_and_type, pool);
            format!("Method:
- class: {}
- name_and_type: {}", class, name_and_type)
        },
        ConstantPoolInfo::FieldRefInfo(f) => {
            let class = get_from_constant_pool(f.class_index, pool); 
            let name_and_type = get_from_constant_pool(f.name_and_type_index, pool);
            let class = print_constant_pool_info(class, pool);
            let name_and_type =  print_constant_pool_info(name_and_type, pool);
            format!("Field:
- class: {}
- name_and_type {}", class, name_and_type)
        
        },
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


fn parse_string_info(file: &mut File) -> ConstantPoolInfo {
    let string_index = read_u2_to_u16(file);
    ConstantPoolInfo::StringInfo(StringInfo { string_index })
}

fn parse_field_ref_info(file: &mut File) -> ConstantPoolInfo {
    let class_index = read_u2_to_u16(file); 
    let name_and_type_index = read_u2_to_u16(file); 
    ConstantPoolInfo::FieldRefInfo(FieldRefInfo { class_index, name_and_type_index })
}


fn parse_utf8_info(file: &mut File) -> ConstantPoolInfo {
    let length = read_u2_to_u16(file); 
    let mut buf = vec![0; length as usize];
    file.read_exact(&mut buf).unwrap();
    let s = String::from_utf8(buf.clone()).unwrap();
    ConstantPoolInfo::Utf8Info(s)
}

fn parse_name_and_type_info(file: &mut File) -> ConstantPoolInfo {
    let name_index = read_u2_to_u16(file);
    let descriptor_index = read_u2_to_u16(file);
    ConstantPoolInfo::NameAndTypeInfo(NameAndTypeInfo { name_index, descriptor_index })
}

fn parse_class_info(file: &mut File) -> ConstantPoolInfo {
    let name_index = read_u2_to_u16(file);
    ConstantPoolInfo::ClassInfo(ClassInfo { name_index })
}

fn parse_methodref_info(file: &mut File) -> ConstantPoolInfo {
    let class_index = read_u2_to_u16(file);
    let name_and_type_index = read_u2_to_u16(file);
    ConstantPoolInfo::MethodRefInfo(MethodRefInfo { class_index, name_and_type_index })
}


fn read_u1(file: &mut File) -> U1 {
    let mut buf = [0; 1];
    file.read_exact(&mut buf).unwrap();
    buf[0]
}

fn read_u2(file: &mut File) -> U2 {
    let mut buf = [0; 2];
    file.read_exact(&mut buf).unwrap();
    buf
}

fn read_u2_to_u16(file: &mut File) -> u16 {
    let mut buf = [0; 2];
    file.read_exact(&mut buf).unwrap();
    u16::from_be_bytes(buf)
}

fn read_u4(file: &mut File) -> U4 {
    let mut buf = [0; 4];
    file.read_exact(&mut buf).unwrap();
    buf
}
