use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct IndexQuery {
    pub q: String,
    pub offset: i64,
    pub limit: i64,
    pub filter: Option<String>,
    pub sort: Option<Vec<String>>,
    pub facets: Option<Vec<String>>,
    pub distinct: Option<String>,
    pub hybrid: Option<Hybrid>
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Hybrid {
    pub embedder: String,
}
