use tree_sitter::{Parser, Tree, TreeCursor};
use std::{path::PathBuf, fs::File, io::{Read, BufReader, BufRead}};

use super::Collector;

pub fn parse_rs_file(p: &PathBuf, collector: &mut Collector) {
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

    bfs_syntax_tree(&mut cursor, collector, text);
}

fn bfs_syntax_tree(cursor: &mut TreeCursor, collector: &mut Collector, src: &String) {
    // let mut layer = vec![];
    loop {
        let node = cursor.node();

        // print!("{} ", node.kind());
        // if node.kind() == "field_identifier" {
        //     print!("-{}", node.start_position());
        // }
        // if node.kind() == "function_item" {
        //     print!("-[{}]", node.parent().unwrap().kind());
        //     for i in 0..node.child_count() {
        //         if node.child(i).unwrap().kind() == "parameters" {
        //             for j in 0..node.child(i).unwrap().child_count() {
        //                 print!("-{}", node.child(i).unwrap().child(j).unwrap().kind());
        //             }
        //         }
        //         // print!("-{}", node.child(i).unwrap().kind());
        //     }
        // }

        match node.kind() {
            "struct" | "enum" => 
                collector.struct_def += 1,
            "union" | "union_item" => 
                (),
            "tuple_struct_pattern" | "struct_item" | "enum_item" | "enum_variant_list" | "enum_variant" | "struct_expression"
             | "tuple_pattern" =>
                (),
            "impl" | "impl_item" => 
                (),
            "trait" => 
                collector.trait_def += 1,
            "trait_item" | "trait_bounds" | "constrained_type_parameter" | "type_binding"
             | "removed_trait_bound" | "bounded_type" | "associated_type" | "higher_ranked_trait_bound" =>
                (),
            "self_parameter" => 
                (), // collector.method_def += 1,
            "reference_expression" => 
                collector.ref_op += 1,
            "ref" | "ref_pattern" =>
                (),
            "field_expression" => 
                collector.field_access += 1,
            "ordered_field_declaration_list" =>
                (),
            "match_expression" => 
                collector.match_block += 1,
            "function_item" => {
                // check if first parameter is self_parameter
                let mut is_method = false;
                for i in 0..node.child_count() {
                    if node.child(i).unwrap().kind() == "parameters" {
                        for j in 0..node.child(i).unwrap().child_count() {
                            if node.child(i).unwrap().child(j).unwrap().kind() == "self_parameter"{
                                is_method = true;
                                break;
                            }
                        }
                        break;
                    }
                }

                match is_method {
                    true => 
                        collector.method_def += 1,
                    false => 
                        collector.function_def += 1,
                }
                
            }
            "function_signature_item" | "function_type" | "function_modifiers" => 
                (),
            "generic_function" => 
                (),
            "call_expression" => {
                if node.child(0).unwrap().kind() == "field_expression" {
                    collector.method_call += 1;
                }
                else {
                    collector.function_call += 1;
                }
                // println!("{}\n{} {}", node.to_sexp(), node.start_position(), node.end_position());
            }
            "closure_expression" => 
                collector.closure_def += 1,
            "closure_parameters" | "move" =>
                (),
            "dynamic_type" | "type_cast_expression" => 
                collector.dyn_use += 1,
            "dyn" => 
                (),
            "macro_definition" =>
                collector.macro_def += 1,
            "macro_invocation" => 
                collector.macro_use += 1,
            "fragment_specifier" | "macro_rules!" | "macro_rule" | "$" | "tt" | "@" | "ty" => 
                (),
            "generic_type" =>
                collector.genrics += 1,
            "generic_type_with_turbofish" =>
                (),
            "source_file" => 
                (),
            "mod" | "use" | "use_declaration" | "use_list" | "scoped_use_list" | "mod_item" | "crate" 
             | "super" | ".." | "use_as_clause" | "as" | "use_wildcard" | "foreign_mod_item" => 
                (),
            "token_tree" | "token_tree_pattern" | "token_binding_pattern" | "token_repetition" | "token_repetition_pattern" => 
                (),
            "block" | "break" |  "break_expression" | "return_expression" | "return" => 
                (),
            "while_expression" | "loop_expression" | "for_expression" => 
                collector.loops += 1,
            "while" | "loop" | "continue" | "continue_expression"| "for"
             | "loop_label" =>
                (),
            "if_expression" | "if" | "else" | "else_clause" => 
                (),
            "expression_statement" | "empty_statement" =>
                (),
            "identifier" | "type_identifier" | "type_arguments" | "reference_type" 
            | "primitive_type" | "arguments" | "parameters" | "parameter" 
            | "field_identifier" | "scoped_identifier" | "field_initializer_list"
            | "scoped_type_identifier" | "mutable_specifier" | "ident"
            | "optional_type_parameter" | "const_parameter" | "variadic_parameter" => 
                (),
            "match_pattern" | "match_arm" | "match" | "match_block" =>
                (),
            "string_literal" | "integer_literal" | "raw_string_literal" | "boolean_literal"
             | "char_literal" | "float_literal" | "literal" | "negative_literal" => 
                (),
            "or_pattern" | "reference_pattern" | "mut_pattern"| "remaining_field_pattern"
             | "range_pattern" | "slice_pattern" | "captured_pattern"| "field_pattern"
             | "struct_pattern" => 
                (),
            "(" | ")" | "=>" | "."  | "{" | "}" | "&" | "|" | "\"" | "," | "*"
             | "!" | "_" | "::" | ";" | "=" | "+=" | "-=" | ":" | "&&" | "?"
             | "==" | "<" | ">" | "->" | "#" | "[" | "]" | "true" | "false"
             | "||" | "+" | "!=" | "..=" | "<<" | "'" | "-" | "<=" | ">="
             | "/" | "%" | "^" | "|=" | ">>" | "/=" | "*=" | "<<=" | "&="
             | ">>=" | "^=" | "%=" => 
                (),
            "fn" =>
                (),
            "attribute_item" | "attribute" | "inner_attribute_item" => 
                (),
            "let" | "let_declaration" | "shorthand_field_identifier"
            | "field_declaration" | "field_declaration_list"
             | "field_initializer" | "declaration_list" | "let_condition"
             | "shorthand_field_initializer" | "base_field_initializer" => 
                (),
            "escape_sequence" => 
                (),
            "unit_expression" | "unit_type" | "unary_expression" | "compound_assignment_expr" | "binary_expression"
             | "tuple_expression" | "array_expression" | "range_expression" | "array_type" | "assignment_expression"
             | "expr" | "tuple_type" | "index_expression" | "type_item" | "type" | "abstract_type" | "type_parameters"
             | "parenthesized_expression" | "pointer_type" | "bracketed_type" | "empty_type" | "meta" | "qualified_type" => 
                (),
            "try_expression" => 
                (),
            "in" => 
                (),
            "visibility_modifier" | "pub" | "vis" =>
                (),
            "self" => 
                (),
            "line_comment" => 
                collector.lines -= 1,
            "block_comment" =>
                (),
            "static" | "static_item" =>
                (),
            "const" | "const_item" | "const_block" =>
                (),
            "extern" | "extern_crate_declaration" | "extern_modifier" =>
                (),
            "metavariable" => 
                (),
            "lifetime" | "for_lifetimes" => 
                (),
            "default" => 
                (),
            "unsafe" | "unsafe_block" =>
                (),
            "where" | "where_clause" | "where_predicate" =>
                (),
            "ERROR" =>
                (),
            "..." => 
                (),
            "await" | "async" | "await_expression" | "async_block" =>
                (),
            "path" =>
                (),
            "pat" =>
                (),
            "item" | "stmt" => 
                (),
            _ => 
                println!("{}", node.kind()),
        };
        
        // layer.push(cursor.clone());


        if !cursor.goto_next_sibling() {   
            // println!();  
            if cursor.goto_parent() {
                cursor.goto_first_child();
            }

            if cursor.goto_first_child() {
                bfs_syntax_tree(cursor, collector, src);
                cursor.goto_parent();
            }

            while cursor.goto_next_sibling() {            
                if cursor.goto_first_child() {
                    bfs_syntax_tree(cursor, collector, src);
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