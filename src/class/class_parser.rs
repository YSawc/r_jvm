use super::class_file;
use super::constant::{index_to_constant_type, Constant, ConstantType};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, Read};
use std::str;

#[derive(Debug)]
pub struct ClassFileReader {
    reader: BufReader<File>,
}

impl ClassFileReader {
    pub fn read_constant(&mut self, ty: ConstantType) -> Option<Constant> {
        match ty {
            ConstantType::Class => self.read_class(),
            ConstantType::Fieldref => self.read_field_ref(),
            ConstantType::Methodref => self.read_method(),
            ConstantType::InterfaceMethodref => self.read_interface_method_ref(),
            ConstantType::String => self.read_string(),
            ConstantType::Integer => self.read_integer(),
            ConstantType::Float => self.read_float(),
            ConstantType::Long => self.read_long(),
            ConstantType::Double => self.read_double(),
            ConstantType::NameAndType => self.read_name_and_type(),
            ConstantType::Utf8 => self.read_utf8(),
            ConstantType::MethodHandle => self.read_method_handle(),
            ConstantType::MethodType => self.read_method_type(),
            ConstantType::InvokeDynamic => self.read_invoke_dynamic(),
        }
    }

    fn read_class(&mut self) -> Option<Constant> {
        let class_index = self.read_u16()?;
        Some(Constant::ClassInfo { class_index })
    }

    fn read_field_ref(&mut self) -> Option<Constant> {
        let class_index = self.read_u16()?;
        let name_and_type_index = self.read_u16()?;
        Some(Constant::FieldrefInfo {
            class_index,
            name_and_type_index,
        })
    }

    fn read_method(&mut self) -> Option<Constant> {
        let class_index = self.read_u16()?;
        let name_and_type_index = self.read_u16()?;
        Some(Constant::MethodInfo {
            class_index,
            name_and_type_index,
        })
    }

    fn read_interface_method_ref(&mut self) -> Option<Constant> {
        let class_index = self.read_u16()?;
        let name_and_type_index = self.read_u16()?;
        Some(Constant::InterfaceMethodrefInfo {
            class_index,
            name_and_type_index,
        })
    }

    fn read_string(&mut self) -> Option<Constant> {
        let string_index = self.read_u16()?;
        Some(Constant::StringInfo { string_index })
    }

    fn read_integer(&mut self) -> Option<Constant> {
        let bytes = self.read_u32()?;
        Some(Constant::IntegerInfo { bytes })
    }

    fn read_float(&mut self) -> Option<Constant> {
        let bytes = self.read_u32()?;
        Some(Constant::FloatInfo { bytes })
    }

    fn read_long(&mut self) -> Option<Constant> {
        let hi_bytes = self.read_u32()?;
        let low_bytes = self.read_u32()?;
        Some(Constant::LongInfo {
            hi_bytes,
            low_bytes,
        })
    }

    fn read_double(&mut self) -> Option<Constant> {
        let hi_bytes = self.read_u32()?;
        let low_bytes = self.read_u32()?;
        Some(Constant::DoubleInfo {
            hi_bytes,
            low_bytes,
        })
    }

    fn read_name_and_type(&mut self) -> Option<Constant> {
        let name_index = self.read_u16()?;
        let descriptor_index = self.read_u16()?;
        Some(Constant::NameAndtypeInfo {
            name_index,
            descriptor_index,
        })
    }

    fn read_utf8(&mut self) -> Option<Constant> {
        let length = self.read_u16()?;
        let mut bytes: Vec<u8> = Vec::with_capacity(length as usize);
        for _ in 0..=length - 1 {
            bytes.push(self.read_u8().unwrap());
        }
        let str = str::from_utf8(&bytes).unwrap().to_string();

        Some(Constant::Utf8Info { length, str })
    }

    fn read_method_handle(&mut self) -> Option<Constant> {
        let reference_kind = self.read_u8()?;
        let referende_index = self.read_u16()?;
        Some(Constant::MethodHandleInfo {
            reference_kind,
            referende_index,
        })
    }

    fn read_method_type(&mut self) -> Option<Constant> {
        let descriptor_index = self.read_u16()?;
        Some(Constant::MethodTypeInfo { descriptor_index })
    }

    fn read_invoke_dynamic(&mut self) -> Option<Constant> {
        let bootstrap_method_attr_index = self.read_u16()?;
        let name_and_type_index = self.read_u16()?;
        Some(Constant::InvokeDynamicInfo {
            bootstrap_method_attr_index,
            name_and_type_index,
        })
    }
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

    pub fn read_u8(&mut self) -> Option<u8> {
        let mut buf = [0u8; 1];
        match self.reader.read(&mut buf) {
            Ok(sz) => {
                assert_eq!(sz, 1);
                Some(buf[0] as u8)
            }
            Err(_) => None,
        }
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

        for _ in 0..=class_file.constant_pool_count {
            let tg = self.read_u8()?;
            let c_ty = index_to_constant_type(tg)?;
            println!("tag : {}", tg);
            println!("info : {:?}", self.read_constant(c_ty));
        }

        Some(())
    }
}
