use crate::constants::PATH;
use crate::utils;

use std::fs::{copy, create_dir_all};
use std::{path::Path};

pub fn export_snippets(args: &[String]) -> Result<(), &str> {
    // Get file path
    let export_path = args.get(2).ok_or("No export path specified")?;

    // Check if file path exists
    let export_path = Path::new(&export_path);
    if let Some(parent) = export_path.parent() {
        if !parent.exists() {
            utils::print_warning(&format!("Creating parent path: {}", parent.display()));

            match create_dir_all(parent) {
                Ok(_) => utils::print_success("Parent path created"),
                Err(e) => utils::print_error(&format!("Parent path was not created: {}", e)),
            }
        }
    }

    match copy(PATH, export_path) {
        Ok(_) => {
            utils::print_success("File exported successfully");
            Ok(())
        }
        Err(e) => {
            utils::print_error(&format!("File was not exported: {}", e));
            Err("File error")
        }
    }
}