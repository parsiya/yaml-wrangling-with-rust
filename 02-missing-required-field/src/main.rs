use std::fs;

fn main() {
    let contents = fs::read_to_string("../multiple-rules.yaml")
        .expect("Should have been able to read the file");

    let rule_file: RuleFile = serde_yaml::from_str::<RuleFile>(&contents).unwrap();

    println!("{:#?}", rule_file)

}

use serde::{Serialize, Deserialize};

// struct to hold the entire rule file
#[derive(Debug, Serialize, Deserialize)]
struct RuleFile {
    rules: Vec<Rule>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Rule {
    id: String,
    languages: Vec<String>,
    // added field
    dummy: String,
}