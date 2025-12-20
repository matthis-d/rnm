use argh::FromArgs;
use log::{info, warn};
use regex::Regex;
use std::fs;

mod utils;

/// Replaces filenames using regexes
#[derive(FromArgs)]
pub struct CliArguments {
    /// regex of filenames to replace
    #[argh(positional)]
    from: String,

    /// replacement to apply with $1, $2, etc. as values catched in regex.
    // IMPROVEMENT: Fix typo - "catched" should be "caught"
    // IMPROVEMENT: Consider using PathBuf instead of String for better type safety
    #[argh(positional)]
    to: String,
}

pub fn rename_files(path: &str, args: &CliArguments) -> std::io::Result<()> {
    info!("from {} into {}", args.from, args.to);

    // Compile regex once before processing files for better performance
    let re = match Regex::new(&args.from) {
        Ok(regex) => regex,
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid regex pattern '{}': {}", args.from, e),
            ));
        }
    };

    // Handle error from get_file_entries (e.g., path doesn't exist)
    let entries = match utils::files::get_file_entries(path) {
        Ok(entries) => entries,
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to read directory '{}': {}", path, e),
            ));
        }
    };

    // IMPROVEMENT: get_file_entries returns Vec which loads all entries into memory
    // Consider returning an iterator instead for better memory efficiency
    for entry in entries {
        let entry_path = entry.path();

        // Work with Path directly - handles both UTF-8 and non-UTF8 paths
        // Use lossy conversion for display/logging purposes
        match utils::string::replace_name(&entry_path, &re, &args.to) {
            Ok(output_path) => {
                // Compare paths directly instead of strings
                if entry_path != output_path {
                    // IMPROVEMENT: Consider checking if output file already exists before renaming
                    // Handle rename errors gracefully - log and continue
                    if let Err(e) = fs::rename(&entry_path, &output_path) {
                        let entry_display = entry_path.to_string_lossy();
                        let output_display = output_path.to_string_lossy();
                        warn!(
                            "Failed to rename '{}' to '{}': {}",
                            entry_display, output_display, e
                        );
                    }
                }
            }
            Err(e) => {
                let entry_display = entry_path.to_string_lossy();
                warn!("Failed to process file '{}': {}", entry_display, e);
            }
        }
    }
    Ok(())
}
