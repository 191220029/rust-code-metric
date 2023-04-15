use std::{path::PathBuf, fs::read_dir,};


use xlsxwriter::{Workbook, Worksheet};

use crate::code_metric::{Collector, parse_rs::parse_rs_file};

/// Iterate a benchmark suit directory and collect code-metric for each benchmark.
/// A benchmark suit directory is supposed to have a structure like:
/// + benchmark_suit
/// |----benchmark_1(directory)
/// |----benchmark_2(directory)
/// |----...
pub fn iter_benchmark_suit(p: &PathBuf, out_path: &PathBuf) -> anyhow::Result<()> {
    let workbook = match Workbook::new(out_path.to_str().unwrap()) {
        Ok(wb) => wb,
        Err(e) => {
            eprintln!("Fail to write workbook {}\n{}", out_path.display(), e);
            return Ok(());
        }
    };
    let mut sheet = match workbook.add_worksheet(Some("code_metrics")){
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("Fail to create sheet.\n{}", e);
            return Ok(());
        }
    };

    write_xlsx_header(&mut sheet);

    let mut row = 1;
    for entry in read_dir(p)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let benchmark_name = entry.file_name();
            sheet.write_string(row, 0, benchmark_name.to_str().unwrap(), None).unwrap();
            
            let mut collector = Collector::new();
            iter_directory_recursively(&entry.path(), &mut collector)?;
            collector.write_to_sheet(&mut sheet, row, 1);

            row += 1;
        }
    }

    Ok(())
}

pub fn iter_directory_recursively(p: &PathBuf, collector: &mut Collector) -> anyhow::Result<()> {
    for entry in read_dir(p)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            if entry.file_name().to_str().unwrap() == "target" {
                // println!("{}", entry.file_name().to_str().unwrap());
                continue;
            }
            iter_directory_recursively(&entry.path(), collector)?;
        }
        else if entry.file_type()?.is_file()  && entry.path().extension().is_some()
            && entry.path().extension().unwrap() == "rs" {
            // eprintln!("Found rust file {}", entry.path().display());
            
            parse_rs_file(&entry.path(), collector);
        }
    }

    Ok(())
}

fn write_xlsx_header(sheet: &mut Worksheet) {
    let headers = [
        "files",
        "lines",
        "function_def",
        "function_call",
        "method_def",
        "method_call",
        "macro_def",
        "macro_use",
        "structs",
        "field_access",
        "traits",
        "match_block",
        "reference",
        "loops",
        "generics",
        "closure_def",
        "dyn_use",
    ];

    let mut i = 1;
    headers.iter().for_each(|s| {
        sheet.write_string(0, i, s, None).unwrap();
        i += 1;
    })
}