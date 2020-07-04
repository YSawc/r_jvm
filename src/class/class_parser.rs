use super::attribute::{Attribute, AttributeInfo, CodeAttribute, Exception, LineNumber};
use super::class_file::ClassFile;
use super::constant::{index_to_constant_type, Constant, ConstantType};
use super::field::FieldInfo;
use super::method::MethodInfo;
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

// FieldInfo
impl ClassFileReader {
    fn read_field_info(&mut self, constant_pool: &Vec<Constant>) -> Option<FieldInfo> {
        let access_flags = self.read_u16()?;
        let name_index = self.read_u16()?;
        let descriptor_index = self.read_u16()?;
        let attributes_count = self.read_u16()?;
        let mut attributes = vec![];
        for _ in 0..attributes_count {
            attributes.push(self.read_attribute_info(constant_pool)?);
        }

        Some(FieldInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        })
    }
}

// method
impl ClassFileReader {
    fn read_method_info(&mut self, constant_pool: &Vec<Constant>) -> Option<MethodInfo> {
        let access_flags = self.read_u16()?;
        let name_index = self.read_u16()?;
        let descriptor_index = self.read_u16()?;
        let attributes_count = self.read_u16()?;
        let mut attributes = vec![];
        for _ in 0..attributes_count {
            let a_attribute = self.read_attribute_info(constant_pool)?;
            attributes.push(a_attribute);
        }

        Some(MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        })
    }
}

// attribute
impl ClassFileReader {
    fn read_attribute_info(&mut self, constant_pool: &Vec<Constant>) -> Option<AttributeInfo> {
        let attribute_name_index = self.read_u16()?;
        let attribute_length = self.read_u32()?;
        let name = constant_pool[attribute_name_index as usize - 1].get_utf8()?;
        let info = match name.as_str() {
            "ConstantValue" => self.read_constant_value_attribute()?,
            "Code" => self.read_code_attribute(constant_pool)?,
            "ExceptionsAttribute" => self.read_exceptions_attribute()?,
            "LineNumberTable" => self.read_line_number_table_attribute()?,
            _ => Attribute::None,
        };

        Some(AttributeInfo {
            attribute_name_index,
            attribute_length,
            info,
        })
    }

    fn read_constant_value_attribute(&mut self) -> Option<Attribute> {
        let constant_value = self.read_u16()?;
        Some(Attribute::ConstantValue { constant_value })
    }

    fn read_code_attribute(&mut self, constant_pool: &Vec<Constant>) -> Option<Attribute> {
        let max_stack = self.read_u16()?;
        let max_locals = self.read_u16()?;
        let code_length = self.read_u32()?;
        let mut code = vec![];
        for _ in 0..code_length {
            code.push(self.read_u8()?);
        }
        let exception_table_length = self.read_u16()?;
        let mut exception_table = vec![];
        for _ in 0..exception_table_length {
            exception_table.push(self.read_exception()?);
        }
        let attributes_count = self.read_u16()?;
        let mut attributes = vec![];
        for _ in 0..attributes_count {
            attributes.push(self.read_attribute_info(constant_pool)?)
        }
        Some(Attribute::Code(CodeAttribute {
            max_stack,
            max_locals,
            code_length,
            code: Box::into_raw(Box::new(code)),
            exception_table_length,
            exception_table,
            attributes_count,
            attributes,
        }))
    }

    fn read_exception(&mut self) -> Option<Exception> {
        let start_pc = self.read_u16()?;
        let end_pc = self.read_u16()?;
        let handler_pc = self.read_u16()?;
        let catch_type = self.read_u16()?;
        Some(Exception {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        })
    }

    fn read_exceptions_attribute(&mut self) -> Option<Attribute> {
        let number_of_exceptions = self.read_u16()?;
        let mut exception_index_table = vec![];
        for _ in 0..number_of_exceptions {
            exception_index_table.push(self.read_u16()?)
        }
        Some(Attribute::Exceptions {
            number_of_exceptions,
            exception_index_table,
        })
    }

    fn read_line_number_table_attribute(&mut self) -> Option<Attribute> {
        let line_number_table_length = self.read_u16()?;
        let mut line_number_table = vec![];
        for _ in 0..line_number_table_length {
            line_number_table.push(self.read_line_number()?);
        }

        Some(Attribute::LineNumberTable {
            line_number_table_length,
            line_number_table,
        })
    }

    fn read_line_number(&mut self) -> Option<LineNumber> {
        let start_pc = self.read_u16()?;
        let line_number = self.read_u16()?;

        Some(LineNumber {
            start_pc,
            line_number,
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

        for _ in 0..=class_file.constant_pool_count - 2 {
            let tg = self.read_u8()?;
            let c_ty = index_to_constant_type(tg)?;
            println!("tag : {}", tg);
            println!("info : {:?}", self.read_constant(c_ty));
        }

        println!("test");
        class_file.access_flags = self.read_u16()?;
        println!("access_flags : {}", class_file.access_flags);

        class_file.this_class = self.read_u16()?;
        println!("this_class : {}", class_file.this_class);

        class_file.super_class = self.read_u16()?;
        println!("super_class : {}", class_file.super_class);

        class_file.interfaces_count = self.read_u16()?;
        println!("interfaces_count : {}", class_file.interfaces_count);

        for _ in 0..=class_file.interfaces_count {}

        class_file.fields_count = self.read_u16()?;
        println!("fields_count : {}", class_file.fields_count);

        for _ in 0..=class_file.fields_count {}

        class_file.methods_count = self.read_u16()?;
        println!("methods_count : {}", class_file.methods_count);

        for _ in 0..=class_file.methods_count {}

        class_file.attributes_count = self.read_u16()?;
        println!(
            "attributes_count interfaces_count : {}",
            class_file.attributes_count
        );
        let mut methods = vec![];
        for _ in 0..methods_count {
            methods.push(self.read_method_info(&constant_pool)?);
        }

        for _ in 0..=class_file.attributes_count {}
        Some(())

        // for _ in 0..attributes_count {}
        Some(ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool_count,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces_count,
            interfaces,
            fields_count,
            fields: vec![],
            methods_count,
            methods,
            attributes_count,
            attributes: vec![],
        })
    }
}
