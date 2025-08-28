use std::{error::Error, fs};

pub struct Config {
    target: String,
    file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Self, &'static str> {
        Ok(Config {
            target: args.nth(1).ok_or("No target file provided")?,
            file_path: args.next().ok_or("No file provided")?,
        })
    }
}

pub fn run(config: Config) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let target = config.target;
    let res = search(&contents, &target);
    Ok(res)
}

fn search<'a>(contents: &'a str, target: &str) -> Vec<String> {
    contents.lines()
        .filter(|line|line.contains(target)).map(|line|line.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_config() {
        let args = vec![
            "program".to_string(),
            "target.txt".to_string(),
            "file.txt".to_string(),
        ].into_iter();
        let config = Config::build(args).unwrap();
        assert_eq!(config.target, "target.txt");
        assert_eq!(config.file_path, "file.txt");
    }

    fn log() {
        let txt = "\
a
c\
B";
        println!("log: {}", txt);
    }

    #[test]
    fn test_log() {
        log();
    }

    #[test]
    fn test_search() {
        let contents = "\
This is a test file.
It contains some text.";
        let target = "contains";
        assert_eq!(search(contents, target), vec!["It contains some text."]);
    }

    #[test]
    fn test_run() {
        let args = vec![
            "program".to_string(),
            "o".to_string(),
            "/Applications/htc-3/self/rust-learn/_12/assert/test.log".to_string(),
        ].into_iter();
        let config = Config::build(args).unwrap();
        assert!(run(config).is_ok());
    }

    fn search_lower<'a>(contents: &'a str, target: &'a str) -> Vec<&'a str> {
        let mut vec = Vec::new();
        for content in contents.lines() {
            if content.to_lowercase().contains(&target.to_lowercase()) {
                vec.push(content);
            }
        }
        vec
    }

    #[test]
    fn test_search_lower() {
        let contents = "\
abc
DEF
G";
        let target = "def";
        let result = search_lower(contents, target);
        assert_eq!(result, vec!["DEF"]);
    }
}