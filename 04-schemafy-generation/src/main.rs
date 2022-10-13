fn main() {
    let content = std::fs::read_to_string("../rule_schema.yaml").unwrap();
    let schema = serde_yaml::from_str(&content).unwrap();

    use schemafy_lib::Expander;
    let mut expander = Expander::new(
        Some("Schema"),
        "::schemafy_core::",
        &schema,
    );

    let code = expander.expand(&schema);
    println!("{}", code.to_string());
}