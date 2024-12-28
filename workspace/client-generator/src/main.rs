use std::fs;
use crate::context::Context;
use crate::introspection::Root;
use crate::parser::parse;
use clap::{Parser, ValueEnum};
use serde_json::json;
use std::fs::File;
use std::io::Write;

mod context;
mod generators;
mod introspection;
mod model;
mod parser;

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    Typescript,
    Rust,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,
    #[arg(short, long)]
    queries: String,
    #[arg(short, long)]
    output: String,
    #[arg(short, long)]
    format: Format,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let args = Args::parse();

    let client = reqwest::Client::new();
    let response = client
        .post(args.url)
        .body(r#"{"query":"query IntrospectionQuery { __schema { queryType { name } mutationType { name } subscriptionType { name } types {  ...FullType } directives {  name  description   locations  args {  ...InputValue  } } }  }  fragment FullType on __Type { kind name description fields(includeDeprecated: true) { name description args {  ...InputValue } type {  ...TypeRef } isDeprecated deprecationReason } inputFields { ...InputValue } interfaces { ...TypeRef } enumValues(includeDeprecated: true) { name description isDeprecated deprecationReason } possibleTypes { ...TypeRef }  }  fragment InputValue on __InputValue { name description type { ...TypeRef } defaultValue } fragment TypeRef on __Type { kind name ofType { kind name ofType { kind name ofType { kind name ofType { kind name ofType { kind name ofType { kind name ofType { kind name ofType { kind name ofType { kind name } } } } } } } } } }","operationName":"IntrospectionQuery"}"#)
        .send()
        .await
        .unwrap();
    let root = response.json::<Root>().await.unwrap();
    let mut context = Context::default();
    let models = root.data.__schema.to_class_models(&mut context);

    let dir = fs::read_dir(args.queries).unwrap();

    let mut documents = vec![];
    
    for entry in dir {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        let path = entry_path.as_path();
        println!("parsing {}", path.display());
        let document = parse(fs::read_to_string(path).unwrap().as_str());
        let model = models.iter().find(|model| model.name == "Query").unwrap();
        model.apply(&mut context, &document);
        documents.push(document);
    }

    let mut file = File::create(format!("{}/queries.json", args.output)).unwrap();
    file.write_all(json!(documents).to_string().as_bytes()).unwrap();

    context.build_interface_fields();

    let models = context.get_class_models();
    let mut file = File::create(format!("{}/models.json", args.output)).unwrap();
    file.write_all(json!(models).to_string().as_bytes()).unwrap();

    let mut file = File::create(format!("{}/context.json", args.output)).unwrap();
    file.write_all(json!(context).to_string().as_bytes()).unwrap();

    match args.format {
        Format::Typescript => {
            let mut file = File::create(format!("{}/models.ts", args.output)).unwrap();
            generators::typescript::generate(&context, &mut file)
        },
        Format::Rust => {
            let mut file = File::create(format!("{}/models.rs", args.output)).unwrap();
            generators::rust::generate(&context, &mut file)
        },
    }
}
