use std::path::PathBuf;

use self::code_type::CodeType;

pub mod code_type;

pub struct Collector {
    code_elements: Vec<CodeType>
}

impl Collector {
    pub fn new() -> Self {
        Collector { code_elements: vec![] }
    }

    pub fn append(&mut self, code: CodeType) {
        self.code_elements.push(code);
    }

    pub fn incr_func_call(&mut self, label: &String) {
        unimplemented!()
    }

    pub fn incr_method_call(&mut self, label: &String) {
        unimplemented!()
    }

    pub fn to_xls(&self, out_path: &PathBuf) {
        unimplemented!()
    }
}