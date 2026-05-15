use crate::constants::Snippet;
use crate::utils;

use crate::constants::YELLOW;
use crate::constants::RESET; 
use crate::constants::BOLD;

use arboard::Clipboard;

pub fn copy_snippet(args: &[String], snippets: &[Snippet]) -> Result<(), String> {
    let id = args.get(2).ok_or("No ID specified")?;
    let id: u64 = match id.parse() {
        Ok(id) => id,
        Err(e) => {
            utils::print_error(&e.to_string());
            return Err(e.to_string())
        }
    }; 

    let s: Snippet = utils::get_snippet_by_id(id, snippets)?.clone();
    let mut c = Clipboard::new().unwrap();
    
    match c.set_text(s.content) {
        Ok(_) => {
            println!("Snippet {}{}{} copied to {}clipboard{}", YELLOW, id, RESET, BOLD, RESET);
            Ok(())
        },
        Err(e) => {
            utils::print_error(&e.to_string());
            Err(e.to_string())
        }
    }
    
}