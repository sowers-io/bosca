use crate::context::BoscaContext;
use crate::models::bible::bible::Bible;
use crate::models::bible::book::Book;
use crate::models::bible::reference::Reference;
use async_graphql::Error;
use std::collections::HashMap;

pub async fn parse(
    ctx: &BoscaContext,
    bible: &Bible,
    human: &str,
) -> Result<Vec<Reference>, Error> {
    let parts: Vec<&str> = human.split(',').collect();
    let mut references = Vec::new();

    for part in parts {
        if let Some(reference) = parse_single(ctx, bible, part.trim()).await? {
            references.push(reference);
        }
    }

    let mut joined_references: HashMap<String, String> = HashMap::new();
    for reference in &references {
        if let Some(chapter) = reference.chapter_usfm() {
            let usfms = joined_references.entry(chapter).or_default();

            if !usfms.is_empty() {
                usfms.push('+');
            }
            usfms.push_str(reference.usfm());
        } else if let Some(book) = reference.book_usfm() {
            let usfms = joined_references.entry(book).or_default();
            if usfms.is_empty() {
                usfms.push_str(reference.usfm());
            }
        }
    }

    Ok(joined_references
        .values()
        .map(|usfm| Reference::new(usfm.clone()))
        .collect())
}

async fn parse_single(
    ctx: &BoscaContext,
    bible: &Bible,
    human: &str,
) -> Result<Option<Reference>, Error> {
    let human_lower = human.to_lowercase();
    let mut book: Option<Book> = None;
    let mut non_book = None;

    let books = ctx
        .content
        .bibles
        .get_books(&bible.metadata_id, bible.version, &bible.variant)
        .await?;

    for b in books.into_iter() {
        if human_lower.starts_with(&b.name_long.to_lowercase()) {
            non_book = Some(human_lower[b.name_long.len()..].trim().to_string());
            book = Some(b);
            break;
        } else if human_lower.starts_with(&b.name_short.to_lowercase()) {
            non_book = Some(human_lower[b.name_short.len()..].trim().to_string());
            book = Some(b);
            break;
        } else if human_lower.starts_with(&b.abbreviation.to_lowercase()) {
            non_book = Some(human_lower[b.abbreviation.len()..].trim().to_string());
            book = Some(b);
            break;
        }
    }

    if book.is_none() || non_book.is_none() {
        return Ok(None);
    }

    let book = book.unwrap();
    let non_book = non_book.unwrap();
    if non_book.is_empty() {
        return Ok(Some(book.reference.clone()));
    }

    let mut number_parts: Vec<String> = non_book.split(':').map(|s| s.to_string()).collect();
    let chapters = ctx
        .content
        .bibles
        .get_chapters(&bible.metadata_id, bible.version, &bible.variant, book.reference.usfm())
        .await?;

    if let Some(chapter) = chapters.into_iter().find(|c| {
        c.reference
            .chapter()
            .unwrap_or_default()
            .eq_ignore_ascii_case(&number_parts[0])
    }) {
        if number_parts.len() == 1 {
            return Ok(Some(chapter.reference.clone()));
        }

        if number_parts[1].contains('–') {
            number_parts[1] = number_parts[1].replace('–', "-");
        }

        if number_parts[1].contains('-') {
            let range_parts: Vec<&str> = number_parts[1].split('-').collect();
            if range_parts.len() == 2 {
                let start: usize = range_parts[0].parse()?;
                let end: usize = range_parts[1].parse()?;
                let usfms: Vec<String> = (start..=end)
                    .map(|i| format!("{}.{}", chapter.reference.usfm(), i))
                    .collect();
                return Ok(Some(Reference::new(usfms.join("+"))));
            } else {
                number_parts[1] = range_parts[0].to_string();
            }
        }

        return Ok(Some(Reference::new(format!(
            "{}.{}",
            chapter.reference.usfm(),
            number_parts[1]
        ))));
    }

    Ok(Some(book.reference.clone()))
}
