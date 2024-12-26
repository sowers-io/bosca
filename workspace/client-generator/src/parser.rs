use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::Write;

pub trait Fields {
    fn name(&self) -> &str;
    fn get_fields(&self) -> Option<&Vec<Field>>;
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DocumentType {
    Query,
    Mutation,
    Subscription,
    Fragment,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub name: String,
    pub document_type: Option<DocumentType>,
    #[serde(skip)]
    pub fields: Option<Vec<Field>>,
    pub inputs: Option<Vec<Field>>,
    pub query: String,
    pub sha256: String,
}

impl Fields for Document {
    fn name(&self) -> &str {
        &self.name
    }

    fn get_fields(&self) -> Option<&Vec<Field>> {
        self.fields.as_ref()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    #[serde(skip)]
    pub document_name: String,
    pub type_name: Option<String>,
    pub name: String,
    #[serde(skip)]
    pub fields: Option<Vec<Field>>,
    #[serde(skip)]
    pub inputs: Option<Vec<Field>>,
    pub nullable: Option<bool>,
}

impl Fields for Field {
    #[allow(clippy::misnamed_getters)]
    fn name(&self) -> &str {
        &self.document_name
    }
    fn get_fields(&self) -> Option<&Vec<Field>> {
        self.fields.as_ref()
    }
}

impl Fields for Box<Field> {
    #[allow(clippy::misnamed_getters)]
    fn name(&self) -> &str {
        &self.document_name
    }
    fn get_fields(&self) -> Option<&Vec<Field>> {
        self.fields.as_ref()
    }
}

pub fn parse(query_to_parse: &str) -> Document {
    let mut hasher = Sha256::new();
    hasher.update(query_to_parse);
    let result = hasher.finalize();

    let re = Regex::new(r"\s+").unwrap();
    let doc_query = query_to_parse
        .to_owned()
        .replace("\n", " ")
        .replace("\r", " ")
        .replace("\t", " ");
    let clean_query = re.replace_all(&doc_query, " ");
    let mut document = Document {
        name: "".to_owned(),
        document_type: None,
        fields: None,
        query: clean_query.to_string(),
        inputs: None,
        sha256: hex::encode(result),
    };

    let mut buf = String::new();

    let mut stack = Vec::<Field>::new();
    let chars = query_to_parse.trim().chars().collect::<Vec<char>>();

    parse_type_name(&mut document, None, &chars, 0, &mut buf, &mut stack);

    if !buf.is_empty() {
        panic!("invalid state: `{}`", buf)
    }
    document
}

fn scan_for_char(chars: &[char], start: usize, find: char, negate: bool) -> usize {
    for (i, char) in chars.iter().enumerate().skip(start) {
        if (negate && *char != find) || (!negate && *char == find) {
            return i;
        }
    }
    start
}

fn parse_input(
    document: &mut Document,
    chars: &[char],
    start: usize,
) -> (Vec<Field>, usize) {
    let mut variable_name = String::new();
    let mut type_name = String::new();
    let mut variable = true;
    let mut i = start;
    let mut inputs = Vec::<Field>::new();
    while i < chars.len() {
        let char = chars[i];
        match char {
            'A'..='Z' | 'a'..='z' | '_' => {
                if variable {
                    variable_name.push(char);
                } else {
                    type_name.push(char);
                }
            }
            '$' => {
            }
            ':' => {
                variable = false;
            }
            '[' => {
            }
            ']' => {
            }
            ' ' | '\r' | '\n' | '\t' | ',' | ')' | '!' => {
                if !variable && !type_name.is_empty() {
                    let field = Field {
                        document_name: document.name.clone(),
                        type_name: Some(type_name.to_owned()),
                        name: variable_name.to_owned(),
                        fields: None,
                        inputs: None,
                        nullable: Some(chars[i] != '!'),
                    };
                    inputs.push(field);
                    variable = true;
                    variable_name.clear();
                    type_name.clear();
                } else if !inputs.is_empty() && chars[i] == '!' {
                    variable = true;
                    inputs.last_mut().unwrap().nullable = Some(false);
                }
                if char == ')' {
                    return (inputs, i + 1);
                }
            }
            _ => {
                panic!("invalid input state: `{}`", char);
            }
        }
        i += 1;
    }
    panic!("invalid state");
}

fn parse_type_name(
    document: &mut Document,
    type_name: Option<String>,
    chars: &[char],
    start: usize,
    buf: &mut String,
    stack: &mut Vec<Field>,
) -> usize {
    let mut alias = String::new();
    // let mut ignore = false;
    let mut i = start;
    while i < chars.len() {
        let char = chars[i];
        // if ignore {
        //     if char == ')' {
        //         ignore = false;
        //     }
        //     i += 1;
        //     continue;
        // }
        match char {
            'A'..='Z' | 'a'..='z' | '_' => {
                buf.push(char);
            }
            '.' => {
                if chars[i + 1] == '.' && chars[i + 2] == '.' {
                    i = scan_for_char(chars, i + 3, ' ', true);
                    if chars[i] == 'o' && chars[i + 1] == 'n' {
                        i = scan_for_char(chars, i + 2, ' ', true);
                    }
                }
            }
            '(' => {
                let (inputs, next_i) = parse_input(document, chars, i + 1);
                i = next_i;
                if stack.is_empty() {
                    document.inputs = Some(inputs);
                } else {
                    stack.last_mut().unwrap().inputs = Some(inputs);
                }
                // ignore = true;
                continue;
            }
            '{' => {
                if let Some(current) = stack.last_mut() {
                    if current.fields.is_none() {
                        current.fields = Some(vec![]);
                    }
                    if current.fields.as_ref().unwrap().is_empty() {
                        stack.push(Field {
                            document_name: document.name.clone(),
                            type_name: type_name.clone(),
                            name: if alias.is_empty() {
                                buf.trim().to_owned()
                            } else {
                                alias.to_owned()
                            },
                            fields: None,
                            inputs: None,
                            nullable: Some(true),
                        });
                        buf.clear();
                        alias.clear();
                    } else {
                        let last = current.fields.as_mut().unwrap().pop().unwrap();
                        stack.push(last);
                    }
                } else if document.name.is_empty() {
                    document.name = buf.trim().to_owned();
                    buf.clear();
                } else {
                    stack.push(Field {
                        document_name: document.name.clone(),
                        type_name: type_name.clone(),
                        name: "root".to_owned(),
                        fields: None,
                        inputs: None,
                        nullable: Some(true),
                    });
                }
            }
            ':' => {
                let val = buf.to_owned();
                alias.write_str(&val).unwrap();
                buf.clear();
            }
            ' ' | '\r' | '\n' | '\t' => {
                if document.document_type.is_none() {
                    match buf.trim().to_lowercase().as_str() {
                        "query" => {
                            document.document_type = Some(DocumentType::Query);
                        }
                        "mutation" => {
                            document.document_type = Some(DocumentType::Mutation);
                        }
                        "subscription" => {
                            document.document_type = Some(DocumentType::Subscription);
                        }
                        "fragment" => {
                            document.document_type = Some(DocumentType::Fragment);
                        }
                        _ => panic!("invalid document type: {}", buf),
                    }
                    buf.clear();
                } else if !buf.is_empty() {
                    if let Some(field) = stack.last_mut() {
                        if field.fields.is_none() {
                            field.fields = Some(vec![]);
                        }
                        if field.name.is_empty() {
                            field.name = if alias.is_empty() {
                                buf.trim().to_owned()
                            } else {
                                alias.to_owned()
                            };
                            buf.clear();
                        } else {
                            let children = field.fields.as_mut().unwrap();
                            children.push(Field {
                                document_name: document.name.clone(),
                                type_name: type_name.clone(),
                                name: if alias.is_empty() {
                                    buf.trim().to_owned()
                                } else {
                                    alias.to_owned()
                                },
                                fields: None,
                                inputs: None,
                                nullable: Some(true),
                            });
                            buf.clear();
                            alias.clear();
                        }
                    } else if document.name.is_empty() {
                        document.name = buf.trim().to_owned();
                        buf.clear();
                    } else {
                        panic!("invalid state: `{}`", buf)
                    }
                }
            }
            '}' => {
                let mut last = stack.pop().unwrap();
                if last.name.is_empty() {
                    last.name = if alias.is_empty() {
                        buf.trim().to_owned()
                    } else {
                        alias.to_owned()
                    };
                    buf.clear();
                    alias.clear();
                }
                if stack.is_empty() {
                    if document.fields.is_none() {
                        document.fields = Some(vec![]);
                    }
                    let fields = document.fields.as_mut().unwrap();
                    for field in last.fields.unwrap() {
                        fields.push(field);
                    }
                } else {
                    stack
                        .last_mut()
                        .unwrap()
                        .fields
                        .as_mut()
                        .unwrap()
                        .push(last);
                }
            }
            _ => {
                panic!("invalid state: `{}`", char);
            }
        }
        i += 1;
    }
    chars.len()
}
