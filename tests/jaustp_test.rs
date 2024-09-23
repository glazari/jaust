use jaust::class_file;

use text_diff::assert_diff;

use std::process::Command;
use std::sync::Once;
use std::fs;
use anyhow::Result;

static INIT: Once = Once::new();

fn initialize() {
    INIT.call_once(|| {
        // clean class files
        fs::remove_dir_all("./test_class_files").unwrap_or(());

        // java files to compile (all in test_files recursively that end in .java)
        let files = fs::read_dir("./test_files").unwrap()
            .map(|f| f.unwrap().path())
            .filter(|f| f.extension().unwrap() == "java");

        // compile example classes
        let out = Command::new("javac")
            .arg("-d")
            .arg("./test_class_files")
            .args(files) 
            .output()
            .expect("failed to execute javac");

        if !out.status.success() {
            let mut msg = format!("javac failed: {:?}", out.status);
            msg.push_str(&format!("stdout:\n{}", String::from_utf8(out.stdout).unwrap()));
            msg.push_str(&format!("stderr:\n{}", String::from_utf8(out.stderr).unwrap()));
            panic!("{}", msg);
        }


    });
}


fn javap_summary(file: &str) -> Result<String> {
    let javap_out = Command::new("javap")
        .arg("-private")
        .arg(file)
        .output()
        .expect("failed to execute javap");
    if !javap_out.status.success() {
        let mut msg = format!("javap failed: {:?}", javap_out.status);
        msg.push_str(&format!("stdout:\n{}", String::from_utf8(javap_out.stdout)?));
        msg.push_str(&format!("stderr:\n{}", String::from_utf8(javap_out.stderr)?));
        return Err(anyhow::anyhow!(msg));
    }
    let out = String::from_utf8(javap_out.stdout)?;
    Ok(out)
}

fn jaustp_test_template(file: &str) {
    initialize();
    let cf = class_file::read_class_file(file).unwrap();
    let ops = class_file::JavapOptions {
        private: true,
        code: false,
    };
    let jaustp_out = class_file::jaustp_summary(&cf, &ops);
    let javap_out = javap_summary(file).unwrap();
    assert_diff(&jaustp_out, &javap_out, "\n", 0);
}

macro_rules! javap_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            jaustp_test_template($value);
        }
    )*
    }
}

javap_tests! {
    jaustp_example_test: "./test_class_files/Example.class",
    jaustp_enum_test: "./test_class_files/EnumTest.class",
    jaustp_b_test: "./test_class_files/B.class",
    jaustp_c_test: "./test_class_files/C.class",
    jaustp_my_class1_test: "./test_class_files/MyClass1.class",
    jaustp_my_class2_test: "./test_class_files/MyClass2.class",
    jaustp_com_example_record_test: "./test_class_files/com/example/RecordTest.class",
}
