
use std::io::{self, Write};

use crate::constants::{
    DIM,
    RED,
    BOLD,
    CYAN,
    GREEN,
    RESET,
    YELLOW,

    Buffer,
    Snippet,
    SnippetKind,
    SnippetQuestion,
};

// General
pub fn are_all_answered(arr: &Vec<SnippetQuestion>) -> bool {
    for q in arr {
        if !q.answered {
            return false;
        }
    }
    return true;
}

pub fn extract_text(buf: &Buffer) -> String {
    match buf {
        Buffer::Text(v) => v.clone(),
        _ => panic!("Expected text"),
    }
}

pub fn extract_kind(buf: &Buffer) -> SnippetKind {
    match buf {
        Buffer::Kind(v) => v.clone(),
        _ => panic!("Expected kind"),
    }
}

pub fn get_last_id(snippets: &[Snippet]) -> u64 {
    snippets.iter().map(|s| s.id).max().unwrap_or(0)
}

pub fn get_snippet_by_id(id: u64, snippets: &[Snippet]) -> Result<&Snippet, String> {
    snippets
        .iter()
        .find(|s| s.id == id)
        .ok_or_else(|| format!("Snippet with id {} not found", id))
}

pub fn delete_snippet_by_id(id: u64, snippets: &mut Vec<Snippet>) -> Result<(), String> {
    // Check if id is on json snippets and get the index
    let index = snippets
        .iter()
        .position(|s| s.id == id)
        .ok_or_else(|| format!("Snippet with id {} not found", id))?; // Al usar "?" pasamos el error y en caso de haberlo, se retorna solo Err

    snippets.remove(index);
    Ok(())
}

pub fn ask_optional(label: &str, current: &str) -> Result<Option<String>, String> {
    print!("{}{}{} [{}] > ", BOLD, label, RESET, current);
    io::stdout().flush().map_err(|e| e.to_string())?;

    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .map_err(|e| e.to_string())?;

    let answer = answer.trim();

    if answer.is_empty() {
        Ok(None)
    } else {
        Ok(Some(answer.to_string()))
    }
}

// Printing utils
pub fn kind_label(kind: &SnippetKind) -> &'static str {
    match kind {
        SnippetKind::Command => "command",
        SnippetKind::Json => "json",
        SnippetKind::Note => "note",
    }
}

pub fn truncate(value: &str, max_len: usize) -> String {
    let chars: Vec<char> = value.chars().collect();
    if chars.len() <= max_len {
        return value.to_string();
    }

    let mut shortened: String = chars.into_iter().take(max_len.saturating_sub(1)).collect();
    shortened.push('…');
    shortened
}

pub fn tags_label(tags: &[String]) -> String {
    if tags.is_empty() {
        format!("{}none{}", DIM, RESET)
    } else {
        tags.join(", ")
    }
}

pub fn print_success(message: &str) {
    println!("{}ok{} {}", GREEN, RESET, message);
}

pub fn print_warning(message: &str) {
    println!("{}warn{} {}", YELLOW, RESET, message);
}

pub fn print_error(message: &str) {
    println!("{}error{} {}", RED, RESET, message);
}

pub fn print_snippet_table(title: &str, snippets: &[&Snippet]) {
    println!(
        "{}{}{} {}{}{} snippet(s)",
        BOLD,
        title,
        RESET,
        DIM,
        snippets.len(),
        RESET
    );

    if snippets.is_empty() {
        println!("{}No snippets found.{}", DIM, RESET);
        return;
    }

    println!(
        "{}{:>4}  {:<10}  {:<30}  {:<28}  {}{}",
        DIM, "ID", "TYPE", "TITLE", "TAGS", "PREVIEW", RESET
    );
    println!("{}", "-".repeat(96));

    for snippet in snippets {
        let preview = snippet.content.replace('\n', " ");
        println!(
            "{:>4}  {:<10}  {:<30}  {:<28}  {}",
            snippet.id,
            kind_label(&snippet.kind),
            truncate(&snippet.title, 30),
            truncate(&tags_label(&snippet.tags), 28),
            truncate(&preview, 20)
        );
    }
}

pub fn print_snippet_detail(snippet: &Snippet) {
    println!("{}#{}{}  {}", CYAN, snippet.id, RESET, snippet.title);
    println!(
        "{}type:{} {}  {}tags:{} {}",
        DIM,
        RESET,
        kind_label(&snippet.kind),
        DIM,
        RESET,
        tags_label(&snippet.tags)
    );
    println!("{}created:{} {}", DIM, RESET, snippet.created_at);
    println!();
    println!("{}content{}", BOLD, RESET);
    println!("{}", "-".repeat(72));
    println!("{}", snippet.content);
}