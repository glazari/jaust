use super::access_flags::AccessFlag;
use super::constant_pool::ConstantPool;
use super::fields::AccessFlag as FieldAccessFlag;
use super::methods::AccessFlag as MethodAccessFlag;
use super::methods::Method;
use super::ClassFile;

use std::str::Chars;
use anyhow::Result;
use anyhow::anyhow;
use std::iter::Peekable;

/// Print a summary of the class file. like javap does by default.
pub fn print_tldr(cf: &ClassFile) {
    let mut out = String::new();
    let source = cf.attributes.get_source_file(&cf.constant_pool);
    if let Some(source) = source {
        out.push_str(&format!("Compiled from \"{}\"\n", source));
    }
    add_class_line(cf, &mut out);

    add_fields(cf, &mut out);

    add_methods(cf, &mut out);

    out.push_str("}\n");

    print!("{}", out);
}

fn add_fields(cf: &ClassFile, out: &mut String) {
    let indent = "  ";
    for field in &cf.fields.fields {
        let field_name = cf.constant_pool.get_to_string(field.name_index);
        let field_descriptor = cf.constant_pool.get_to_string(field.descriptor_index);

        let flags = field.access_flags.flag_vector();
        let mut modifiers = Vec::new();

        if flags.contains(&FieldAccessFlag::Public) {
            modifiers.push("public");
        } else if flags.contains(&FieldAccessFlag::Private) {
            modifiers.push("private");
            continue;
        } else if flags.contains(&FieldAccessFlag::Protected) {
            modifiers.push("protected");
        }

        if flags.contains(&FieldAccessFlag::Static) {
            modifiers.push("static");
        }

        if flags.contains(&FieldAccessFlag::Final) {
            modifiers.push("final");
        }

        if flags.contains(&FieldAccessFlag::Volatile) {
            modifiers.push("volatile");
        }

        if flags.contains(&FieldAccessFlag::Transient) {
            modifiers.push("transient");
        }

        if flags.contains(&FieldAccessFlag::Synthetic) {
            modifiers.push("synthetic");
        }

        if flags.contains(&FieldAccessFlag::Enum) {
            modifiers.push("enum");
        }

        out.push_str(indent);
        out.push_str(&modifiers.join(" "));
        out.push_str(" ");

        match field_descriptor.as_str().get(0..1) {
            Some("L") => out.push_str(&field_descriptor[1..].replace("/", ".").replace(";", "")),
            Some("I") => out.push_str("int"),
            Some("J") => out.push_str("long"),
            Some("F") => out.push_str("float"),
            Some("D") => out.push_str("double"),
            Some("S") => out.push_str("short"),
            Some("B") => out.push_str("byte"),
            Some("C") => out.push_str("char"),
            Some("Z") => out.push_str("boolean"),
            _ => out.push_str(&field_descriptor),
        }

        out.push_str(" ");
        out.push_str(&field_name);
        out.push_str(";\n");
    }
}

fn add_methods(cf: &ClassFile, out: &mut String) {
    let indent = "  ";
    for method in &cf.methods.methods {
        out.push_str(indent);
        add_method_modifiers(method, &cf.constant_pool, out);
        out.push_str(" ");

        let method_name = cf.constant_pool.get_to_string(method.name_index);
        if method_name == "<clinit>" { // static initializer
            out.push_str("{};\n");
            continue;
        }

        let descriptor = cf.constant_pool.get_to_string(method.descriptor_index);
        let signature = parse_method_descriptor(&descriptor, method_name);

        if signature.name == "<init>" {
            let class_name = cf.constant_pool.get_to_string(cf.this_class);
            out.push_str(&class_name);
        } else {
            out.push_str(&signature.return_type);
            out.push_str(" ");
            out.push_str(&signature.name);
        }


        out.push_str("(");
        out.push_str(&signature.args.join(", "));
        out.push_str(");\n");
    }
}

fn parse_method_descriptor(descriptor: &str, name: String) -> MethodSignature {
    let mut args = Vec::new();

    let mut chars = descriptor.chars().peekable();
    let c = chars.next().unwrap();
    assert_eq!(c, '(');
    loop {
        if let Some(')') = chars.peek() {
            break;
        }

        let _type = parse_type_descriptor(&mut chars)
            .unwrap_or_else(|e| e.to_string());
        args.push(_type);
    }
    let c = chars.next();
    assert_eq!(c, Some(')'));

    let return_type = parse_type_descriptor(&mut chars)
        .unwrap_or_else(|e| e.to_string());

    MethodSignature {
        name,
        args,
        return_type,
    }
}

type Pchars<'a> = Peekable<Chars<'a>>;

fn parse_type_descriptor(chars: &mut Pchars) -> Result<String> {
    let c = chars.next();
    let out = match c {
        Some('I') => "int", 
        Some('J') => "long",
        Some('F') => "float",
        Some('D') => "double",
        Some('S') => "short",
        Some('B') => "byte",
        Some('C') => "char",
        Some('Z') => "boolean",
        Some('V') => "void",
        Some('L') => {
            let mut arg = String::new();
            loop {
                let c = chars.next();
                match c {
                    Some(';') | Some(')') => break,
                    Some('/') => arg.push('.'),
                    Some(c) => arg.push(c),
                    None => return Err(anyhow!("Unexpected end of method descriptor")),
                }
            }
            return Ok(arg);
        }
        Some('[') => {
            let mut arg = parse_type_descriptor(chars)?;
            arg.push_str("[]");
            return Ok(arg);
        }
        _ => return Err(anyhow!("Unknown type in method descriptor: {:?}", c)),
        
    };
    Ok(out.to_string())
}


#[derive(Debug, PartialEq)]
struct MethodSignature {
    name: String,
    args: Vec<String>,
    return_type: String,
}

fn add_method_modifiers(method: &Method, cp: &ConstantPool, out: &mut String) {
    let flags = method.access_flags.flag_vector();
    let o = flags.iter().map(|f| f.to_str().to_string()).collect::<Vec<String>>().join(" ");
    out.push_str(&o);
    return;
    
    if flags.contains(&MethodAccessFlag::Public) {
        out.push_str("public");
    } else if flags.contains(&MethodAccessFlag::Private) {
        out.push_str("private");
    } else if flags.contains(&MethodAccessFlag::Protected) {
        out.push_str("protected");
    }

    if flags.contains(&MethodAccessFlag::Static) {
        out.push_str("static");
    }

    if flags.contains(&MethodAccessFlag::Final) {
        out.push_str("final");
    }

    if flags.contains(&MethodAccessFlag::Synchronized) {
        out.push_str("synchronized");
    }
}

pub fn add_class_line(cf: &ClassFile, out: &mut String) {
    add_class_modifiers(cf, out);

    let class_name = cf.constant_pool.get_to_string(cf.this_class);
    out.push_str(" ");
    out.push_str(&class_name);

    if cf.super_class != 0 {
        let super_class_name = cf.constant_pool.get_to_string(cf.super_class);
        out.push_str(" extends ");
        out.push_str(&super_class_name);
    }

    if cf.interfaces.interfaces.len() > 0 {
        out.push_str(" implements ");
        for interface in &cf.interfaces.interfaces {
            let interface_name = cf.constant_pool.get_to_string(*interface);
            out.push_str(&interface_name);
            out.push_str(", ");
        }
        out.pop();
        out.pop();
    }

    out.push_str(" {\n");
}

fn add_class_modifiers(cf: &ClassFile, out: &mut String) {
    let flags = cf.access_flags.flag_vector();

    let mut modifiers = Vec::new();

    if flags.contains(&AccessFlag::Public) {
        modifiers.push("public");
    }

    if flags.contains(&AccessFlag::Final) {
        modifiers.push("final");
    }

    if flags.contains(&AccessFlag::Abstract) {
        modifiers.push("abstract");
    }

    if flags.contains(&AccessFlag::Interface) {
        modifiers.push("interface");
    } else if flags.contains(&AccessFlag::Enum) {
        modifiers.push("enum");
    } else {
        modifiers.push("class");
    }

    out.push_str(&modifiers.join(" "));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn method_sig(name: &str, args: Vec<&str>, return_type: &str) -> MethodSignature {
        MethodSignature {
            name: name.to_string(),
            args: args.iter().map(|s| s.to_string()).collect(),
            return_type: return_type.to_string(),
        }
    }

    #[test]
    fn test_parse_method_descriptor() {
        let cases = vec![
            ("()V", method_sig("f", vec![], "void")),
            ("(II)I", method_sig("f", vec!["int", "int"], "int")),
            ("(Ljava/lang/String;)V", method_sig("f", vec!["java.lang.String"], "void")),
            ("(Ljava/lang/String;I)V", method_sig("f", vec!["java.lang.String", "int"], "void")),
            ("(Ljava/lang/String;I)I", method_sig("f", vec!["java.lang.String", "int"], "int")),
            ("(Ljava/lang/String;I)[I", method_sig("f", vec!["java.lang.String", "int"], "int[]")),
            ("(Ljava/lang/String;I)[[I", method_sig("f", vec!["java.lang.String", "int"], "int[][]")),
        ];

        for (descriptor, expected) in cases {
            let signature = parse_method_descriptor(descriptor, "f".to_string());
            assert_eq!(signature, expected);
        }
        }


    #[test]
    fn test_parse_type_descriptor() {
        let cases = vec![
            ("I", "int"),
            ("J", "long"),
            ("F", "float"),
            ("D", "double"),
            ("S", "short"),
            ("B", "byte"),
            ("C", "char"),
            ("Z", "boolean"),
            ("V", "void"),
            ("Lcom/example/Class;", "com.example.Class"),
            ("[Lcom/example/Class;", "com.example.Class[]"),
        ];

        for (descriptor, expected) in cases {
            let mut descriptor = descriptor.chars().peekable();
            let type_ = parse_type_descriptor(&mut descriptor).unwrap();
            assert_eq!(type_, expected);
        }

        
    }
}
