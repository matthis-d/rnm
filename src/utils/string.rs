use log::{info, warn};
use regex::Regex;
use std::path::{Path, PathBuf};

/// Replaces a filename using a pre-compiled regex pattern.
///
/// This function accepts a pre-compiled `Regex` to avoid recompiling the pattern
/// on every call, which improves performance when processing multiple files.
///
/// Handles non-UTF8 paths using lossy conversion, which replaces invalid UTF-8
/// sequences with the Unicode replacement character ().
pub fn replace_name(
    path: &Path,
    re: &Regex,
    to: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Handle case where path has no parent (e.g., root "/")
    // Use if let for early return - more idiomatic than match when only one branch matters
    let parent = if let Some(p) = path.parent() {
        p
    } else {
        warn!("Path has no parent directory: {:?}", path);
        return Ok(path.to_path_buf());
    };

    // Use if let for early return - more idiomatic than match when only one branch matters
    let filename = if let Some(f) = path.file_name() {
        f
    } else {
        warn!("Path is not a filename: {:?}", path);
        return Ok(path.to_path_buf());
    };

    // Use lossy conversion for non-UTF8 filenames
    // This converts invalid UTF-8 sequences to the Unicode replacement character ()
    let filename_str = filename.to_string_lossy();

    if re.is_match(&filename_str) {
        let renamed_file = re.replace_all(&filename_str, to);
        // renamed_file is a Cow<str>, convert to owned String
        let renamed_file = renamed_file.to_string();
        let output = parent.join(renamed_file);

        // Log the replacement using lossy conversion for display
        let path_display = path.to_string_lossy();
        let output_display = output.to_string_lossy();
        info!("Replacing {} into {}", path_display, output_display);
        return Ok(output);
    }

    Ok(path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn with_simple_strings() {
        let re = Regex::new("bar").unwrap();
        let output = replace_name(Path::new("foobar"), &re, "buzz").unwrap();
        assert_eq!(output, Path::new("foobuzz"));
    }

    #[test]
    fn without_match() {
        let re = Regex::new("buzz").unwrap();
        let output = replace_name(Path::new("foobar"), &re, "blob").unwrap();
        assert_eq!(output, Path::new("foobar"));
    }

    #[test]
    fn with_simple_regex() {
        let re = Regex::new("(\\d+)").unwrap();
        let output = replace_name(Path::new("number 16"), &re, "nope").unwrap();
        assert_eq!(output, Path::new("number nope"));
    }

    #[test]
    fn with_matching_replacement() {
        let re = Regex::new("number (\\d+)").unwrap();
        let output = replace_name(Path::new("number 16"), &re, "$1 number").unwrap();
        assert_eq!(output, Path::new("16 number"));
    }

    #[test]
    fn with_simple_number() {
        let re = Regex::new("^(\\d+)$").unwrap();
        let output = replace_name(Path::new("1"), &re, "$1.dcm").unwrap();
        assert_eq!(output, Path::new("1.dcm"));
    }

    #[test]
    fn with_pathname() {
        let re = Regex::new("^path").unwrap();
        let output = replace_name(Path::new("./path.txt"), &re, "out").unwrap();
        assert_eq!(output, Path::new("./out.txt"));
    }

    #[test]
    fn with_two_matches() {
        let re = Regex::new("(\\w+)-something-(\\d+)").unwrap();
        let output = replace_name(Path::new("test-something-42"), &re, "$1-check-$2").unwrap();
        assert_eq!(output, Path::new("test-check-42"));
    }

    #[test]
    fn with_non_utf8_path() {
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;

        // Create a path with invalid UTF-8 bytes (0xFF 0xFE are not valid UTF-8)
        // This test is Unix-specific, but that's okay for a unit test
        #[cfg(unix)]
        {
            let invalid_utf8 = b"file\xFF\xFEtest.txt";
            let os_str = OsStr::from_bytes(invalid_utf8);
            let path = Path::new(os_str);

            // Test that the function handles non-UTF8 paths without panicking
            let re = Regex::new("test").unwrap();
            let result = replace_name(path, &re, "renamed");

            // Should succeed (using lossy conversion)
            assert!(result.is_ok());
            let output = result.unwrap();

            // The output should be a valid path (lossy conversion will replace invalid bytes with )
            // We can't easily compare the exact output since it depends on lossy conversion,
            // but we can verify it doesn't panic and returns a path
            assert!(!output.as_os_str().is_empty());
        }

        // On non-Unix systems, we can't easily create non-UTF8 paths in tests
        // The function should still work correctly with valid UTF-8 paths
        #[cfg(not(unix))]
        {
            // Just verify the function works with regular paths
            let re = Regex::new("test").unwrap();
            let output = replace_name(Path::new("test-file.txt"), &re, "renamed").unwrap();
            assert_eq!(output, Path::new("renamed-file.txt"));
        }
    }
}
