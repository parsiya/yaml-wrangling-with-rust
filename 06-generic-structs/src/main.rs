use serde::{Serialize, Deserialize};

// This allows us to split the rules without caring about their contents.
#[derive(Debug, Serialize, Deserialize)]
struct GenericRuleFile {
    rules: Vec<serde_yaml::Mapping>,
}

fn main() {
    let content = std::fs::read_to_string("../multiple-rules.yaml").unwrap();
    // deserialize
    let generic_rule_file: GenericRuleFile = serde_yaml::from_str(&content).unwrap();

    // go through each rule. Each one is a Mapping.
    for single_rule in generic_rule_file.rules {
        
        // create a new GenericRuleFile object with only one rule.
        let mut new_rules: Vec<serde_yaml::Mapping> = Vec::new();
        new_rules.push(single_rule);
        let new_generic_rule = GenericRuleFile{
            rules: new_rules,
        };

        // convert it to yaml.
        let single_rule_yaml: String = serde_yaml::to_string(&new_generic_rule).unwrap();
        // print it.
        println!("{}", single_rule_yaml);
    }
}