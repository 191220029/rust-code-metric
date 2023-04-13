use std::{path::PathBuf, fs::{read_dir, File, read_to_string}, io::{BufReader, BufRead}};

use syn::{__private::TokenStream, buffer::TokenBuffer};

use crate::code_metric::Collector;

pub fn iter_directory_recursively(p: &PathBuf, collector: &mut Collector) -> anyhow::Result<()> {
    for entry in read_dir(p)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            iter_directory_recursively(&entry.path(), collector)?;
        }
        else if entry.file_type()?.is_file()  && entry.path().extension().is_some()
            && entry.path().extension().unwrap() == "rs" {
            // eprintln!("Found rust file {}", entry.path().display());
            
            scan_rs_file(&entry.path(), collector);

        }
    }

    Ok(())
    // unimplemented!()
}

fn scan_rs_file(p: &PathBuf, collector: &mut Collector) {
    let file = match File::open(p) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Fail to read file {}", p.display());
            return;
        }
    };
    
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    while let Ok(len) = reader.read_line(&mut buffer) {
        if len == 0 {
            break; // Break out of the loop when the end of the file is reached.
        }
      
        // Do something with the line
        println!("{}: {}", p.display(), buffer);
      
        buffer.clear(); // Clear the buffer for the next line to be read.
    }

    
}