use super::access_flags::AccessFlag;
use super::constant_pool::ConstantPool;
use super::fields::AccessFlag as FieldAccessFlag;
use super::methods::AccessFlag as MethodAccessFlag;
use super::methods::Method;
use super::ClassFile;

use anyhow::anyhow;
use anyhow::Result;
use std::iter::Peekable;
use std::str::Chars;

pub struct Options {
    pub private: bool,
    pub code: bool,
}

/// Print a summary of the class file. like javap does by default.
pub fn print_tldr(cf: &ClassFile, opts: &Options) {
    let out = jaustp_summary(cf, opts);
    print!("{}", out);
}

pub fn jaustp_summary(cf: &ClassFile, opts: &Options) -> String {
    let mut out = String::new();
    let source = cf.attributes.get_source_file(&cf.constant_pool);
    if let Some(source) = source {
        out.push_str(&format!("Compiled from \"{}\"\n", source));
    }
    add_class_line(cf, &mut out);

    add_fields(cf, &mut out, opts);

    add_methods(cf, &mut out, opts);

    out.push_str("}\n");

    out
}

fn add_fields(cf: &ClassFile, out: &mut String, opts: &Options) {
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
            if !opts.private {
                continue;
            }
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
        if modifiers.len() > 0 {
            out.push_str(" ");
        }

        let mut descriptor = field_descriptor.chars().peekable();
        let _type = parse_type_descriptor(&mut descriptor).unwrap_or_else(|e| e.to_string());
        out.push_str(&_type);

        out.push_str(" ");
        out.push_str(&field_name);
        out.push_str(";\n");
    }
}

fn add_methods(cf: &ClassFile, out: &mut String, opts: &Options) {
    let indent = "  ";
    for method in &cf.methods.methods {
        let flags = method.access_flags.flag_vector();

        if flags.contains(&MethodAccessFlag::Private) && !opts.private {
            continue;
        }

        let modifiers = flags
            .iter()
            .map(|f| f.to_str().to_string())
            .collect::<Vec<String>>()
            .join(" ");
        out.push_str(indent);
        out.push_str(&modifiers);

        if modifiers.len() > 0 {
            out.push_str(" ");
        }

        let method_name = cf.constant_pool.get_to_string(method.name_index);
        if method_name == "<clinit>" {
            // static initializer
            out.push_str("{};\n");
            continue;
        }

        let descriptor = cf.constant_pool.get_to_string(method.descriptor_index);
        let signature = parse_method_descriptor(&descriptor, method_name);

        if signature.name == "<init>" {
            let class_name = cf.constant_pool.get_to_string(cf.this_class);
            let class_name = class_name.replace("/", ".");
            out.push_str(&class_name);
        } else {
            out.push_str(&signature.return_type);
            out.push_str(" ");
            out.push_str(&signature.name);
        }

        out.push_str("(");
        out.push_str(&signature.args.join(", "));
        out.push_str(")");

        let checked_exceptions = method.attributes.get_checked_exceptions(&cf.constant_pool);
        if checked_exceptions.len() > 0 {
            let exceptions = checked_exceptions.iter()
                .map(|e| e.replace('/', "."))
                .collect::<Vec<String>>() 
                .join(" ");
            out.push_str(" throws ");
            out.push_str(&exceptions);
        }

        out.push_str(";\n");
        if opts.code {
            print_code(&method, &cf.constant_pool, out);
            out.push_str("\n");
        }
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

        let _type = parse_type_descriptor(&mut chars).unwrap_or_else(|e| e.to_string());
        args.push(_type);
    }
    let c = chars.next();
    assert_eq!(c, Some(')'));

    let return_type = parse_type_descriptor(&mut chars).unwrap_or_else(|e| e.to_string());

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

pub fn add_class_line(cf: &ClassFile, out: &mut String) {
    add_class_modifiers(cf, out);

    let class_name = cf.constant_pool.get_to_string(cf.this_class);
    let class_name = class_name.replace("/", ".");
    out.push_str(" ");
    out.push_str(&class_name);

    if cf.super_class != 0 {
        let super_class_name = cf.constant_pool.get_to_string(cf.super_class);
        if super_class_name != "java/lang/Object" {
            let super_class_name = super_class_name.replace("/", ".");
            out.push_str(" extends ");
            out.push_str(&super_class_name);
        }
    }

    if cf.interfaces.interfaces.len() > 0 {
        out.push_str(" implements ");
        for interface in &cf.interfaces.interfaces {
            let interface_name = cf.constant_pool.get_to_string(*interface);
            let interface_name = interface_name.replace("/", ".");
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

    let kind = if flags.contains(&AccessFlag::Interface) {
        "interface"
    } else if flags.contains(&AccessFlag::Enum) {
        "enum"
    } else {
        "class"
    };

    if flags.contains(&AccessFlag::Public) {
        modifiers.push("public");
    }

    if flags.contains(&AccessFlag::Final) {
        modifiers.push("final");
    }

    if flags.contains(&AccessFlag::Abstract) && kind != "interface" /* interfaces are always abstract */ {
        modifiers.push("abstract");
    }

    modifiers.push(kind);
    out.push_str(&modifiers.join(" "));
}

fn print_code(method: &Method, cp: &ConstantPool, out: &mut String) {
    let code = method.get_code().unwrap();
    for c in code.code() {
        out.push_str(&format!("\t- {}\n", c.to_string()));
    }
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
            (
                "(Ljava/lang/String;)V",
                method_sig("f", vec!["java.lang.String"], "void"),
            ),
            (
                "(Ljava/lang/String;I)V",
                method_sig("f", vec!["java.lang.String", "int"], "void"),
            ),
            (
                "(Ljava/lang/String;I)I",
                method_sig("f", vec!["java.lang.String", "int"], "int"),
            ),
            (
                "(Ljava/lang/String;I)[I",
                method_sig("f", vec!["java.lang.String", "int"], "int[]"),
            ),
            (
                "(Ljava/lang/String;I)[[I",
                method_sig("f", vec!["java.lang.String", "int"], "int[][]"),
            ),
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
