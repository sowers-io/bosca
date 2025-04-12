use async_graphql::Object;
use crate::models::security::credentials::CredentialType;

pub struct PrincipalCredentialObject {
    identifier: String,
    credential: CredentialType,
}

impl PrincipalCredentialObject {

    pub fn new(identifier: String, credential: CredentialType) -> Self {
        Self {
            identifier,
            credential,
        }
    }
}

#[Object(name = "PrincipalCredential")]
impl PrincipalCredentialObject {
    async fn identifier(&self) -> &String {
        &self.identifier
    }

    #[graphql(name = "type")]
    async fn credential_type(&self) -> &CredentialType {
        &self.credential
    }
}
