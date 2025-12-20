use log::{info, warn};
use regex::Regex;
use std::path::PathBuf;

/// Replaces a filename using a pre-compiled regex pattern.
///
/// This function accepts a pre-compiled `Regex` to avoid recompiling the pattern
/// on every call, which improves performance when processing multiple files.
pub fn replace_name(
    name: &str,
    re: &Regex,
    to: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let filepath = PathBuf::from(name);

    // Handle case where path has no parent (e.g., root "/")
    let parent = match filepath.parent() {
        Some(p) => p,
        None => {
            warn!("Path has no parent directory: {}", name);
            return Ok(String::from(name));
        }
    };

    // Use pattern matching instead of comparing to None
    let filename = match filepath.file_name() {
        Some(f) => f,
        None => {
            warn!("Path is not a filename: {}", name);
            return Ok(String::from(name));
        }
    };

    // Handle non-UTF8 filenames
    let filename_str = match filename.to_str() {
        Some(s) => s,
        None => {
            warn!("Could not handle file with non-UTF8 name: {}", name);
            return Ok(String::from(name));
        }
    };

    if re.is_match(filename_str) {
        let renamed_file = re.replace_all(filename_str, to);
        // renamed_file is a Cow<str>, convert to owned String
        let renamed_file = renamed_file.to_string();
        let output = parent.join(renamed_file);

        // Handle non-UTF8 output paths
        let output_str = match output.to_str() {
            Some(s) => s,
            None => {
                warn!(
                    "Could not create output path with non-UTF8 characters for: {}",
                    name
                );
                return Ok(String::from(name));
            }
        };

        info!("Replacing {} into {}", name, output_str);
        return Ok(String::from(output_str));
    }

    Ok(String::from(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_simple_strings() {
        let re = Regex::new("bar").unwrap();
        let output = replace_name("foobar", &re, "buzz").unwrap();
        assert_eq!(output, "foobuzz");
    }

    #[test]
    fn without_match() {
        let re = Regex::new("buzz").unwrap();
        let output = replace_name("foobar", &re, "blob").unwrap();
        assert_eq!(output, "foobar");
    }

    #[test]
    fn with_simple_regex() {
        let re = Regex::new("(\\d+)").unwrap();
        let output = replace_name("number 16", &re, "nope").unwrap();
        assert_eq!(output, "number nope");
    }

    #[test]
    fn with_matching_replacement() {
        let re = Regex::new("number (\\d+)").unwrap();
        let output = replace_name("number 16", &re, "$1 number").unwrap();
        assert_eq!(output, "16 number");
    }

    #[test]
    fn with_simple_number() {
        let re = Regex::new("^(\\d+)$").unwrap();
        let output = replace_name("1", &re, "$1.dcm").unwrap();
        assert_eq!(output, "1.dcm");
    }

    #[test]
    fn with_pathname() {
        let re = Regex::new("^path").unwrap();
        let output = replace_name("./path.txt", &re, "out").unwrap();
        assert_eq!(output, "./out.txt");
    }

    #[test]
    fn with_two_matches() {
        let re = Regex::new("(\\w+)-something-(\\d+)").unwrap();
        let output = replace_name("test-something-42", &re, "$1-check-$2").unwrap();
        assert_eq!(output, "test-check-42");
    }
}
