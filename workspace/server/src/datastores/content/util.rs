use crate::graphql::content::content::{ExtensionFilterType, FindAttributeInput};
use postgres_types::ToSql;
use crate::models::content::ordering::Order::Ascending;
use crate::models::content::ordering::Ordering;

pub fn build_ordering_names(ordering: &[Ordering], names: &mut Vec<String>) {
    for attr in ordering {
        for p in attr.path.iter() {
            names.push(p.clone());
        }
    }
}

pub fn build_ordering<'a>(
    attributes_column: &str,
    start_index: i32,
    ordering: &[Ordering],
    values: &mut Vec<&'a (dyn ToSql + Sync)>,
    names: &'a [String],
) -> String {
    let mut index = start_index;
    let mut buf = "order by ".to_owned();
    let mut n = 0;
    for (i, attr) in ordering.iter().enumerate() {
        if i > 0 {
            buf.push_str(", ");
        }
        buf.push_str(attributes_column);
        for _ in attr.path.iter() {
            let name = names.get(n).unwrap();
            n += 1;
            values.push(name as &(dyn ToSql + Sync));
            buf.push_str(format!("->${}", index).as_str());
            index += 1;
        }
        buf.push(' ');
        buf.push_str(if attr.order == Ascending {
            "asc"
        } else {
            "desc"
        });
    }
    if buf == "order by " {
        return "".to_owned();
    }
    buf
}


pub fn build_find_args<'a>(
    query: &str,
    alias: &str,
    attributes: &'a [FindAttributeInput],
    content_types: &'a Option<Vec<String>>,
    extension_filter: Option<ExtensionFilterType>,
    offset: &'a i64,
    limit: &'a i64,
) -> (String, Vec<&'a (dyn ToSql + Sync)>) {
    let mut q = query.to_string();
    let mut values = Vec::new();
    let mut pos = 1;
    if !attributes.is_empty() || (content_types.is_some() && !content_types.as_ref().unwrap().is_empty()) {
        q.push_str(" where ");
    }
    if !attributes.is_empty() {
        for i in 0..attributes.len() {
            let attr = attributes.get(i).unwrap();
            if i > 0 {
                q.push_str(" and ");
            }
            q.push_str(format!(" {}.attributes->>(${}::varchar) = ${}::varchar ", alias, pos, pos + 1).as_str());
            pos += 2;
            values.push(&attr.key as &(dyn ToSql + Sync));
            values.push(&attr.value as &(dyn ToSql + Sync));
        }
    }
    if let Some(content_types) = content_types {
        if !content_types.is_empty() {
            if !values.is_empty() {
                q.push_str(" and ");
            }
            q.push_str(format!(" {}.content_type in (", alias).as_str());
            for (ix, content_type) in content_types.iter().enumerate() {
                if ix > 0 {
                    q.push_str(", ");
                }
                q.push_str(format!("${}", pos).as_str());
                pos += 1;
                values.push(content_type as &(dyn ToSql + Sync));
            }
            q.push_str(") ")
        }
    }
    match extension_filter {
        Some(ExtensionFilterType::Document) => {
            q.push_str(format!(" inner join documents d on ({}.id = d.metadata_id and {}.version = d.version) ", alias, alias).as_str());
        }
        Some(ExtensionFilterType::DocumentTemplate) => {
            q.push_str(format!(" inner join document_templates d on ({}.id = d.metadata_id and {}.version = d.version) ", alias, alias).as_str());
        }
        Some(ExtensionFilterType::Guide) => {
            q.push_str(format!(" inner join guides g on ({}.id = g.metadata_id and {}.version = g.version) ", alias, alias).as_str());
        }
        Some(ExtensionFilterType::GuideTemplate) => {
            q.push_str(format!(" inner join guide_templates g on ({}.id = g.metadata_id and {}.version = g.version) ", alias, alias).as_str());
        }
        _ => {}
    }
    q.push_str(format!(" order by lower({}.name) asc ", alias).as_str()); // TODO: when adding MetadataIndex & CollectionIndex, make this configurable so it is based on an index
    q.push_str(format!(" offset ${}", pos).as_str());
    pos += 1;
    values.push(offset as &(dyn ToSql + Sync));
    q.push_str(format!(" limit ${}", pos).as_str());
    values.push(limit as &(dyn ToSql + Sync));
    (q.to_string(), values)
}
