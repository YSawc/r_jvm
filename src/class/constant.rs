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
