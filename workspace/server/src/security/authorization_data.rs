#[derive(PartialEq, Eq)]
pub enum AuthorizationDataType {
    Header,
    Cookie,
}

pub struct AuthorizationData {
    pub data_type: AuthorizationDataType,
    pub data: String,
}

impl AuthorizationData {
    pub fn new(data_type: AuthorizationDataType, data: String) -> AuthorizationData {
        AuthorizationData { data_type, data }
    }
}
