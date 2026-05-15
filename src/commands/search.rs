use crate::constants::Snippet;
use crate::utils;

use std::collections::HashSet;

pub fn search_snippets(args: &Vec<String>, snippets: &[Snippet]) -> Result<(), String> {
    let slice = args
        .get(2..)
        .filter(|s| !s.is_empty())
        .ok_or("No tags specified")?;

    let search_query = slice.join(" ").to_ascii_lowercase();
    let r: Vec<&Snippet> = snippets
        .iter()
        .filter(|s| {
            s.content.to_ascii_lowercase().contains(&search_query)
                || s.title.to_ascii_lowercase().contains(&search_query)
        })
        .collect();

    utils::print_snippet_table(&format!("Search: {}", search_query), &r);
    Ok(())
}


pub fn search_by_tag(args: &Vec<String>, snippets: &[Snippet]) -> Result<(), String> {
    // Extract the search_query
    let tags_to_find = args
        .get(2..)
        .filter(|t| !t.is_empty())
        .ok_or("No tags specified")?;

    let tag_set: HashSet<_> = tags_to_find.iter().collect();

    // Compare 2 slices
    let matches: Vec<&Snippet> = snippets
        .iter()
        .filter(|snippet| snippet.tags.iter().any(|tag| tag_set.contains(tag)))
        .collect();

    utils::print_snippet_table(&format!("Tags: {}", tags_to_find.join(", ")), &matches);
    Ok(())
}