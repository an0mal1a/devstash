// Internal dependencies
pub mod help_panel;
pub mod json_core;
pub mod utils;

use core::error;
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use std::io::{self, Write}; 
use std::{env};


const PATH: &str = "snippets.json";

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
    answered: bool
}

#[derive(Clone, Debug)]
pub enum Buffer {
    Text(String),
    Kind(SnippetKind)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SnippetKind {
    Command,
    Json,
    Note,
}

fn add_snippet(args: &Vec<String>, snippets: &mut Vec<Snippet>) {
    let to_add = match args.get(2) { 
        Some(v) => v.parse().unwrap_or(1),
        None => 1
        
    };

    for _ in 0..to_add {
        let mut questions: Vec<SnippetQuestion> = vec![
            SnippetQuestion{
                title: "Título del snippet -> ".to_string(),
                qtype: "input".to_string(),
                buf: Buffer::Text("".to_string()),
                answered: false
            },
            SnippetQuestion{
                title: "Contenido del snippet -> ".to_string(),
                qtype: "input".to_string(),
                buf: Buffer::Text("".to_string()),
                answered: false
            },
            SnippetQuestion{
                title: "Tags de snippet -> ".to_string(),
                qtype: "input".to_string(),
                buf: Buffer::Text("".to_string()),
                answered: false
            },
            SnippetQuestion{
                title: "Tipo del snippet -> ".to_string(),
                qtype: "enum".to_string(),
                buf: Buffer::Kind(SnippetKind::Note),
                answered: false
            },
        ];

        while !utils::are_all_answered(&questions){

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
                        _ => Buffer::Kind(SnippetKind::Note)
                    };
                }
                q.answered = true;
            }
        };

        let last_id = utils::get_last_id(&snippets);
        snippets.push(
            Snippet {
                id: last_id + 1,
                title: utils::extract_text(&questions[0].buf),
                content: utils::extract_text(&questions[1].buf),
                tags: utils::extract_text(&questions[2].buf).split(',').map(|tag| tag.trim().to_string()).filter(|tag| !tag.is_empty()).collect(),
                kind: utils::extract_kind(&questions[3].buf),
                created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
            }
        );
    }
        
}


fn list_snippets(args: &[String], snippets: &[Snippet]) {
    let limit: usize = match args.get(2) {
        Some(v) => v.parse().unwrap_or(10),
        None => 10,
    };

    let limit = limit.min(snippets.len());
    let slice = &snippets[0..limit];
    println!("{:#?}", slice);
}


fn show_snippet(args: &Vec<String>, snippets: &[Snippet]){
    let id: u64 = match args.get(2) {
        Some(v) => v.parse().unwrap_or(1),
        None => 1,
    };

    match utils::get_snippet_by_id(id, snippets) {
        Ok(snippet) => println!("{:#?}", snippet),
        Err(e) => println!("{}", e),
    }

}

fn delete_snippet(args: &Vec<String>, snippets: &mut Vec<Snippet>)  {
    let id: u64 = match args.get(2) {
        Some(v) => v.parse().unwrap(),
        None => { return; }
    };

    match utils::delete_snippet_by_id(id, snippets) {
        Ok(_) => println!("The ID {} has been removed", id),
        Err(e) => println!("{}", e)
    }
}

fn search_snippets(args: &Vec<String>, snippets: &[Snippet]) {
    let slice = match args.get(2..){
        Some(slice) if !slice.is_empty() => slice,
        _ => return
    };

    let search_query = slice.join(" ").to_ascii_lowercase();
    let r: Vec<&Snippet> = snippets.iter().filter(|s| s.content.to_ascii_lowercase().contains(&search_query) || s.title.to_ascii_lowercase().contains(&search_query)).collect();

    println!("{:#?}", r )
}


fn search_by_tag(args: &Vec<String>, snippets: &[Snippet]){
    // Extract the search_query
    let tags_to_find = match args.get(2..) {
        Some(slice) if !slice.is_empty() => slice,
        _ => return
    };

    let tag_set: HashSet<_> = tags_to_find.iter().collect();

    // Compare 2 slices
    let matches: Vec<&Snippet> = snippets
        .iter()
        .filter(|snippet| {
            snippet.tags.iter().any(|tag| tag_set.contains(tag))
        }).collect();

    println!("{:#?}", matches)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { help_panel::main(); return }

    let mut snippets: Vec<Snippet> = match json_core::parse(PATH.to_string()) {
        Ok(sn) => sn,
        Err(e) => { println!("Error: {}", e); return; }
    }; 
   
    let action: &String = &args[1];
    if action == "add" { add_snippet(&args, &mut snippets); }
    else if action == "list" { list_snippets(&args, &snippets) }
    else if action == "show" { show_snippet(&args, &snippets); }
    else if action == "delete" { delete_snippet(&args, &mut snippets); }
    else if action == "search" { search_snippets(&args, &snippets)  }
    else if action == "tag" { search_by_tag(&args, &snippets); }
    else if action == "export" { unimplemented!(); }
    else if action == "import" { unimplemented!(); }



    let saved = match json_core::save(PATH, &snippets) {
        Ok(sn) => sn,
        Err(e) => { println!("Error: {}", e); return; }
    }; 

    return
}

