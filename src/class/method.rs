use super::attribute::{Attribute, AttributeInfo};

#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl MethodInfo {
    pub fn get_code_attribute(&self) -> Option<&Attribute> {
        for i in 0..self.attributes_count as usize {
            match self.attributes[i].info {
                Attribute::Code { .. } => return Some(&self.attributes[i].info),
                _ => {}
            };
        }
        None
    }
}
