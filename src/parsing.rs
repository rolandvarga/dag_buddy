use itertools::Itertools;
use regex::Regex;
use std::fs;

fn error_parsing(_file_name: &str) -> &str {
    return "Unable to parse '{_file_name}'";
}

pub fn parse_file(file_name: &str) -> String {
    println!("parsing file: '{file_name}'");
    return fs::read_to_string(file_name).expect(error_parsing(file_name));
}

pub fn parse_tables(file: &str) -> Vec<String> {
    let re: Regex = Regex::new(r"from\s*(?P<table>[a-zA-Z_]*)").unwrap();
    let file_lower = file.to_lowercase();

    let tables = re.captures_iter(file_lower.as_str()).filter_map(|cap| {
        let group = cap.get(1);
        match group {
            Some(name) => Some(name.as_str().to_string()),
            _ => None,
        }
    });
    tables
        .map(|m| m.to_string())
        .into_iter()
        .unique()
        .collect::<Vec<String>>()
}
