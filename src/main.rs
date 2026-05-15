// Internal dependencies
pub mod help_panel;
pub mod json_core;
pub mod utils;

use std::fs::{self, copy, create_dir_all};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashSet;
use std::io::{self, Write};
use std::{env, path::Path};

use serde::{Deserialize, Serialize};
use arboard::Clipboard;

const PATH: &str = "snippets.json";
pub const RESET: &str = "\x1b[0m";
pub const DIM: &str = "\x1b[2m";
pub const BOLD: &str = "\x1b[1m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const RED: &str = "\x1b[31m";
pub const CYAN: &str = "\x1b[36m";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Snippet {
    id: u64,
    title: String,
    tags: Vec<String>,
    content: String,
    kind: SnippetKind,
    created_at: i64,
}

#[derive(Clone, Debug)]
pub struct SnippetQuestion {
    title: String,
    qtype: String,
    buf: Buffer,
    answered: bool,
}

#[derive(Clone, Debug)]
pub enum Buffer {
    Text(String),
    Kind(SnippetKind),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SnippetKind {
    Command,
    Json,
    Note,
}

impl std::fmt::Display for SnippetKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            SnippetKind::Command => write!(f, "Command"),
            SnippetKind::Json => write!(f, "Json"),
            SnippetKind::Note => write!(f, "Note")
        }
    }
}

fn add_snippet(args: &Vec<String>, snippets: &mut Vec<Snippet>) {
    let to_add = match args.get(2) {
        Some(v) => v.parse().unwrap_or(1),
        None => 1,
    };

    for _ in 0..to_add {
        let mut questions: Vec<SnippetQuestion> = vec![
            SnippetQuestion {
                title: format!("{}Title{}   > ", BOLD, RESET),
                qtype: "input".to_string(),
                buf: Buffer::Text("".to_string()),
                answered: false,
            },
            SnippetQuestion {
                title: format!("{}Content{} > ", BOLD, RESET),
                qtype: "input".to_string(),
                buf: Buffer::Text("".to_string()),
                answered: false,
            },
            SnippetQuestion {
                title: format!("{}Tags{}    > ", BOLD, RESET),
                qtype: "input".to_string(),
                buf: Buffer::Text("".to_string()),
                answered: false,
            },
            SnippetQuestion {
                title: format!("{}Type{}    > ", BOLD, RESET),
                qtype: "enum".to_string(),
                buf: Buffer::Kind(SnippetKind::Note),
                answered: false,
            },
        ];

        while !utils::are_all_answered(&questions) {
            for q in questions.iter_mut().filter(|q| !q.answered) {
                print!("{}", q.title);
                io::stdout().flush().unwrap();

                let mut answ = String::new();
                io::stdin().read_line(&mut answ).unwrap();
                let answ = answ.trim();

                if q.qtype == "input" {
                    q.buf = Buffer::Text(answ.to_string());
                } else {
                    q.buf = match answ {
                        "command" => Buffer::Kind(SnippetKind::Command),
                        "json" => Buffer::Kind(SnippetKind::Json),
                        _ => Buffer::Kind(SnippetKind::Note),
                    };
                }
                q.answered = true;
            }
        }

        let last_id = utils::get_last_id(&snippets);
        snippets.push(Snippet {
            id: last_id + 1,
            title: utils::extract_text(&questions[0].buf),
            content: utils::extract_text(&questions[1].buf),
            tags: utils::extract_text(&questions[2].buf)
                .split(',')
                .map(|tag| tag.trim().to_string())
                .filter(|tag| !tag.is_empty())
                .collect(),
            kind: utils::extract_kind(&questions[3].buf),
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        });
        utils::print_success(&format!("Snippet #{} saved", last_id + 1));
    }
}

fn edit_snippet(args: &[String], snippets: &mut Vec<Snippet>) -> Result<(), String>  {
    let id_to_edit = args.get(2).ok_or("No ID specified")?;
    let id: u64 = match id_to_edit.parse() {
        Ok(id) => id,
        Err(e) => {
            utils::print_error(&e.to_string());
            return Err(e.to_string())
        }
    }; 
    
    let s = snippets.iter_mut().find(|s| s.id == id).unwrap();

    if let Some(title) = utils::ask_optional("Title", &s.title)? {
        s.title = title
    }
    
    if let Some(content) = utils::ask_optional("Content", &s.content)? {
        s.content = content
    }

    if let Some(tags) = utils::ask_optional("Tags", &s.tags.join(","))? {
        s.tags = tags.split(',')
            .map(|tag| tag.trim().to_string())
            .filter(|tag| !tag.is_empty())
            .collect() 
    }

    if let Some(kind) = utils::ask_optional("Kind", &s.kind.to_string())? {
        s.kind = match kind.as_str() {
            "command" => SnippetKind::Command,
            "json" => SnippetKind::Json,
            _ => SnippetKind::Note
        };
    }

    Ok(())
}

fn list_snippets(args: &[String], snippets: &[Snippet]) {
    let limit: usize = match args.get(2) {
        Some(v) => v.parse().unwrap_or(10),
        None => 10,
    };

    let limit = limit.min(snippets.len());
    let slice = &snippets[0..limit];
    let rows: Vec<&Snippet> = slice.iter().collect();
    utils::print_snippet_table("DevStash", &rows);
}

fn show_snippet(args: &Vec<String>, snippets: &[Snippet]) {
    let id: u64 = match args.get(2) {
        Some(v) => v.parse().unwrap_or(1),
        None => 1,
    };

    match utils::get_snippet_by_id(id, snippets) {
        Ok(snippet) => utils::print_snippet_detail(snippet),
        Err(e) => utils::print_error(&e),
    }
}

fn delete_snippet(args: &Vec<String>, snippets: &mut Vec<Snippet>) {
    let id: u64 = match args.get(2) {
        Some(v) => v.parse().unwrap(),
        None => {
            return;
        }
    };

    match utils::delete_snippet_by_id(id, snippets) {
        Ok(_) => utils::print_success(&format!("Snippet #{} removed", id)),
        Err(e) => utils::print_error(&e),
    }
}

fn search_snippets(args: &Vec<String>, snippets: &[Snippet]) -> Result<(), String> {
    let slice = args
        .get(2..)
        .filter(|s| !s.is_empty())
        .ok_or("No tags specified")?;

    let search_query = slice.join(" ").to_ascii_lowercase();
    let r: Vec<&Snippet> = snippets
        .iter()
        .filter(|s| {
            s.content.to_ascii_lowercase().contains(&search_query)
                || s.title.to_ascii_lowercase().contains(&search_query)
        })
        .collect();

    utils::print_snippet_table(&format!("Search: {}", search_query), &r);
    Ok(())
}

fn search_by_tag(args: &Vec<String>, snippets: &[Snippet]) -> Result<(), String> {
    // Extract the search_query
    let tags_to_find = args
        .get(2..)
        .filter(|t| !t.is_empty())
        .ok_or("No tags specified")?;

    let tag_set: HashSet<_> = tags_to_find.iter().collect();

    // Compare 2 slices
    let matches: Vec<&Snippet> = snippets
        .iter()
        .filter(|snippet| snippet.tags.iter().any(|tag| tag_set.contains(tag)))
        .collect();

    utils::print_snippet_table(&format!("Tags: {}", tags_to_find.join(", ")), &matches);
    Ok(())
}

fn copy_snippet(args: &[String], snippets: &[Snippet]) -> Result<(), String> {
    let id = args.get(2).ok_or("No ID specified")?;
    let id: u64 = match id.parse() {
        Ok(id) => id,
        Err(e) => {
            utils::print_error(&e.to_string());
            return Err(e.to_string())
        }
    }; 

    let s: Snippet = utils::get_snippet_by_id(id, snippets)?.clone();
    let mut c = Clipboard::new().unwrap();
    
    match c.set_text(s.content) {
        Ok(v) => {
            println!("Snippet {}{}{} copied to {}clipboard{}", YELLOW, id, RESET, BOLD, RESET);
            Ok(())
        },
        Err(e) => {
            utils::print_error(&e.to_string());
            Err(e.to_string())
        }
    }
    
}

fn export_snippets(args: &[String]) -> Result<(), &str> {
    // Get file path
    let export_path = args.get(2).ok_or("No export path specified")?;

    // Check if file path exists
    let export_path = Path::new(&export_path);
    if let Some(parent) = export_path.parent() {
        if !parent.exists() {
            utils::print_warning(&format!("Creating parent path: {}", parent.display()));

            match create_dir_all(parent) {
                Ok(_) => utils::print_success("Parent path created"),
                Err(e) => utils::print_error(&format!("Parent path was not created: {}", e)),
            }
        }
    }

    match copy(PATH, export_path) {
        Ok(_) => {
            utils::print_success("File exported successfully");
            Ok(())
        }
        Err(e) => {
            utils::print_error(&format!("File was not exported: {}", e));
            Err("File error")
        }
    }
}

fn import_snippets(args: &[String], snippets: &mut Vec<Snippet>) -> Result<(), String> {
    let import_path = args.get(2).ok_or("No import specified".to_string())?;

    if !Path::new(import_path).exists() {
        return Err("Import file does not exist".to_string());
    }

    // Check if the file is compatible
    json_core::compatibility_check(import_path.to_string())?;

    let content = fs::read_to_string(import_path).unwrap();
    let snippets_to_import: Vec<Snippet> = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    
    Ok(json_core::merge_jsons(snippets_to_import, snippets))
}

fn restore_snippets(args: &[String]) -> Result<(), String> {
    let import_path = args.get(2).ok_or("No import specified")?;

    if !Path::new(import_path).exists() {
        return Err("Import file does not exist".to_string());
    }

    // Check if the file is compatible.
    json_core::compatibility_check(import_path.to_string())?;

    match copy(import_path, PATH) {
        Ok(_) => {
            utils::print_success("File imported successfully");
            Ok(())
        }
        Err(e) => Err(format!("Failed to import file: {}", e)),
    }
}


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
        "add" => { add_snippet(&args, &mut snippets); } 
        "edit" => { let _ = edit_snippet(&args, &mut snippets); should_save = true; } 
        "help" => { return help_panel::main(); } 
        "list" => { list_snippets(&args, &snippets); } 
        "show" => { show_snippet(&args, &snippets); } 
        "delete" => { delete_snippet(&args, &mut snippets); should_save = true; } 
        "copy" => { copy_snippet(&args, &snippets);  } 
        "export" => { let _ = export_snippets(&args); }
        "search" => { 
            if let Err(e) = search_snippets(&args, &snippets) {
                utils::print_error(&e);
            } 
        },
        "tag" => { 
            if let Err(e) = search_by_tag(&args, &snippets) {
                utils::print_error(&e);
            } 
        }
        "restore" => {
            if let Err(e) = restore_snippets(&args) {
                utils::print_error(&e);
            }
            should_save = true;
        }
        "import" => {
            if let Err(e) = import_snippets(&args, &mut snippets) {
                utils::print_error(&e);
            }
            should_save = true;
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
