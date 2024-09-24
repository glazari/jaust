use anyhow::anyhow;
use anyhow::Result;

use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub(super) struct MethodSignature {
    pub(super) name: String,
    pub(super) args: Vec<String>,
    pub(super) return_type: String,
}

pub(super) fn parse_method_descriptor(descriptor: &str, name: String) -> MethodSignature {
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

pub(super) fn parse_type_descriptor(chars: &mut Pchars) -> Result<String> {
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
