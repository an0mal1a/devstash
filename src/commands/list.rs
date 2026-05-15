use crate::utils;
use crate::constants::Snippet;

pub fn list_snippets(args: &[String], snippets: &[Snippet]) {
    let limit: usize = match args.get(2) {
        Some(v) => v.parse().unwrap_or(10),
        None => 10,
    };

    let limit = limit.min(snippets.len());
    let slice = &snippets[0..limit];
    let rows: Vec<&Snippet> = slice.iter().collect();
    utils::print_snippet_table("DevStash", &rows);
}