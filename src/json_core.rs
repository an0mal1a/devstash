use std::path::Path;
use std::fs;

use crate::Snippet;
use crate::utils::get_last_id;

// Compatibility of files
pub fn compatibility_check(import_path: String) -> Result<(), String>{
    // file is json
    let content = fs::read_to_string(import_path).unwrap();
    
    // file has correct structure
    match serde_json::from_str::<Vec<Snippet>>(&content) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("JSON inválido o estructura incorrecta");
            println!("{}", e);
            return  Err(format!("Invalid structure: {}", e));
        }
    }
}

pub fn merge_jsons(snippets_to_import: Vec<Snippet>, current_snippets: &mut Vec<Snippet>) {
    let mut next_id = get_last_id(&current_snippets) + 1;
    let mut imported = 0;

    for mut snippet in snippets_to_import {
        snippet.id = next_id;
        next_id += 1;

        let already_exists = current_snippets.iter().any(|s| {
            s.title == snippet.title 
                && 
            s.content == snippet.content
        });

        if already_exists { continue; }

        imported += 1;
        current_snippets.push(snippet);
    }

    println!("DevStash imported: {}", imported)
}

pub fn parse(path: String) -> Result<Vec<Snippet>, Box<dyn std::error::Error>> {
    if !Path::new(&path).exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(path)?;
    let snippets: Vec<Snippet> = serde_json::from_str(&content)?;

    Ok(snippets)
}

pub fn save(path: &str, snippets: &Vec<Snippet>) -> Result<(), Box<dyn std::error::Error>> {
    let content_string = serde_json::to_string_pretty(snippets).unwrap();
    fs::write(path, content_string).unwrap();
    Ok(())
}
