use super::super::class::class_file::ClassFile;
use std::collections::HashMap;

pub type GcType<T> = *mut T;

#[derive(Debug)]
pub struct ClassHeap {
    pub class_map: HashMap<String, ClassFile>,
}

impl ClassHeap {
    pub fn new() -> Self {
        ClassHeap {
            class_map: HashMap::default(),
        }
    }

    pub fn get_class(&self, class_name: &str) -> Option<&ClassFile> {
        self.class_map.get(class_name).and_then(|c| Some(c))
    }

    pub fn insert_class(&mut self, class_name: &str, class_file: ClassFile) -> Option<()> {
        self.class_map.insert(class_name.to_string(), class_file);
        Some(())
    }
}
