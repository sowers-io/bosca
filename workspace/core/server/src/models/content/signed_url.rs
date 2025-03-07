#[derive(Clone)]
pub struct SignedUrlHeader {
    pub name: String,
    pub value: String,
}

#[derive(Clone)]
pub struct SignedUrl {
    pub url: String,
    pub headers: Vec<SignedUrlHeader>,
}
