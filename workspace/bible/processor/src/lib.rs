pub mod bible;
pub mod book;
pub mod context;
pub mod dblmetadata;
pub mod error;
pub mod html_context;
pub mod metadata;
mod singleton;
pub mod string_context;
pub mod usx;

use crate::bible::Bible;
use crate::book::Book;
use crate::context::UsxContext;
use crate::error::Error;
use crate::metadata::{ManifestName, Metadata, MetadataPublicationContent};
use quick_xml::de::from_reader;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek};
use std::str::from_utf8;
use std::sync::{Arc, Mutex};
use zip::ZipArchive;

pub fn process_path(path: &str) -> Result<Bible, Error> {
    let file = File::open(path)?;
    process_reader(file)
}

pub fn process_reader(reader: impl Read + Seek) -> Result<Bible, Error> {
    let mut zip = ZipArchive::new(reader)?;

    let metadata_file = zip.by_name("metadata.xml")?;
    let buf = BufReader::new(metadata_file);
    let manifest: Metadata = Metadata::new(from_reader(buf)?);

    let mut books = Vec::<Arc<Mutex<Book>>>::new();
    for name in &manifest.names {
        let content = &manifest.publication.contents.get(name.id());
        if content.is_none() {
            continue;
        }
        let content = content.unwrap();
        let mut file = zip.by_name(content.file())?;
        let buf = BufReader::new(&mut file);
        let book = process_book(buf, name, content)?;
        books.push(book);
    }

    Ok(Bible::new(manifest, books))
}

pub fn process_book(
    buf: impl BufRead,
    name: &ManifestName,
    content: &MetadataPublicationContent,
) -> Result<Arc<Mutex<Book>>, Error> {
    let book = Arc::new(Mutex::new(Book::new(name.clone(), content.clone())));
    let mut reader = Reader::from_reader(buf);
    // let cfg = reader.config_mut();
    // cfg.trim_text(true);
    // cfg.expand_empty_elements = true;
    // cfg.trim_markup_names_in_closing_tags = true;
    let mut raw = Vec::new();
    let mut buf = Vec::new();
    let mut context = UsxContext::new(Arc::clone(&book));
    loop {
        let mut is_text = false;
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let tag_name = from_utf8(e.name().as_ref())?.to_string();
                let len = e.len();
                let position = (reader.buffer_position() as usize - len) as i64 - 3;
                let mut attributes = HashMap::new();
                for x in e.attributes() {
                    let r = x?;
                    let key = from_utf8(r.key.as_ref())?.to_string();
                    let value = from_utf8(r.value.as_ref())?.to_string();
                    attributes.insert(key, value);
                }
                context.push(&tag_name, &attributes, position);
            }
            Ok(Event::Empty(e)) => {
                let tag_name = from_utf8(e.name().as_ref())?.to_string();
                let position = (reader.buffer_position() as usize - e.len()) as i64 - 3;
                let end = (reader.buffer_position()) as i64;
                let mut attributes = HashMap::new();
                for x in e.attributes() {
                    let r = x?;
                    let key = from_utf8(r.key.as_ref())?.to_string();
                    let value = from_utf8(r.value.as_ref())?.to_string();
                    attributes.insert(key, value);
                }
                context.push(&tag_name, &attributes, position);
                context.pop(end);
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape()?.into_owned();
                context.add_text(
                    &text,
                    (reader.buffer_position() as usize - text.len()) as i64,
                );
                is_text = true;
            }
            Ok(Event::End(_)) => {
                context.pop(reader.buffer_position() as i64);
            }
            Ok(Event::Eof) => {
                let mut book_mut = book.lock().unwrap();
                book_mut.raw = from_utf8(raw.as_slice())?.to_string();
                return Ok(Arc::clone(&book));
            }
            Err(e) => {
                return Err(Error::new(format!(
                    "Error at position {}: {:?}",
                    reader.error_position(),
                    e
                )))
            }
            _ => (),
        }
        if !is_text {
            raw.push(u8::try_from('<').unwrap());
        }
        raw.extend_from_slice(buf.to_vec().as_slice());
        if !is_text {
            raw.push(u8::try_from('>').unwrap());
        }
        buf.clear();
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::sync::Arc;
    use crate::html_context::HtmlContext;
    use crate::process_path;
    use crate::usx::item::{IUsxItem, UsxItem};

    #[test]
    fn test_process_html() {
        let bible = process_path("testdata/asv.zip").unwrap();
        let book = bible.books_by_usfm.get("GEN").unwrap();
        let book = book.lock().unwrap();
        let chapter = book.chapters_by_usfm.get("GEN.1").unwrap();
        let mut ctx = HtmlContext::new(true, false, false, true);
        let html = chapter.lock().unwrap().to_html(&mut ctx);
        println!("{html}");
    }

    #[test]
    fn test_process_chapter_raw() {
        let bible = process_path("testdata/asv.zip").unwrap();
        let book = bible.books_by_usfm.get("GEN").unwrap();
        let book = book.lock().unwrap();
        let chapter = book.chapters_by_usfm.get("GEN.1").unwrap();
        let position = chapter.lock().unwrap().position().unwrap();
        let raw = book.get_raw_content(&position);
        println!("`{}`", raw);
    }

    #[test]
    fn test_process_text() {
        let bible = process_path("testdata/asv.zip").unwrap();
        let book = bible.books_by_usfm.get("GEN").unwrap();
        let book = book.lock().unwrap();
        let chapter = book.chapters_by_usfm.get("GEN.1").unwrap();
        let text = chapter.lock().unwrap().to_string(&None);
        println!("`{}`", text.trim());
    }

    #[test]
    fn test_get_verses() {
        let bible = process_path("testdata/asv.zip").unwrap();
        let book = bible.books_by_usfm.get("2PE").unwrap();
        let chapter = {
            let book = book.lock().unwrap();
            let chapter = book.chapters_by_usfm.get("2PE.3").unwrap();
            Arc::clone(chapter)
        };
        let verses = match chapter.lock().unwrap().deref() {
            UsxItem::Chapter(c) => c.get_verses(Arc::clone(book)),
            _ => Vec::new(),
        };
        println!("{}", verses.len());
    }
}
