use log::{info, warn};
use regex::Regex;
use std::path::PathBuf;

// IMPROVEMENT: Regex compilation is expensive - this function compiles the regex on every call
// Should accept a pre-compiled Regex instead, or compile once at a higher level
pub fn replace_name(
    name: &str,
    from: &str,
    to: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Compile regex and handle invalid regex patterns
    let re = Regex::new(from).map_err(|e| format!("Invalid regex pattern '{}': {}", from, e))?;

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
        let output = replace_name("foobar", "bar", "buzz").unwrap();
        assert_eq!(output, "foobuzz");
    }

    #[test]
    fn without_match() {
        let output = replace_name("foobar", "buzz", "blob").unwrap();
        assert_eq!(output, "foobar");
    }

    #[test]
    fn with_simple_regex() {
        let output = replace_name("number 16", "(\\d+)", "nope").unwrap();
        assert_eq!(output, "number nope");
    }

    #[test]
    fn with_matching_replacement() {
        let output = replace_name("number 16", "number (\\d+)", "$1 number").unwrap();
        assert_eq!(output, "16 number");
    }

    #[test]
    fn with_simple_number() {
        let output = replace_name("1", "^(\\d+)$", "$1.dcm").unwrap();
        assert_eq!(output, "1.dcm");
    }

    #[test]
    fn with_pathname() {
        let output = replace_name("./path.txt", "^path", "out").unwrap();
        assert_eq!(output, "./out.txt");
    }

    #[test]
    fn with_two_matches() {
        let output = replace_name(
            "test-something-42",
            "(\\w+)-something-(\\d+)",
            "$1-check-$2",
        )
        .unwrap();
        assert_eq!(output, "test-check-42");
    }

    #[test]
    fn with_invalid_regex() {
        let result = replace_name("test", "[invalid", "replacement");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid regex"));
    }
}
