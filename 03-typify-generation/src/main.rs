use schemars::schema::Schema;
use typify::{TypeSpace, TypeSpaceSettings};

fn main() {

    // read the JSON schema in YAML format.
    let content = std::fs::read_to_string("../rule_schema.yaml").unwrap();

    // create the schema (this is useful if you want to validate).
    let schema: schemars::schema::RootSchema = serde_yaml::from_str::<schemars::schema::RootSchema>(&content).unwrap();

    // if the file was in JSON instead of YAML.
    // let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content).unwrap();

    // optionally print the schema.
    // println!("{:#?}", schema);

    // I have no idea what's happening here.
    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));

    // panic here.
    type_space.add_ref_types(schema.definitions).unwrap();
    let base_type = &schema.schema;
    // Only convert the top-level type if it has a name [comment from the original code]
    if (|| base_type.metadata.as_ref()?.title.as_ref())().is_some() {
        let _ = type_space.add_type(&Schema::Object(schema.schema)).unwrap();
    }

    // add the derive directives to the struct
    let code = format!(
        "{}\n{}",
        "use serde::{Deserialize, Serialize};",
        type_space.to_string()
    );

    println!("{}", code);
}