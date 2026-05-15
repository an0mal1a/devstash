use serde::{Deserialize, Serialize};

pub const PATH: &str = "snippets.json";
pub const RESET: &str = "\x1b[0m";
pub const DIM: &str = "\x1b[2m";
pub const BOLD: &str = "\x1b[1m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const RED: &str = "\x1b[31m";
pub const CYAN: &str = "\x1b[36m";
pub const VERSION: &str = "v0.2";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Snippet {
    pub id: u64,
    pub title: String,
    pub tags: Vec<String>,
    pub content: String,
    pub kind: SnippetKind,
    pub created_at: String,
}

#[derive(Clone, Debug)]
pub struct SnippetQuestion {
    pub title: String,
    pub qtype: String,
    pub buf: Buffer,
    pub answered: bool,
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