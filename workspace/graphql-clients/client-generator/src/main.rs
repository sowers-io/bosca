use crate::context::Context;
use crate::introspection::{Kind, Root};
use crate::model::{ClassModel, ClassType, FieldModel, FieldType};
use crate::parser::parse;
use clap::{Parser, ValueEnum};
use serde_json::json;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;

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
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(r#"{"query":"query IntrospectionQuery { __schema { queryType { name } mutationType { name } subscriptionType { name } types {  ...FullType } directives {  name  description   locations  args {  ...InputValue  } } }  }  fragment FullType on __Type { kind name description fields(includeDeprecated: true) { name description args {  ...InputValue } type {  ...TypeRef } isDeprecated deprecationReason } inputFields { ...InputValue } interfaces { ...TypeRef } enumValues(includeDeprecated: true) { name description isDeprecated deprecationReason } possibleTypes { ...TypeRef }  }  fragment InputValue on __InputValue { name description type { ...TypeRef } defaultValue } fragment TypeRef on __Type { kind name ofType { kind name ofType { kind name ofType { kind name ofType { kind name ofType { kind name ofType { kind name ofType { kind name ofType { kind name ofType { kind name } } } } } } } } } }","operationName":"IntrospectionQuery"}"#)
        .send()
        .await
        .unwrap();
    if !response.status().is_success() {
        panic!("failed to introspect schema: {} -> {}", response.status(), response.text().await.unwrap());
    }
    let introspection_response = response.text().await.unwrap();
    let root = serde_json::from_str::<Root>(&introspection_response).unwrap();
    let mut context = Context::default();
    let models = root.data.__schema.to_class_models(&mut context);

    let dir = fs::read_dir(args.queries).unwrap();

    let mut documents = vec![];

    for entry in dir {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        let path = entry_path.as_path();
        if !path.display().to_string().ends_with(".graphql") {
            continue;
        }
        println!("parsing {}", path.display());
        let mut document = parse(fs::read_to_string(path).unwrap().as_str());
        let model_name = if document.document_type.unwrap() == parser::DocumentType::Mutation {
            "Mutation"
        } else {
            "Query"
        };
        let model = models
            .iter()
            .find(|model| model.name == model_name)
            .unwrap();
        document.output_object = model.apply(&mut context, &document);
        documents.push(document);
    }

    context.build_interface_fields();

    let input = ClassModel::new(
        ClassType::Interface,
        Kind::InputObject,
        "InputObject".to_owned(),
        "InputObject".to_owned(),
    );
    context.register_model(Arc::new(input));

    for document in documents.iter_mut() {
        if document.inputs.is_none() || document.inputs.as_ref().unwrap().is_empty() {
            continue;
        }
        let input_name = format!("{}Input", document.name);
        let mut input = ClassModel::new(
            ClassType::Class,
            Kind::InputObject,
            input_name.clone(),
            input_name,
        );
        if let Some(fields) = &document.inputs {
            for field in fields {
                let t = context.register_reference(field.type_name.as_ref().unwrap().as_str());
                input.add_field_model(FieldModel {
                    name: field.name.clone(),
                    field_type: FieldType::Object,
                    field_type_scalar: FieldType::Unknown,
                    field_type_references: vec![t],
                    nullable: false,
                })
            }
        }
        let input = Arc::new(input);
        document.input_object = Some(Arc::clone(&input));
        let r = context.register_model(Arc::clone(&input));
        context.register_interface_implementation("InputObject", r);
    }

    let mut file = File::create(format!("{}/queries.json", args.output)).unwrap();
    file.write_all(json!(documents).to_string().as_bytes())
        .unwrap();

    let models = context.get_class_models();
    let mut file = File::create(format!("{}/models.json", args.output)).unwrap();
    file.write_all(json!(models).to_string().as_bytes())
        .unwrap();

    let mut file = File::create(format!("{}/context.json", args.output)).unwrap();
    file.write_all(json!(context).to_string().as_bytes())
        .unwrap();

    let mut file = File::create(format!("{}/schema.json", args.output)).unwrap();
    file.write_all(introspection_response.as_bytes())
        .unwrap();

    match args.format {
        Format::Typescript => {
            let mut file = File::create(format!("{}/models.ts", args.output)).unwrap();
            generators::typescript::generate(&context, &mut file)
        }
        Format::Rust => {
            let mut file = File::create(format!("{}/models.rs", args.output)).unwrap();
            generators::rust::generate(&context, &mut file)
        }
    }

    match args.format {
        Format::Typescript => {
            let mut file = File::create(format!("{}/client.ts", args.output)).unwrap();
            generators::typescript::generate_client(&context, &documents, &mut file)
        }
        Format::Rust => {
            panic!("unsupported client output format");
        }
    }
}
