use std::{path::PathBuf, fs::{read_dir, File}, io::{Read, BufReader, BufRead}, vec, any::Any};

use tree_sitter::{Parser, Tree, TreeCursor};

use crate::code_metric::Collector;

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
    // unimplemented!()
}

fn parse_rs_file(p: &PathBuf, collector: &mut Collector) {
    let mut file = match File::open(p) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Fail to open file {}\n{}", p.display(), e);
            return;
        }
    };

    let mut codes = String::new();
    match file.read_to_string(&mut codes) {
        Ok(len) => {
            if len == 0 {
                return;
            }
        },
        Err(e) => {
            eprintln!("Fail to read file {}.\n{}", p.display(), e);
            return;
        }
    }

    collector.files += 1;
    let reader = BufReader::new(File::open(p).unwrap());
    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.len() > 0 {
                    collector.lines += 1;
                }
            }
            Err(e) =>   eprintln!("{}", e),
        };
    }



    let rust = tree_sitter_rust::language();
    let mut parser = Parser::new();
    match parser.set_language(rust) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Fail to set parser.\n{}", e);
            return;
        },
    };

    match parser.parse(&codes, None) {
        Some(tree) => {
            scan_syntax_tree(&tree, collector, &codes);
        },
        None => {
            eprintln!("Fail to parse {}.", p.display());
            return;
        }
    }
}

fn scan_syntax_tree(tree: &Tree, collector: &mut Collector, text: &String) { 
    let root = tree.root_node();
    let mut cursor = root.walk();

    bfs_syntax_tree(&mut cursor, collector);
}

fn bfs_syntax_tree(cursor: &mut TreeCursor, collector: &mut Collector) {
    // let mut layer = vec![];
    loop {
        let node = cursor.node();

        // print!("{} ", node.kind());

        // if node.kind() == "field_identifier" {
        //     print!("-{}", node.start_position());
        // }
        // else if node.kind() == "call_expression" {
        //     for i in 0..node.child_count() {
        //         print!("-{}", node.child(i).unwrap().kind());
        //     }
        // }

        match node.kind() {
            "line_comment" => 
                collector.lines -= 1,
            "struct" | "enum" => 
                collector.struct_def += 1,
            "tuple_struct_pattern" | "struct_item" | "enum_item" | "enum_variant_list" | "enum_variant" | "struct_expression" =>
                (),
            "impl" | "impl_item" => 
                (),
            "self_parameter" => {
                collector.method_def += 1;
                collector.function_def -= 1;
            }
            "reference_expression" => 
                collector.ref_op += 1,
            "field_expression" => 
                collector.field_access += 1,
            "match_expression" => 
                collector.match_block += 1,
            "function_item" => 
                collector.function_def += 1,
            "call_expression" => {
                if node.child(0).unwrap().kind() == "field_expression" {
                    collector.method_call += 1;
                }
                else {
                    collector.function_call += 1;
                }
                // println!("{}\n{} {}", node.to_sexp(), node.start_position(), node.end_position());
            }
            "macro_invocation" => 
                collector.macro_use += 1,
            "generic_type" =>
                collector.genrics += 1,
            "source_file" => 
                (),
            "mod" | "use" | "use_declaration" | "use_list" | "scoped_use_list" | "mod_item" | "crate" => 
                (),
            "token_tree" => 
                (),
            "block" | "break" |  "break_expression" | "return_expression" | "return" => 
                (),
            "while_expression" | "loop_expression" | "for_expression" => 
                collector.loops += 1,
            "while" | "loop" | "continue" | "continue_expression"| "for" =>
                (),
            "if_expression" | "if" | "else" | "else_clause" => 
                (),
            "expression_statement" | "empty_statement" =>
                (),
            "identifier" | "type_identifier" | "type_arguments" | "reference_type" 
            | "primitive_type" | "arguments" | "parameters" | "parameter" 
            | "field_identifier" | "scoped_identifier" | "field_initializer_list"
            | "scoped_type_identifier" | "mutable_specifier" => 
                (),
            "match_pattern" | "match_arm" | "match" | "match_block" =>
                (),
            "string_literal" | "integer_literal" => 
                (),
            "or_pattern" => 
                (),
            "(" | ")" | "=>" | "."  | "{" | "}" | "&" | "|" | "\"" | ","
             | "!" | "_" | "::" | ";" | "=" | "+=" | "-=" | ":" | "&&" | "?"
             | "==" | "<" | ">" | "->" | "#" | "[" | "]"=> 
                (),
            "fn" =>
                (),
            "attribute_item" | "attribute" => 
                (),
            "let" | "let_declaration" | "struct_pattern" | "shorthand_field_identifier"
             | "field_pattern" | "field_declaration" | "field_declaration_list"
             | "field_initializer" | "declaration_list" => 
                (),
            "escape_sequence" => 
                (),
            "unit_expression" | "unit_type" | "unary_expression" | "compound_assignment_expr" | "binary_expression" => 
                (),
            "try_expression" => 
                (),
            "in" => 
                (),
            "visibility_modifier" | "pub" =>
                (),
            "self" => 
                (),
            _ => 
                println!("{}", node.kind()),
        };
        
        // layer.push(cursor.clone());


        if !cursor.goto_next_sibling() {     
            if cursor.goto_parent() {
                cursor.goto_first_child();
            }

            if cursor.goto_first_child() {
                bfs_syntax_tree(cursor, collector);
                cursor.goto_parent();
            }

            while cursor.goto_next_sibling() {            
                if cursor.goto_first_child() {
                    bfs_syntax_tree(cursor, collector);
                    cursor.goto_parent();
                }
            }
            // layer.into_iter().for_each(|mut c| {
            //     if c.goto_first_child() {
            //         bfs_syntax_tree(cursor, collector);
            //     }
            // });
        
            break;
        }
        // dfs logic
        // if cursor.goto_first_child() {
        //     dfs_syntax_tree(cursor, collector);
        // }
        // if !cursor.goto_next_sibling() {
        //     break;
        // }
    }
    // dfs logic
    // cursor.goto_parent();
}