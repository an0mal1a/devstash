use crate::constants::Snippet;
use crate::utils;


pub fn delete_snippet(args: &Vec<String>, snippets: &mut Vec<Snippet>) {
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
