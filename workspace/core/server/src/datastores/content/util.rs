use crate::models::content::attribute_location::AttributeLocation;
use crate::models::content::attribute_type::AttributeType;
use crate::models::content::find_query::{ExtensionFilterType, FindQueryInput};
use crate::models::content::ordering::Order::Ascending;
use crate::models::content::ordering::Ordering;
use postgres_types::ToSql;
use serde_json::json;
use uuid::Uuid;

pub fn build_ordering_names(ordering: &[Ordering], names: &mut Vec<String>) {
    for attr in ordering {
        if let Some(path) = &attr.path {
            for p in path.iter() {
                names.push(p.clone());
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn build_ordering<'a>(
    table_alias: &str,
    collection_item_attributes_column: &str,
    metadata_item_attributes_column: &str,
    relationship_attributes_column: &str,
    start_index: i32,
    ordering: &[Ordering],
    values: &mut Vec<&'a (dyn ToSql + Sync)>,
    names: &'a [String],
) -> (String, i32) {
    let mut index = start_index;
    let mut buf = "order by ".to_owned();
    let mut n = 0;
    for (i, attr) in ordering.iter().enumerate() {
        let field = attr.get_field();
        if attr.path.is_none() && field.is_none() {
            continue;
        }
        if i > 0 && !buf.ends_with(", ") {
            buf.push_str(", ");
        }
        let mut has_ordering = false;
        if let Some(path) = &attr.path {
            if !path.is_empty() {
                buf.push('(');
                if attr
                    .attribute_location
                    .unwrap_or(AttributeLocation::Relationship)
                    == AttributeLocation::Relationship
                {
                    buf.push_str(relationship_attributes_column);
                } else if !collection_item_attributes_column.is_empty()
                    && !metadata_item_attributes_column.is_empty()
                {
                    buf.push_str(format!("(case when {collection_item_attributes_column} is null then {metadata_item_attributes_column} else {collection_item_attributes_column} end)").as_str());
                } else if !collection_item_attributes_column.is_empty() {
                    buf.push_str(collection_item_attributes_column);
                } else if !metadata_item_attributes_column.is_empty() {
                    buf.push_str(metadata_item_attributes_column);
                }
                for _ in path.iter() {
                    let name = names.get(n).unwrap();
                    n += 1;
                    values.push(name as &(dyn ToSql + Sync));
                    buf.push_str(format!("->>${index}").as_str());
                    index += 1;
                }
                buf.push_str(")::");
                match attr.attribute_type.unwrap_or(AttributeType::String) {
                    AttributeType::String => buf.push_str("varchar"),
                    AttributeType::Int => buf.push_str("bigint"),
                    AttributeType::Float => buf.push_str("double precision"),
                    AttributeType::Date => buf.push_str("int"),
                    AttributeType::DateTime => buf.push_str("bigint"),
                    AttributeType::Profile => buf.push_str("uuid"),
                    AttributeType::Metadata => buf.push_str("uuid"),
                    AttributeType::Collection => buf.push_str("uuid"),
                }
                has_ordering = true;
            } else if let Some(field) = field {
                buf.push_str(table_alias);
                buf.push('.');
                buf.push_str(field);
                has_ordering = true;
            }
        } else if let Some(field) = field {
            buf.push_str(table_alias);
            buf.push('.');
            buf.push_str(field);
            has_ordering = true;
        }
        if has_ordering {
            buf.push(' ');
            buf.push_str(if attr.order == Ascending {
                "asc"
            } else {
                "desc"
            });
        }
    }
    if buf == "order by " {
        return ("".to_owned(), index);
    }
    (buf, index)
}

#[allow(clippy::too_many_arguments)]
pub fn build_find_args<'a>(
    base_type: &str,
    query: &str,
    table_alias: &str,
    item_attributes_column: &str,
    relationship_attributes_column: &str,
    find_query: &'a FindQueryInput,
    category_ids: &'a Option<Vec<Uuid>>,
    trait_ids: &'a Option<Vec<String>>,
    count: bool,
    names: &'a mut Vec<String>,
) -> (String, Vec<&'a (dyn ToSql + Sync)>) {
    let mut q = query.to_string();
    let mut values = Vec::new();
    let mut pos = 1;

    if let Some(category_ids) = category_ids {
        if !category_ids.is_empty() {
            for category_id in category_ids {
                q.push_str(format!(" inner join {base_type}_categories as cid on (cid.{base_type}_id = {table_alias}.id and cid.category_id = ${pos}) ").as_str());
                pos += 1;
                values.push(category_id as &(dyn ToSql + Sync));
            }
        }
    }

    if let Some(trait_ids) = trait_ids {
        if !trait_ids.is_empty() {
            for trait_id in trait_ids {
                q.push_str(format!(" inner join {base_type}_traits as tid on (tid.{base_type}_id = {table_alias}.id and tid.trait_id = ${pos}) ").as_str());
                pos += 1;
                values.push(trait_id as &(dyn ToSql + Sync));
            }
        }
    }

    match find_query.extension_filter {
        Some(ExtensionFilterType::Document) => {
            q.push_str(format!(" inner join documents d on ({table_alias}.id = d.metadata_id and {table_alias}.version = d.version) ").as_str());
        }
        Some(ExtensionFilterType::DocumentTemplate) => {
            q.push_str(format!(" inner join document_templates dt on ({table_alias}.id = dt.metadata_id and {table_alias}.version = dt.version) ").as_str());
        }
        Some(ExtensionFilterType::Guide) => {
            q.push_str(
                format!(
                    " inner join guides g on ({table_alias}.id = g.metadata_id and {table_alias}.version = g.version) "
                )
                .as_str(),
            );
        }
        Some(ExtensionFilterType::GuideTemplate) => {
            q.push_str(format!(" inner join guide_templates gt on ({table_alias}.id = gt.metadata_id and {table_alias}.version = gt.version) ").as_str());
        }
        Some(ExtensionFilterType::CollectionTemplate) => {
            q.push_str(format!(" inner join collection_templates ct on ({table_alias}.id = ct.metadata_id and {table_alias}.version = ct.version) ").as_str());
        }
        _ => {}
    }

    q.push_str(format!(" where {table_alias}.deleted = false ").as_str());

    if base_type == "collection" {
        if let Some(collection_type) = &find_query.collection_type {
            q.push_str(format!(" and {table_alias}.type = ${pos} ").as_str());
            pos += 1;
            values.push(collection_type as &(dyn ToSql + Sync));
        }
    }

    if !find_query.attributes.is_empty()
        && find_query
            .attributes
            .iter()
            .any(|a| !a.attributes.is_empty())
    {
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
                q.push_str(
                    format!(
                        " {}.{}->>(${}::varchar) = ${}::varchar ",
                        table_alias,
                        item_attributes_column,
                        pos,
                        pos + 1
                    )
                    .as_str(),
                );
                pos += 2;
                values.push(&attr.key as &(dyn ToSql + Sync));
                values.push(&attr.value as &(dyn ToSql + Sync));
            }
            q.push_str(" ) ");
        }
    }

    if let Some(content_types) = &find_query.content_types {
        if !content_types.is_empty() {
            q.push_str(format!(" and {table_alias}.content_type in (").as_str());
            for (ix, content_type) in content_types.iter().enumerate() {
                if ix > 0 {
                    q.push_str(", ");
                }
                q.push_str(format!("${pos}").as_str());
                pos += 1;
                values.push(content_type as &(dyn ToSql + Sync));
            }
            q.push_str(") ")
        }
    }

    if !count {
        if let Some(ordering) = &find_query.ordering {
            let js = json!(ordering);
            let ordering: Vec<Ordering> = serde_json::from_value(js).unwrap();
            build_ordering_names(&ordering, names);
            let (ordering_sql, index) = build_ordering(
                table_alias,
                item_attributes_column,
                "",
                relationship_attributes_column,
                pos,
                &ordering,
                &mut values,
                names,
            );
            pos = index;
            if !ordering_sql.is_empty() {
                q.push_str(ordering_sql.as_str());
            }
        } else {
            q.push_str(format!(" order by lower({table_alias}.name) asc ").as_str());
            // TODO: when adding MetadataIndex & CollectionIndex, make this configurable so it is based on an index
        }
        if find_query.offset.is_some() {
            q.push_str(format!(" offset ${pos}").as_str());
            values.push(find_query.offset.as_ref().unwrap() as &(dyn ToSql + Sync));
            pos += 1;
        }
        if find_query.limit.is_some() {
            q.push_str(format!(" limit ${pos}").as_str());
            values.push(find_query.limit.as_ref().unwrap() as &(dyn ToSql + Sync));
        }
    }
    (q.to_string(), values)
}
