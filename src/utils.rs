use crate::SnippetQuestion;
use crate::SnippetKind;
use crate::Snippet;
use crate::Buffer;

pub fn are_all_answered(arr: &Vec<SnippetQuestion>) -> bool {
    for q in arr {
        if !q.answered {
            return false
        } 
    }
    return true
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
    snippets
        .iter()
        .map(|s| s.id)
        .max()
        .unwrap_or(0)
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