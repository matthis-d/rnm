use log::info;
use regex::Regex;

pub fn replace_name(name: &str, from: &str, to: &str) -> String {
    let re = Regex::new(from).unwrap();
    let output = re.replace_all(name, to);
    info!("Replacing {} into {}", name, output);
    format!("{}", output)
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
}
