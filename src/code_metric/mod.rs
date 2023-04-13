use std::path::PathBuf;

use self::code_type::CodeType;

pub mod code_type;

pub struct Collector {
    code_elements: Vec<CodeType>
}

impl Collector {
    pub fn append(&mut self, code: CodeType) {
        self.code_elements.push(code);
    }

    pub fn to_xls(&self, out_path: &PathBuf) {
        
    }
}