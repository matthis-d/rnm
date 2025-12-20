use argh::FromArgs;
use log::info;
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
    use log::warn;

    info!("from {} into {}", args.from, args.to);

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
        // IMPROVEMENT: Using to_str() loses information about non-UTF8 paths
        // Consider using Path/PathBuf operations directly instead
        if let Some(pathname) = entry.path().to_str() {
            // IMPROVEMENT: Regex is compiled on every call to replace_name
            // Should compile once and reuse (see string.rs comments)
            match utils::string::replace_name(&pathname, &args.from, &args.to) {
                Ok(output) => {
                    if output != pathname {
                        // IMPROVEMENT: Consider checking if output file already exists before renaming
                        // Handle rename errors gracefully - log and continue
                        if let Err(e) = fs::rename(pathname, &output) {
                            warn!("Failed to rename '{}' to '{}': {}", pathname, output, e);
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to process file '{}': {}", pathname, e);
                }
            }
        } else {
            // IMPROVEMENT: Files with non-UTF8 paths are silently skipped
            // Log a warning for non-UTF8 paths
            warn!("Skipping file with non-UTF8 path: {:?}", entry.path());
        }
    }
    Ok(())
}
