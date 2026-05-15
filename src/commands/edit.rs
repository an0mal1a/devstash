use crate::constants::{
    SnippetKind,
    Snippet
};
use crate::utils;


pub fn edit_snippet(args: &[String], snippets: &mut Vec<Snippet>) -> Result<(), String>  {
    let id_to_edit = args.get(2).ok_or("No ID specified")?;
    let id: u64 = match id_to_edit.parse() {
        Ok(id) => id,
        Err(e) => {
            utils::print_error(&e.to_string());
            return Err(e.to_string())
        }
    }; 
    
    let s = match snippets.iter_mut().find(|s| s.id == id){
        Some(s) => s,
        None => {
            utils::print_error("No snippet found with that ID");
            return Err("no snippet found".to_string());
        }
    };

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