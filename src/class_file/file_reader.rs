use std::fs::File;
use std::io::Read;
use std::io::Result;

pub struct FileReader {
    file: File,
}

// The type names that are used in the class file documentation
type U1 = u8;
type U2 = [u8; 2];
type U4 = [u8; 4];

impl FileReader {
    pub fn new(filename: &str) -> Result<FileReader> {
        let f = FileReader {
            file: File::open(filename)?,
        };
        Ok(f)
    }

    pub fn read_u1(&mut self) -> Result<U1> {
        let mut buf = [0; 1];
        self.file.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    pub fn read_u2(&mut self) -> Result<U2> {
        let mut buf = [0; 2];
        self.file.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn read_u2_to_u16(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        self.file.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    pub fn read_u4(&mut self) -> Result<U4> {
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn read_string(&mut self) -> Result<String> {
        let length = self.read_u2_to_u16()?;
        let mut buf = vec![0; length as usize];
        self.file.read_exact(&mut buf)?;
        let s = String::from_utf8(buf.clone()).unwrap();
        Ok(s)
    }
}
