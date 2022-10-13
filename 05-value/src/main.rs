fn main() {
    let content = std::fs::read_to_string("../multiple-rules.yaml").unwrap();

    // // ----- start of section 1
    // use serde_yaml::Value;
    // // read the file.
    // let rule_file = serde_yaml::from_str::<Value>(&content).unwrap();

    // println!("{:#?}", rule_file);
    // // ----- end of section 1

    // // ----- start of section 2
    // use serde_yaml::Value;
    // let rule_file = serde_yaml::from_str::<Value>(&content).unwrap();
    // // convert it back to a YAML string.
    // let back_to_yaml = serde_yaml::to_string::<Value>(&rule_file).unwrap();
    // println!("{}", back_to_yaml);
    // // ----- end of section 2

    // // ----- start of section 3
    // use serde_yaml::{Mapping};
    // // get the file as a mapping.
    // let rule_file_mapping: Mapping = serde_yaml::from_str::<Mapping>(&content).unwrap();
    // // we know "rules" is a Sequence so we get it.
    // let rules = rule_file_mapping.get("rules").unwrap().as_sequence().unwrap();
    // // go through rules.
    // for rule in rules {
    //     // we know the "id" of each rule is a String.
    //     println!("{}", rule.get("id").unwrap().as_str().unwrap());
    // }
    // // ----- end of section 3
}
