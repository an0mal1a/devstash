use crate::constants::Snippet;
use crate::json_core;

use std::{path::Path};
use std::fs;

pub fn import_snippets(args: &[String], snippets: &mut Vec<Snippet>) -> Result<(), String> {
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