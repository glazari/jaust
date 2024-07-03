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

    for i in 1..constant_pool_count {
        let tag = read_u1(&mut file);
        println!("tag: {:?}", tag);

        match tag {
            10 => parse_methodref_info(&mut file),
            7 => parse_class_info(&mut file),
            _ => { 
                println!("tag not implemented: {}", tag);
                break;
            }
        }

    }


    Ok(())
}

fn parse_class_info(file: &mut File) {
    let name_index = read_u2(file);
    let name_index = u16::from_be_bytes(name_index);
    println!("name_index: {}", name_index);
}

fn parse_methodref_info(file: &mut File) {
    let class_index = read_u2(file);
    let class_index = u16::from_be_bytes(class_index);
    println!("class_index: {}", class_index);
    let name_and_type_index = read_u2(file);
    let name_and_type_index = u16::from_be_bytes(name_and_type_index);
    println!("name_and_type_index: {}", name_and_type_index);
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

fn read_u4(file: &mut File) -> U4 {
    let mut buf = [0; 4];
    file.read_exact(&mut buf).unwrap();
    buf
}
