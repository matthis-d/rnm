use log::{info, warn};
use regex::Regex;
use std::path::PathBuf;

pub fn replace_name(name: &str, from: &str, to: &str) -> String {
    let re = Regex::new(from).unwrap();

    let filepath = PathBuf::from(name);
    let parent = filepath.parent().unwrap();
    let filename = filepath.file_name();

    if None == filename {
        warn!("Path is not a filename");
        return String::from(name);
    }

    let filename = filename.unwrap().to_str();

    if None == filename {
        warn!("Could not handle file {}", name);
        return String::from(name);
    }

    let filename = filename.unwrap();

    if re.is_match(filename) {
        let renamed_file = re.replace_all(filename, to);
        let renamed_file = format!("{}", renamed_file);
        let output = parent.join(renamed_file);
        let output = output.to_str().unwrap();
        info!("Replacing {} into {}", name, output);
        return String::from(output);
    }

    String::from(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_simple_strings() {
        let output = replace_name("foobar", "bar", "buzz");
        assert_eq!(output, "foobuzz");
    }

    #[test]
    fn without_match() {
        let output = replace_name("foobar", "buzz", "blob");
        assert_eq!(output, "foobar");
    }

    #[test]
    fn with_simple_regex() {
        let output = replace_name("number 16", "(\\d+)", "nope");
        assert_eq!(output, "number nope");
    }

    #[test]
    fn with_matching_replacement() {
        let output = replace_name("number 16", "number (\\d+)", "$1 number");
        assert_eq!(output, "16 number");
    }

    #[test]
    fn with_simple_number() {
        let output = replace_name("1", "^(\\d+)$", "$1.dcm");
        assert_eq!(output, "1.dcm");
    }

    #[test]
    fn with_pathname() {
        let output = replace_name("./path.txt", "^path", "out");
        assert_eq!(output, "./out.txt");
    }

    #[test]
    fn with_two_matches() {
        let output = replace_name(
            "test-something-42",
            "(\\w+)-something-(\\d+)",
            "$1-check-$2",
        );
        assert_eq!(output, "test-check-42");
    }
}
