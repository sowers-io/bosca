use crate::usx::item::{IUsxItem, UsxItem};
use std::collections::HashMap;

pub struct HtmlContext {
    pub pretty: bool,
    pub include_footnotes: bool,
    pub include_cross_references: bool,
    pub include_verse_numbers: bool,
    indent: i32,
}

impl HtmlContext {
    pub fn new(
        pretty: bool,
        include_footnotes: bool,
        include_cross_references: bool,
        include_verse_numbers: bool,
    ) -> Self {
        Self {
            include_verse_numbers,
            pretty,
            include_footnotes,
            include_cross_references,
            indent: 0,
        }
    }

    pub fn add_indent(&mut self) {
        self.indent += 2
    }

    pub fn remove_indent(&mut self) {
        self.indent -= 2
    }

    pub fn render(&mut self, tag: &str, item: &UsxItem, text: &Option<String>) -> String {
        let mut html = String::new();
        if self.pretty {
            for _ in 0..self.indent {
                html.push(' ');
            }
        }
        html.push('<');
        html.push_str(tag);
        let mut attrs = HashMap::<String, String>::new();
        if let Some(item_attrs) = item.html_attributes() {
            for entry in item_attrs.iter() {
                attrs.insert(entry.0.clone(), entry.1.clone());
            }
        }
        if let Some(html_class) = item.html_class() {
            attrs.insert("class".to_string(), html_class.clone());
        }
        for (key, value) in attrs {
            let v = format!(" {}=\"{}\"", key, value);
            html.push_str(v.as_str());
        }
        html.push('>');
        if self.pretty {
            html.push('\n');
        }
        self.add_indent();
        let mut child_html = String::new();
        if let Some(text) = text {
            if self.pretty {
                for _ in 0..self.indent {
                    html.push(' ');
                }
            }
            child_html.push_str(text.as_str());
        } else if let Some(children) = item.children() {
            for child in children {
                child_html.push_str(&child.lock().unwrap().to_html(self));
            }
        } else {
            child_html.push_str(&item.to_html(self));
        }
        html.push_str(child_html.as_str());
        if self.pretty && !html.ends_with("\n") {
            html.push('\n');
        }
        self.remove_indent();
        if self.pretty {
            for _ in 0..self.indent {
                html.push(' ');
            }
        }
        html.push_str("</");
        html.push_str(tag);
        html.push('>');
        if self.pretty {
            html.push('\n');
        }
        html
    }
}
