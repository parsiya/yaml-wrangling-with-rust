mod semgrep_rules;
mod utils;

use std::{env, collections::HashMap};

fn main() {

    let args: Vec<String> = env::args().collect();

    // this will panic if nothing is passed.
    let command = &args[1];

    match command.as_str() {
        "test-rules" => test_rules(&args[2].as_str()),
        "index-rules" => create_rule_index(&args[2].as_str()),
        _ => (),
    };

}

fn test_rules(registry_path: &str) {
    // check if the path exists and is a directory, otherwise, panic!
    utils::check_registry_path(registry_path);

    // find and store all rule files.
    let rule_file_paths = utils::find_rules(registry_path.to_string());

    // optional: print these paths.
    // for rule_file in rule_file_paths {
    //     println!("{}", rule_file);
    // }

    // read each rule file
    for rule_file in rule_file_paths {

        // read the rule.
        let contents = utils::read_file_to_string(&rule_file);

        // try to deserialize it
        let deserialized_result = serde_yaml::from_str::<semgrep_rules::RuleFile>(&contents);

        // check for errors.
        match deserialized_result {
            Ok(_) => (),
            Err(e) => println!("[!] File: {}\n\tError: {}", rule_file, e.to_string()),
        };
    }
}


// store all rules in a HashMap where key is rule ID and the value is the rule.
pub fn create_rule_index(registry_path: &str) {
    
    utils::check_registry_path(registry_path);
    // store all rule file paths
    let rule_file_paths = utils::find_rules(registry_path.to_string());
    
    let mut rule_index: HashMap<String, semgrep_rules::Rule> = HashMap::new();

    for rule_file in rule_file_paths {
        
        // read the rule file and deserialize it.
        let contents = utils::read_file_to_string(&rule_file);
        let deserialized_result = serde_yaml::from_str::<semgrep_rules::RuleFile>(&contents);

        // check for errors.
        let deserialized = match deserialized_result {
            Ok(rf) => rf,
            Err(e) => {
                // log the error and move to the next file.
                println!("[!] File: {}\n\tError: {}", rule_file, e.to_string());
                continue;
            },
        };

        // iterate through the rules in the rule file and extract all the rules.
        for individual_rule in deserialized.rules {
            // get the id and own it.
            rule_index.insert(individual_rule.id.to_owned(), individual_rule);
        }
    }
    println!("Number of rules in the index: {}", rule_index.keys().len());
    // print the keys.
    // println!("{:#?}", rule_index.keys());
}