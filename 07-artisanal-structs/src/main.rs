mod semgrep_rules;

fn main() {
    // read the file.
    let content = std::fs::read_to_string("../multiple-rules.yaml").unwrap();

    // deserialize it to RuleFile.
    let deserialize_result = serde_yaml::from_str::<semgrep_rules::RuleFile>(&content);

    // log the error.
    match deserialize_result {
        Ok(rf) => {
            println!("[+] Processed without errors.");
            // convert it to YAML and print.
            let yaml_back: String = serde_yaml::to_string(&rf).unwrap();
            println!("{}", yaml_back);
        },
        Err(e) => println!("[!] Error: {}", e.to_string()),
    };
}
