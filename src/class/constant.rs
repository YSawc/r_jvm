pub enum ConstantType {
    Class,
    Fieldref,
    Methodref,
    InterfaceMethodref,
    String,
    Integer,
    Float,
    Long,
    Double,
    NameAndType,
    Utf8,
    MethodHandle,
    MethodType,
    InvokeDynamic,
}

pub fn index_to_constant_type(val: u8) -> Option<ConstantType> {
    match val {
        7 => Some(ConstantType::Class),
        9 => Some(ConstantType::Fieldref),
        10 => Some(ConstantType::Methodref),
        11 => Some(ConstantType::InterfaceMethodref),
        8 => Some(ConstantType::String),
        3 => Some(ConstantType::Integer),
        4 => Some(ConstantType::Float),
        5 => Some(ConstantType::Long),
        6 => Some(ConstantType::Double),
        12 => Some(ConstantType::NameAndType),
        1 => Some(ConstantType::Utf8),
        15 => Some(ConstantType::MethodHandle),
        16 => Some(ConstantType::MethodType),
        18 => Some(ConstantType::InvokeDynamic),
        _ => None,
    }
}

#[derive(Debug)]
pub enum Constant {
    ClassInfo {
        class_index: u16,
    },
    FieldrefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    MethodInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodrefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    StringInfo {
        string_index: u16,
    },
    IntegerInfo {
        bytes: u32,
    },
    FloatInfo {
        bytes: u32,
    },
    LongInfo {
        hi_bytes: u32,
        low_bytes: u32,
    },
    DoubleInfo {
        hi_bytes: u32,
        low_bytes: u32,
    },
    NameAndtypeInfo {
        name_index: u16,
        descriptor_index: u16,
    },
    Utf8Info {
        length: u16,
        bytes: u8,
    },
    MethodHandleInfo {
        reference_kind: u8,
        referende_index: u16,
    },
    MethodTypeInfo {
        descriptor_index: u16,
    },
    InvokeDynamicInfo {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
}
