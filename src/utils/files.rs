use std::fs;

// IMPROVEMENT: Consider returning an iterator instead of Vec for better memory efficiency
// IMPROVEMENT: Consider accepting &Path or PathBuf instead of &str for better type safety
pub fn get_file_entries(pathname: &str) -> std::io::Result<Vec<fs::DirEntry>> {
    let entries: Vec<fs::DirEntry> = fs::read_dir(pathname)?
        // Keep readable entries
        .filter_map(Result::ok)
        // Keep entries with a readable file type
        .filter_map(|entry| {
            // Handle file_type() error gracefully - skip entries we can't read
            match entry.file_type() {
                Ok(file_type) if file_type.is_file() => Some(entry),
                _ => None,
            }
        })
        .collect();

    Ok(entries)
}
