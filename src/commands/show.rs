use crate::constants::Snippet;
use crate::utils;

pub fn show_snippet(args: &Vec<String>, snippets: &[Snippet]) {
    let id: u64 = match args.get(2) {
        Some(v) => v.parse().unwrap_or(1),
        None => 1,
    };

    match utils::get_snippet_by_id(id, snippets) {
        Ok(snippet) => utils::print_snippet_detail(snippet),
        Err(e) => utils::print_error(&e),
    }
}