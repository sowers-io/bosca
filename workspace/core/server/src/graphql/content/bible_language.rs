use crate::models::bible::bible_language::BibleLanguage;
use async_graphql::Object;

pub struct BibleLanguageObject {
    language: BibleLanguage,
}

impl BibleLanguageObject {
    pub fn new(language: BibleLanguage) -> Self {
        Self { language }
    }
}

#[Object(name = "BibleLanguage")]
impl BibleLanguageObject {
    async fn iso(&self) -> &String {
        &self.language.iso
    }
    async fn name(&self) -> &String {
        &self.language.name
    }
    async fn name_local(&self) -> &String {
        &self.language.name_local
    }
    async fn script(&self) -> &String {
        &self.language.script
    }
    async fn script_code(&self) -> &String {
        &self.language.script_code
    }
    async fn script_direction(&self) -> &String {
        &self.language.script_direction
    }
}
