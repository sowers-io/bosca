use serde::{Deserialize, Serialize};

pub trait Fields {
    fn name(&self) -> &str;
    fn get_fields(&self) -> Option<&Vec<Box<Field>>>;
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DocumentType {
    Query,
    Mutation,
    Subscription,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub name: String,
    pub document_type: Option<DocumentType>,
    pub fields: Option<Vec<Box<Field>>>,
}

impl Fields for Document {
    fn name(&self) -> &str {
        &self.name
    }

    fn get_fields(&self) -> Option<&Vec<Box<Field>>> {
        self.fields.as_ref()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    pub document_name: String,
    pub name: String,
    pub fields: Option<Vec<Box<Field>>>,
}

impl Fields for Field {
    fn name(&self) -> &str {
        &self.document_name
    }
    fn get_fields(&self) -> Option<&Vec<Box<Field>>> {
        self.fields.as_ref()
    }
}

impl Fields for Box<Field> {
    fn name(&self) -> &str {
        &self.document_name
    }
    fn get_fields(&self) -> Option<&Vec<Box<Field>>> {
        self.fields.as_ref()
    }
}

pub fn parse(query_to_parse: &str) -> Document {
    let mut buf = String::new();
    let mut document = Document {
        name: "".to_owned(),
        document_type: None,
        fields: None,
    };
    let mut stack = Vec::<Field>::new();
    for char in query_to_parse.trim().chars() {
        match char {
            'A'..='Z' | 'a'..='z' => {
                buf.push(char);
            }
            '{' => {
                if let Some(current) = stack.last_mut() {
                    if current.fields.is_none() {
                        current.fields = Some(vec![]);
                    }
                    if current.fields.as_ref().unwrap().is_empty() {
                        stack.push(Field {
                            document_name: document.name.clone(),
                            name: buf.trim().to_owned(),
                            fields: None,
                        });
                        buf.clear();
                    } else {
                        let last = current.fields.as_mut().unwrap().pop().unwrap();
                        stack.push(*last);
                    }
                } else if document.name.is_empty() {
                    document.name = buf.trim().to_owned();
                    buf.clear();
                } else {
                    stack.push(Field {
                        document_name: document.name.clone(),
                        name: "root".to_owned(),
                        fields: None,
                    });
                }
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
                        _ => panic!("invalid document type: {}", buf),
                    }
                    buf.clear();
                } else if !buf.is_empty() {
                    if let Some(field) = stack.last_mut() {
                        if field.fields.is_none() {
                            field.fields = Some(vec![]);
                        }
                        if field.name.is_empty() {
                            field.name = buf.trim().to_owned();
                            buf.clear();
                        } else {
                            let children = field.fields.as_mut().unwrap();
                            children.push(Box::from(Field {
                                document_name: document.name.clone(),
                                name: buf.trim().to_owned(),
                                fields: None,
                            }));
                            buf.clear();
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
                    last.name = buf.trim().to_owned();
                    buf.clear();
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
                    stack.last_mut().unwrap().fields.as_mut().unwrap().push(Box::from(last));
                }
            }
            _ => {
                panic!("invalid state: `{}`", char);
            }
        }
    }
    if !buf.is_empty() {
        panic!("invalid state: `{}`", buf)
    }
    document
}
