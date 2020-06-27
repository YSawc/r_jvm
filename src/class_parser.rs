use std::fs::File;
use std::io::{BufReader, Read};

pub struct ClassFileReader {
    reader: BufReader<File>,
}

macro_rules! exist {
    ($expr:expr) => {{
        if !$expr {
            return None;
        }
    }};
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

    pub fn read_16(&mut self) -> Option<u16> {
        let mut buf = [0u8; 2];
        match self.reader.read(&mut buf) {
            Ok(sz) => {
                assert_eq!(sz, 4);
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
        let magic: u32 = self.read_u32()?;
        assert_eq!(magic, 0xCAFEBABE);
        Some(())
    }
}
