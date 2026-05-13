use std::fs;
use std::path::Path;

use crate::Snippet;

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
