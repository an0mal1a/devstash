use crate::constants::PATH;
use crate::json_core;
use crate::utils;


use std::{path::Path};
use std::fs::copy;

pub fn restore_snippets(args: &[String]) -> Result<(), String> {
    let import_path = args.get(2).ok_or("No import specified")?;

    if !Path::new(import_path).exists() {
        return Err("Import file does not exist".to_string());
    }

    // Check if the file is compatible.
    json_core::compatibility_check(import_path.to_string())?;

    match copy(import_path, PATH) {
        Ok(_) => {
            utils::print_success("File imported successfully");
            Ok(())
        }
        Err(e) => Err(format!("Failed to import file: {}", e)),
    }
}