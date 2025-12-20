use std::fs;
use std::path::Path;

/// Returns an iterator over file entries in the specified directory.
///
/// This function returns an iterator instead of a Vec to improve memory efficiency
/// when processing large directories. The iterator lazily processes entries as they
/// are consumed, rather than loading all entries into memory at once.
///
/// # Errors
///
/// Returns an error if the directory cannot be read.
pub fn get_file_entries(
    path: &Path,
) -> std::io::Result<Box<dyn Iterator<Item = fs::DirEntry> + Send>> {
    let read_dir = fs::read_dir(path)?;

    Ok(Box::new(
        read_dir
            // Keep readable entries
            .filter_map(Result::ok)
            // Keep entries with a readable file type
            .filter_map(|entry| {
                // Handle file_type() error gracefully - skip entries we can't read
                match entry.file_type() {
                    Ok(file_type) if file_type.is_file() => Some(entry),
                    _ => None,
                }
            }),
    ))
}
