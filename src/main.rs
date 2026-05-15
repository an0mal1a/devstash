// Internal dependencies
pub mod help_panel;
pub mod json_core;
pub mod constants;
pub mod utils;

// Commands/Actions
mod commands;
use commands::add;
use commands::edit;
use commands::list;
use commands::show;
use commands::copy;
use commands::search;
use commands::import;
use commands::export;
use commands::delete;
use commands::restore;
use std::env;

use constants::{
    PATH,
    Snippet,    
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { return help_panel::main(); }

    let mut snippets: Vec<Snippet> = match json_core::parse(PATH.to_string()) {
        Ok(sn) => sn,
        Err(e) => {
            utils::print_error(&format!("{}", e));
            return;
        }
    };
    let mut should_save = false;

    let action: &String = &args[1];
    match action.as_str() {
        "add" => { add::add_snippet(&args, &mut snippets); } 
        "edit" => { 
            if let Err(e) = edit::edit_snippet(&args, &mut snippets) { 
                utils::print_error(&e);
            } else {
                should_save = true;
            }
        } 
        "help" => { return help_panel::main(); } 
        "list" => { list::list_snippets(&args, &snippets); } 
        "show" => { show::show_snippet(&args, &snippets); } 
        "delete" => { delete::delete_snippet(&args, &mut snippets); should_save = true; } 
        "copy" => { let _ = copy::copy_snippet(&args, &snippets);  } 
        "export" => { let _ = export::export_snippets(&args); }
        "search" => { 
            if let Err(e) = search::search_snippets(&args, &snippets) {
                utils::print_error(&e);
            } 
        },
        "tag" => { 
            if let Err(e) = search::search_by_tag(&args, &snippets) {
                utils::print_error(&e);
            } 
        }
        "restore" => {
            if let Err(e) = restore::restore_snippets(&args) {
                utils::print_error(&e);
            }
        }
        "import" => {
            if let Err(e) = import::import_snippets(&args, &mut snippets) {
                utils::print_error(&e);
            } else {
                should_save = true;
            }
        }
        _ => {
            utils::print_error("Unknown command");
            help_panel::main();
        }
    }

    if should_save {
        match json_core::save(PATH, &snippets) {
            Ok(sn) => sn,
            Err(e) => {
                utils::print_error(&format!("{}", e));
                return;
            }
        };
    }

    return;
}
