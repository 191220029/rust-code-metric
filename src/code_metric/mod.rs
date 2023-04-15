use std::path::PathBuf;

use xlsxwriter::Workbook;

pub mod code_type;

pub struct Collector {
    // code_elements: Vec<CodeType>
    pub function_def: u32,
    pub method_def: u32,
    pub function_call: u32,
    pub method_call: u32,
    pub struct_def: u32,
    pub trait_def: u32,
    pub macro_def: u32,
    pub macro_use: u32,
    pub match_block: u32,
    pub ref_op: u32,
    pub field_access: u32,
    pub loops: u32,
    pub genrics: u32,
    pub lines: u128,
    pub files: u32,
}

impl Collector {
    pub fn new() -> Self {
        // Collector { code_elements: vec![] }
        Collector { function_def: 0, method_def: 0, function_call: 0, method_call: 0, struct_def: 0, trait_def: 0, macro_def: 0, macro_use: 0, lines: 0, files: 0, match_block: 0, ref_op: 0, field_access: 0, loops: 0, genrics: 0 }
    }

    // pub fn append(&mut self, code: CodeType) {
    //     self.code_elements.push(code);
    // }

    // pub fn incr_func_call(&mut self, label: &String) {
    //     unimplemented!()
    // }

    // pub fn incr_method_call(&mut self, label: &String) {
    //     unimplemented!()
    // }

    pub fn to_xls(&self, out_path: &PathBuf) {
        let workbook = match Workbook::new(out_path.to_str().unwrap()) {
            Ok(wb) => wb,
            Err(e) => {
                eprintln!("Fail to write workbook {}\n{}", out_path.display(), e);
                return;
            }
        };
        let mut sheet = match workbook.add_worksheet(Some("code_metrics")){
            Ok(ws) => ws,
            Err(e) => {
                eprintln!("Fail to create sheet.\n{}", e);
                return;
            }
        };

        sheet.write_string(0, 0, "files", None).unwrap();
        sheet.write_string(0, 1, "lines", None).unwrap();
        sheet.write_string(0, 2, "function_def", None).unwrap();
        sheet.write_string(0, 3, "function_call", None).unwrap();
        sheet.write_string(0, 4, "method_def", None).unwrap();
        sheet.write_string(0, 5, "method_call", None).unwrap();
        sheet.write_string(0, 6, "macro_def", None).unwrap();
        sheet.write_string(0, 7, "macro_use", None).unwrap();
        sheet.write_string(0, 8, "structs", None).unwrap();
        sheet.write_string(0, 9, "field_access", None).unwrap();
        sheet.write_string(0, 10, "traits", None).unwrap();
        sheet.write_string(0, 11, "match_block", None).unwrap();
        sheet.write_string(0, 12, "reference", None).unwrap();
        sheet.write_string(0, 13, "loops", None).unwrap();
        sheet.write_string(0, 14, "generics", None).unwrap();

        sheet.write_string(1, 0, self.files.to_string().as_str(), None).unwrap();
        sheet.write_string(1, 1, self.lines.to_string().as_str(), None).unwrap();
        sheet.write_string(1, 2, self.function_def.to_string().as_str(), None).unwrap();
        sheet.write_string(1, 3, self.function_call.to_string().as_str(), None).unwrap();
        sheet.write_string(1, 4, self.method_def.to_string().as_str(), None).unwrap();
        sheet.write_string(1, 5, self.method_call.to_string().as_str(), None).unwrap();
        sheet.write_string(1, 6, self.macro_def.to_string().as_str(), None).unwrap();
        sheet.write_string(1, 7, self.macro_use.to_string().as_str(), None).unwrap();
        sheet.write_string(1, 8, self.struct_def.to_string().as_str(), None).unwrap();
        sheet.write_string(1, 9, self.field_access.to_string().as_str(), None).unwrap();
        sheet.write_string(1, 10, self.trait_def.to_string().as_str(), None).unwrap();
        sheet.write_string(1, 11, self.match_block.to_string().as_str(), None).unwrap();
        sheet.write_string(1, 12, self.ref_op.to_string().as_str(), None).unwrap();
        sheet.write_string(1, 13, self.loops.to_string().as_str(), None).unwrap();
        sheet.write_string(1, 14, self.genrics.to_string().as_str(), None).unwrap();

        // unimplemented!()
    }
}