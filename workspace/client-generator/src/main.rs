use crate::context::Context;
use crate::introspection::Root;
use crate::parser::parse;

mod introspection;
mod model;
mod context;
mod generators;
mod parser;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.bosca.io/graphql")
        .body(r#"
          {"query":"query IntrospectionQuery { __schema { queryType { name } mutationType { name } subscriptionType { name } types {  ...FullType } directives {  name  description   locations  args {  ...InputValue  } } }  }  fragment FullType on __Type { kind name description fields(includeDeprecated: true) { name description args {  ...InputValue } type {  ...TypeRef } isDeprecated deprecationReason } inputFields { ...InputValue } interfaces { ...TypeRef } enumValues(includeDeprecated: true) { name description isDeprecated deprecationReason } possibleTypes { ...TypeRef }  }  fragment InputValue on __InputValue { name description type { ...TypeRef } defaultValue } fragment TypeRef on __Type { kind name ofType { kind name ofType { kind name ofType { kind name ofType { kind name ofType { kind name ofType { kind name ofType { kind name ofType { kind name ofType { kind name } } } } } } } } } }","operationName":"IntrospectionQuery"}"#)
        .send()
        .await
        .unwrap();
    let root = response.json::<Root>().await.unwrap();
    let mut context = Context::default();
    let models = root.data.__schema.to_class_models(&mut context);

    let query = models.iter().find(|model| model.name == "Query").unwrap();
    let document = parse(r#"query GetMetadata {
          content {
            metadata {
              id
              name
              content {
                urls {
                  download {
                    url
                  }
                }
              }
            }
          }
    }"#);

    query.apply(&mut context, &document);

    context.build_interface_fields();

    let models = context.get_classes().iter().map(|x| {
        if let Some(x) = x.get_model() {
            return x
        }
        panic!("missing model: {}", x.name)
    }).collect::<Vec<_>>();

    generators::typescript::generate(&context, &models);
}
