use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Dblmetadata {
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@revision")]
    pub revision: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub identification: Identification,
    #[serde(rename = "type")]
    pub dblmetadata_type: Type,
    pub relationships: Relationships,
    pub agencies: Agencies,
    pub language: Language,
    pub countries: Countries,
    pub format: Format,
    pub names: Names,
    pub manifest: Manifest,
    pub source: Source,
    pub publications: Publications,
    pub copyright: Copyright,
}

#[derive(Serialize, Deserialize)]
pub struct Identification {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub name: String,
    #[serde(rename = "nameLocal")]
    pub name_local: String,
    pub description: String,
    pub abbreviation: String,
    #[serde(rename = "abbreviationLocal")]
    pub abbreviation_local: String,
    pub scope: String,
    #[serde(rename = "dateCompleted")]
    pub date_completed: String,
    #[serde(rename = "bundleProducer")]
    pub bundle_producer: String,
    #[serde(rename = "systemId")]
    pub system_id: Vec<SystemId>,
}

#[derive(Serialize, Deserialize)]
pub struct SystemId {
    #[serde(rename = "@type")]
    pub system_id_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub id: String,
    pub name: Option<String>,
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
    #[serde(rename = "csetId")]
    pub cset_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Type {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub medium: String,
    #[serde(rename = "isConfidential")]
    pub is_confidential: String,
    #[serde(rename = "hasCharacters")]
    pub has_characters: String,
    #[serde(rename = "isTranslation")]
    pub is_translation: String,
    #[serde(rename = "isExpression")]
    pub is_expression: String,
    #[serde(rename = "translationType")]
    pub translation_type: String,
    pub audience: String,
    #[serde(rename = "projectType")]
    pub project_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Relationships {}

#[derive(Serialize, Deserialize)]
pub struct Agencies {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "rightsHolder")]
    pub rights_holder: RightsHolder,
    #[serde(rename = "rightsAdmin")]
    pub rights_admin: RightsAdmin,
    pub contributor: Contributor,
}

#[derive(Serialize, Deserialize)]
pub struct RightsHolder {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub abbr: String,
    pub url: String,
    #[serde(rename = "nameLocal")]
    pub name_local: String,
    pub uid: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RightsAdmin {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub url: String,
    pub uid: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Contributor {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub content: String,
    pub publication: String,
    pub management: String,
    pub finance: String,
    pub qa: String,
    pub uid: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Language {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub iso: String,
    pub name: String,
    #[serde(rename = "nameLocal")]
    pub name_local: String,
    pub script: String,
    #[serde(rename = "scriptCode")]
    pub script_code: String,
    #[serde(rename = "scriptDirection")]
    pub script_direction: String,
    pub ldml: String,
    pub rod: String,
    pub numerals: String,
}

#[derive(Serialize, Deserialize)]
pub struct Countries {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub country: Country,
}

#[derive(Serialize, Deserialize)]
pub struct Country {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub iso: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Format {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "usxVersion")]
    pub usx_version: String,
    #[serde(rename = "versedParagraphs")]
    pub versed_paragraphs: String,
}

#[derive(Serialize, Deserialize)]
pub struct Names {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub name: Vec<NamesName>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NamesName {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub abbr: String,
    pub short: String,
    pub long: String,
}

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub resource: Vec<Resource>,
}

#[derive(Serialize, Deserialize)]
pub struct Resource {
    #[serde(rename = "@checksum")]
    pub checksum: String,
    #[serde(rename = "@mimeType")]
    pub mime_type: String,
    #[serde(rename = "@size")]
    pub size: String,
    #[serde(rename = "@uri")]
    pub uri: String,
}

#[derive(Serialize, Deserialize)]
pub struct Source {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "canonicalContent")]
    pub canonical_content: SourceCanonicalContent,
    pub structure: SourceStructure,
}

#[derive(Serialize, Deserialize)]
pub struct SourceCanonicalContent {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub book: Vec<SourceCanonicalContentBook>,
}

#[derive(Serialize, Deserialize)]
pub struct SourceCanonicalContentBook {
    #[serde(rename = "@code")]
    pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct SourceStructure {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub content: SourceStructureContent,
}

#[derive(Serialize, Deserialize)]
pub struct SourceStructureContent {
    #[serde(rename = "@src")]
    pub src: String,
    #[serde(rename = "@role")]
    pub role: String,
}

#[derive(Serialize, Deserialize)]
pub struct Publications {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub publication: PublicationsPublication,
}

#[derive(Serialize, Deserialize)]
pub struct PublicationsPublication {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@default")]
    pub default: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub name: String,
    #[serde(rename = "nameLocal")]
    pub name_local: String,
    pub description: String,
    #[serde(rename = "descriptionLocal")]
    pub description_local: String,
    pub abbreviation: String,
    #[serde(rename = "abbreviationLocal")]
    pub abbreviation_local: String,
    #[serde(rename = "canonicalContent")]
    pub canonical_content: PublicationCanonicalContent,
    pub structure: PublicationStructure,
}

#[derive(Serialize, Deserialize)]
pub struct PublicationCanonicalContent {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub book: Vec<PublicationCanonicalContentBook>,
}

#[derive(Serialize, Deserialize)]
pub struct PublicationCanonicalContentBook {
    #[serde(rename = "@code")]
    pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct PublicationStructure {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub content: Vec<PublicationStructureContent>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PublicationStructureContent {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@src")]
    pub src: String,
    #[serde(rename = "@role")]
    pub role: String,
}

#[derive(Serialize, Deserialize)]
pub struct Copyright {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "fullStatement")]
    pub full_statement: FullStatement,
}

#[derive(Serialize, Deserialize)]
pub struct FullStatement {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "statementContent")]
    pub statement_content: StatementContent,
}

#[derive(Serialize, Deserialize)]
pub struct StatementContent {
    #[serde(rename = "@type")]
    pub statement_content_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub p: StatementContentP,
}

#[derive(Serialize, Deserialize)]
pub struct StatementContentP {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub strong: String,
}

#[derive(Serialize, Deserialize)]
pub struct Promotion {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "promoVersionInfo")]
    pub promo_version_info: PromoVersionInfo,
}

#[derive(Serialize, Deserialize)]
pub struct PromoVersionInfo {
    #[serde(rename = "@contentType")]
    pub content_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub p: Vec<PromoVersionInfoP>,
}

#[derive(Serialize, Deserialize)]
pub struct PromoVersionInfoP {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub a: A,
}

#[derive(Serialize, Deserialize)]
pub struct A {
    #[serde(rename = "@href")]
    pub href: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}
