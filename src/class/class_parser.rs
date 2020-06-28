use super::class_file;
use std::fs::File;
use std::io::{BufReader, Read};

pub struct ClassFileReader {
    reader: BufReader<File>,
}

impl ClassFileReader {
    pub fn new(filename: &str) -> Option<Self> {
        let file = match File::open(filename) {
            Ok(file) => file,
            Err(_) => return None,
        };
        Some(ClassFileReader {
            reader: BufReader::new(file),
        })
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        let mut buf = [0u8; 2];
        match self.reader.read(&mut buf) {
            Ok(sz) => {
                assert_eq!(sz, 2);
                Some(((buf[0] as u16) << 8) + buf[1] as u16)
            }
            Err(_) => None,
        }
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        let mut buf = [0u8; 4];
        match self.reader.read(&mut buf) {
            Ok(sz) => {
                assert_eq!(sz, 4);
                Some(
                    ((buf[0] as u32) << 24)
                        + ((buf[1] as u32) << 16)
                        + ((buf[2] as u32) << 8)
                        + buf[3] as u32,
                )
            }
            Err(_) => None,
        }
    }

    pub fn read(&mut self) -> Option<()> {
        let mut class_file = class_file::ClassFile::new();
        class_file.magic = self.read_u32()?;
        assert_eq!(class_file.magic, 0xCAFEBABE);
        class_file.minor_version = self.read_u16()?;
        println!("minor_version : {}", class_file.minor_version);
        class_file.major_version = self.read_u16()?;
        println!("major_version : {}", class_file.major_version);
        class_file.constant_pool_count = self.read_u16()?;
        println!("constant_pool_count : {}", class_file.constant_pool_count);
        Some(())
    }
}
