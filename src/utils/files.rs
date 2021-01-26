use std::fs;

pub fn get_file_entries(pathname: &str) -> Vec<fs::DirEntry> {
    fs::read_dir(pathname)
        .unwrap()
        // Keep readable entries
        .filter_map(Result::ok)
        // Keep entries with a redable file type
        .filter(|entry| Result::is_ok(&entry.file_type()))
        // Keep entries of file type
        .filter(|entry| entry.file_type().unwrap().is_file())
        // Return an iterable
        .collect()
}
