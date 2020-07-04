pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub info: Attribute,
}

pub struct CodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code_length: u32,
    pub code: *mut Vec<u8>,
    pub exception_table_length: u16,
    pub exception_table: Vec<Exception>,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

pub enum Attribute {
    ConstantValue {
        constant_value: u16,
    },

    Code(CodeAttribute),
    Exceptions {
        number_of_exceptions: u16,
        exception_index_table: Vec<u16>,
    },

    LineNumberTable {
        line_number_table_length: u16,
        line_number_table: Vec<LineNumber>,
    },
    None,
}

pub struct Exception {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}
