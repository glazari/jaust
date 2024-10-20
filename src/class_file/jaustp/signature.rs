use anyhow::anyhow;
use anyhow::Result;
/// Classes, Methods and fields can have a signature attribute
/// This signature will describe the type with things that the descriptor does not have
/// For example, generics do not exist in the descriptor (for backwards compatibility)
/// But they do exist in the signature.
/// Another example is for Enum constructors where the descriptor shows the 2 extra arguments that
/// the compiler adds (variant name and ordinal) but the signature does not.
///
/// The format of signatures is described in Chapter 4.7.9.1 of the JVM spec
/// https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.7.9
///
///
use std::iter::Peekable;
use std::str::Chars;
use std::string::ToString;

#[derive(Debug)]
pub enum JavaTypeSignature {
    ReferenceTypeSignature(ReferenceTypeSignature),
    BaseTypeSignature(String),
}

#[derive(Debug)]
pub enum ReferenceTypeSignature {
    ClassTypeSignature(ClassTypeSignature),
    TypeVariableSignature(String),
    ArrayTypeSignature(Box<JavaTypeSignature>),
}

#[derive(Debug)]
pub struct ClassTypeSignature {
    pub package_specifiers: Vec<String>, //
    pub simple_class_type_signature: SimpleClassTypeSignature,
    pub class_type_signature_suffix: Vec<SimpleClassTypeSignature>,
}

#[derive(Debug)]
pub struct SimpleClassTypeSignature {
    pub identifier: String,
    pub type_arguments: Vec<ReferenceTypeSignature>,
}

type Pchars<'a> = Peekable<Chars<'a>>;

pub fn parse_signature(signature: &str) -> Result<JavaTypeSignature> {
    let mut chars = signature.chars().peekable();
    parse_java_type_signature(&mut chars)
}

fn parse_java_type_signature(chars: &mut Pchars) -> Result<JavaTypeSignature> {
    let out = match chars.peek() {
        Some('L') => {
            JavaTypeSignature::ReferenceTypeSignature(parse_reference_type_signature(chars)?)
        }
        Some(_) => JavaTypeSignature::BaseTypeSignature(parse_base_type_signature(chars)?),
        None => return Err(anyhow!("Unexpected end of signature")),
    };
    Ok(out)
}

fn parse_base_type_signature(chars: &mut Pchars) -> Result<String> {
    let c = chars.next().unwrap();
    let out = match c {
        'I' => "int".to_string(),
        'J' => "long".to_string(),
        'F' => "float".to_string(),
        'D' => "double".to_string(),
        'S' => "short".to_string(),
        'B' => "byte".to_string(),
        'C' => "char".to_string(),
        'Z' => "boolean".to_string(),
        'V' => "void".to_string(),
        _ => return Err(anyhow!("Invalid base type signature")),
    };
    Ok(out)
}

fn parse_reference_type_signature(chars: &mut Pchars) -> Result<ReferenceTypeSignature> {
    let c = chars.peek().ok_or(anyhow!("Unexpected end of signature"))?;
    match c {
        'L' => Ok(ReferenceTypeSignature::ClassTypeSignature(
            parse_class_type_signature(chars)?,
        )),
        'T' => {
            chars.next();
            Ok(ReferenceTypeSignature::TypeVariableSignature(
                parse_identifier(chars),
            ))
        }
        '[' => {
            chars.next();
            Ok(ReferenceTypeSignature::ArrayTypeSignature(Box::new(
                parse_java_type_signature(chars)?,
            )))
        }
        _ => Err(anyhow!("Invalid reference type signature")),
    }
}

fn expect_char(chars: &mut Pchars, expected: char) -> Result<()> {
    let c = chars.next().expect("Unexpected end of signature");
    if c != expected {
        return Err(anyhow!("Expected {} but got {}", expected, c));
    }
    Ok(())
}

fn parse_class_type_signature(chars: &mut Pchars) -> Result<ClassTypeSignature> {
    expect_char(chars, 'L')?;
    let package_specifiers = parse_package_specifiers(chars);
    let identifier = package_specifiers.last().unwrap().clone();
    let package_specifiers = package_specifiers[..package_specifiers.len() - 1].to_vec();
    let package_specifiers = package_specifiers.to_vec();
    let simple_class_type_signature = parse_simple_class_type_signature(chars, identifier)?;
    let mut class_type_signature_suffix = Vec::new();
    loop {
        match chars.peek() {
            Some('.') => {
                chars.next();
                let identifier = parse_identifier(chars);
                class_type_signature_suffix
                    .push(parse_simple_class_type_signature(chars, identifier)?);
            }
            Some(';') => {
                break;
            }
            _ => break,
        }
    }
    expect_char(chars, ';')?;
    let out = ClassTypeSignature {
        package_specifiers,
        simple_class_type_signature,
        class_type_signature_suffix,
    };
    Ok(out)
}

fn parse_simple_class_type_signature(
    chars: &mut Pchars,
    identifier: String,
) -> Result<SimpleClassTypeSignature> {
    let type_arguments = parse_type_arguments(chars)?;
    Ok(SimpleClassTypeSignature {
        identifier,
        type_arguments,
    })
}

fn parse_type_arguments(chars: &mut Pchars) -> Result<Vec<ReferenceTypeSignature>> {
    let mut type_arguments = Vec::new();
    if *chars.peek().unwrap() != '<' {
        return Ok(type_arguments);
    }

    expect_char(chars, '<')?;
    loop {
        let c = chars.peek().unwrap();
        match c {
            '>' => break,
            _ => {
                let type_argument = parse_type_argument(chars)?;
                type_arguments.push(type_argument);
            }
        }
    }

    expect_char(chars, '>')?;
    Ok(type_arguments)
}

fn parse_type_argument(chars: &mut Pchars) -> Result<ReferenceTypeSignature> {
    let c = chars.peek().unwrap();
    match c {
        '*' => {
            chars.next();
            Ok(ReferenceTypeSignature::ArrayTypeSignature(Box::new(
                JavaTypeSignature::BaseTypeSignature("*".to_string()),
            )))
        }
        '+' | '-' => {
            chars.next();
            let reference_type_signature = parse_reference_type_signature(chars)?;
            Ok(reference_type_signature)
        }
        _ => {
            let reference_type_signature = parse_reference_type_signature(chars)?;
            Ok(reference_type_signature)
        }
    }
}

fn parse_package_specifiers(chars: &mut Pchars) -> Vec<String> {
    let mut package_specifiers = Vec::new();
    package_specifiers.push(parse_identifier(chars));
    loop {
        let c = chars.peek().unwrap();
        match c {
            '/' => {
                chars.next();
                package_specifiers.push(parse_identifier(chars));
            }
            _ => break,
        }
    }
    package_specifiers
}

fn parse_identifier(chars: &mut Pchars) -> String {
    let mut identifier = String::new();
    loop {
        let c = chars.peek().unwrap();
        match c {
            '/' | ';' | '<' | '>' | ':' | '[' => {
                break;
            }

            _ => identifier.push(*c),
        }
        chars.next();
    }
    identifier
}

impl JavaTypeSignature {
    pub fn to_string(&self) -> String {
        match self {
            JavaTypeSignature::ReferenceTypeSignature(r) => r.to_string(),
            JavaTypeSignature::BaseTypeSignature(b) => b.to_string(),
        }
    }
}

impl ReferenceTypeSignature {
    pub fn to_string(&self) -> String {
        match self {
            ReferenceTypeSignature::ClassTypeSignature(c) => c.to_string(),
            ReferenceTypeSignature::TypeVariableSignature(t) => t.to_string(),
            ReferenceTypeSignature::ArrayTypeSignature(a) => a.to_string(),
        }
    }
}

impl ClassTypeSignature {
    pub fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.package_specifiers.join("."));
        if self.package_specifiers.len() > 0 {
            out.push_str(".");
        }
        out.push_str(&self.simple_class_type_signature.to_string());
        for suffix in &self.class_type_signature_suffix {
            out.push_str(".");
            out.push_str(&suffix.to_string());
        }
        out
    }
}

impl SimpleClassTypeSignature {
    pub fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.identifier);
        if self.type_arguments.len() == 0 {
            return out;
        }
        out.push_str("<");
        let type_arguments = self
            .type_arguments
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>()
            .join("");
        out.push_str(&type_arguments);
        out.push_str(">");
        out
    }
}

impl ToString for JavaTypeSignature {
    fn to_string(&self) -> String {
        self.to_string()
    }
}

///
///ReferenceTypeSignature:
///     ClassTypeSignature
///     TypeVariableSignature
///     ArrayTypeSignature
///
///ClassTypeSignature:
/// L [PackageSpecifier] SimpleClassTypeSignature {ClassTypeSignatureSuffix} ;
///
///PackageSpecifier:
///     Identifier / {PackageSpecifier}
///
///SimpleClassTypeSignature:
///     Identifier [TypeArguments]
///
///TypeArguments:
///     < TypeArgument {TypeArgument} >
///
///TypeArgument:
///     [WildcardIndicator] ReferenceTypeSignature
///     *
///
///WildcardIndicator:
/// +
/// -
///
///ClassTypeSignatureSuffix:
/// . SimpleClassTypeSignature
///
///TypeVariableSignature:
/// T Identifier ;
/// ArrayTypeSignature:
/// [ JavaTypeSignature
mod tests {
    use super::*;

    macro_rules! signature_print_tests {
        ($($name:ident, $input:expr, $expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let signature = parse_signature($input).unwrap();
                    assert_eq!(signature.to_string(), $expected);
                }
            )*

        }
    }

    signature_print_tests! {
        enum_sig, "Ljava/lang/Enum<LEnumTest;>;", "java.lang.Enum<EnumTest>",
        //enum_constructor_sig, "(Ljava/lang/String;Ljava/lang/String;)V", "(java.lang.String, java.lang.String)",

    }
}
