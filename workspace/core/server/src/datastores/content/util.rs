use postgres_types::ToSql;
use uuid::Uuid;
use crate::models::content::attribute_type::AttributeType;
use crate::models::content::find_query::{ExtensionFilterType, FindQueryInput};
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
        buf.push('(');
        buf.push_str(attributes_column);
        for _ in attr.path.iter() {
            let name = names.get(n).unwrap();
            n += 1;
            values.push(name as &(dyn ToSql + Sync));
            buf.push_str(format!("->>${}", index).as_str());
            index += 1;
        }
        buf.push_str(")::");
        match attr.attribute_type {
            AttributeType::String => buf.push_str("varchar"),
            AttributeType::Int => buf.push_str("bigint"),
            AttributeType::Float => buf.push_str("double precision"),
            AttributeType::Date => buf.push_str("int"),
            AttributeType::DateTime => buf.push_str("bigint"),
            AttributeType::Profile => buf.push_str("uuid"),
            AttributeType::Metadata => buf.push_str("uuid"),
            AttributeType::Collection => buf.push_str("uuid"),
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

#[allow(clippy::too_many_arguments)]
pub fn build_find_args<'a>(
    base_type: &str,
    query: &str,
    alias: &str,
    find_query: &'a FindQueryInput,
    category_ids: &'a Option<Vec<Uuid>>,
    count: bool
) -> (String, Vec<&'a (dyn ToSql + Sync)>) {
    let mut q = query.to_string();
    let mut values = Vec::new();
    let mut pos = 1;

    if let Some(category_ids) = category_ids {
        if !category_ids.is_empty() {
            for category_id in category_ids {
                q.push_str(format!(" inner join {}_categories as cid on (cid.{}_id = {}.id and cid.category_id = ${}) ", base_type, base_type, alias, pos).as_str());
                pos += 1;
                values.push(category_id as &(dyn ToSql + Sync));
            }
        }
    }

    match find_query.extension_filter {
        Some(ExtensionFilterType::Document) => {
            q.push_str(format!(" inner join documents d on ({}.id = d.metadata_id and {}.version = d.version) ", alias, alias).as_str());
        }
        Some(ExtensionFilterType::DocumentTemplate) => {
            q.push_str(format!(" inner join document_templates dt on ({}.id = dt.metadata_id and {}.version = dt.version) ", alias, alias).as_str());
        }
        Some(ExtensionFilterType::Guide) => {
            q.push_str(format!(" inner join guides g on ({}.id = g.metadata_id and {}.version = g.version) ", alias, alias).as_str());
        }
        Some(ExtensionFilterType::GuideTemplate) => {
            q.push_str(format!(" inner join guide_templates gt on ({}.id = gt.metadata_id and {}.version = gt.version) ", alias, alias).as_str());
        }
        Some(ExtensionFilterType::CollectionTemplate) => {
            q.push_str(format!(" inner join collection_templates ct on ({}.id = ct.metadata_id and {}.version = ct.version) ", alias, alias).as_str());
        }
        _ => {}
    }

    q.push_str(format!(" where {}.deleted = false ", alias).as_str());

    if base_type == "collection" {
        if let Some(collection_type) = &find_query.collection_type {
            q.push_str(format!(" and {}.type = ${} ", alias, pos).as_str());
            pos += 1;
            values.push(collection_type as &(dyn ToSql + Sync));
        }
    }

    if !find_query.attributes.is_empty() && find_query.attributes.iter().any(|a| !a.attributes.is_empty()) {
        q.push_str(" and ");
        for i in 0..find_query.attributes.len() {
            let attrs = find_query.attributes.get(i).unwrap();
            if attrs.attributes.is_empty() {
                continue;
            }
            if i > 0 {
                q.push_str(" or ");
            }
            q.push_str(" ( ");
            for j in 0..attrs.attributes.len() {
                if j > 0 {
                    q.push_str(" and ");
                }
                let attr = attrs.attributes.get(j).unwrap();
                q.push_str(format!(" {}.attributes->>(${}::varchar) = ${}::varchar ", alias, pos, pos + 1).as_str());
                pos += 2;
                values.push(&attr.key as &(dyn ToSql + Sync));
                values.push(&attr.value as &(dyn ToSql + Sync));
            }
            q.push_str(" ) ");
        }
    }

    if let Some(content_types) = &find_query.content_types {
        if !content_types.is_empty() {
            q.push_str(format!(" and {}.content_type in (", alias).as_str());
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

    if !count {
        q.push_str(format!(" order by lower({}.name) asc ", alias).as_str()); // TODO: when adding MetadataIndex & CollectionIndex, make this configurable so it is based on an index
        if find_query.offset.is_some() {
            q.push_str(format!(" offset ${}", pos).as_str());
            values.push(find_query.offset.as_ref().unwrap() as &(dyn ToSql + Sync));
            pos += 1;
        }
        if find_query.limit.is_some() {
            q.push_str(format!(" limit ${}", pos).as_str());
            values.push(find_query.limit.as_ref().unwrap() as &(dyn ToSql + Sync));
        }
    }
    (q.to_string(), values)
}
