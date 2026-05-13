use std::fs;
use std::path::Path;

use crate::Snippet;

pub fn parse(path: String) -> Result<Vec<Snippet>, Box<dyn std::error::Error>> {
    if Path::new(&path).exists() {
        let content = fs::read_to_string(path)?;
        let snippets: Vec<Snippet> = match serde_json::from_str(&content) {
            Ok(v) => v,
            Err(_) => Vec::new(),
        };
        Ok(snippets)
    } else {
        Ok(Vec::new())
    }
}

pub fn save(path: &str, snippets: &Vec<Snippet>) -> Result<(), Box<dyn std::error::Error>> {
    let content_string = serde_json::to_string_pretty(snippets).unwrap();
    fs::write(path, content_string).unwrap();
    Ok(())
}
