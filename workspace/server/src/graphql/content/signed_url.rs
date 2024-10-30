use crate::models::content::signed_url::{SignedUrl, SignedUrlHeader};
use async_graphql::Object;

pub struct SignedUrlObject {
    url: SignedUrl,
}

pub struct SignedUrlHeaderObject {
    header: SignedUrlHeader,
}

#[Object(name = "SignedUrlHeader")]
impl SignedUrlHeaderObject {
    pub async fn name(&self) -> &String {
        &self.header.name
    }

    pub async fn value(&self) -> &String {
        &self.header.value
    }
}

impl SignedUrlObject {
    pub fn new(url: SignedUrl) -> Self {
        Self { url }
    }
}

#[Object(name = "SignedUrl")]
impl SignedUrlObject {
    async fn url(&self) -> &String {
        &self.url.url
    }
    async fn headers(&self) -> Vec<SignedUrlHeaderObject> {
        self.url
            .headers
            .iter()
            .map(|i| SignedUrlHeaderObject { header: i.clone() })
            .collect()
    }
}

impl From<SignedUrl> for SignedUrlObject {
    fn from(url: SignedUrl) -> Self {
        Self::new(url)
    }
}
