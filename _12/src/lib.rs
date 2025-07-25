use std::{error::Error, fs};

pub struct Config {
    target: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("Usage: <program> <target_file> <file>");
        }
        Ok(Config {
            target: args
                .get(1)
                .cloned()
                .expect("No target file provided")
                .to_string(),
            file_path: args.get(2).cloned().expect("No file provided").to_string(),
        })
    }
}

pub fn run(config: Config) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let target = config.target;
    let res = search(&contents, &target);
    Ok(res)
}

fn search<'a>(contents: &'a str, target: &'a str) -> Vec<String> {
    let mut vec = Vec::new();
    for content in contents.lines() {
        if content.contains(target) {
            vec.push(content.to_string());
        }
    }
    vec
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
        ];
        let config = Config::build(&args).unwrap();
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
        ];
        let config = Config::build(&args).unwrap();
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