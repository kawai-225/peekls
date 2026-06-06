use std::path::Path;

use lopdf::{Document, Object};

pub fn read_title(path: &Path) -> Option<String> {
    let document = Document::load(path).ok()?;
    let info = document.trailer.get(b"Info").ok()?;
    let object = resolve_object(&document, info)?;
    let dictionary = object.as_dict().ok()?;
    let title = dictionary.get(b"Title").ok()?;

    object_to_string(title)
}

fn resolve_object<'a>(document: &'a Document, object: &'a Object) -> Option<&'a Object> {
    match object {
        Object::Reference(id) => document.get_object(*id).ok(),
        other => Some(other),
    }
}

fn object_to_string(object: &Object) -> Option<String> {
    match object {
        Object::String(bytes, _) => clean_string(bytes),
        Object::Name(bytes) => clean_string(bytes),
        _ => None,
    }
}

fn clean_string(bytes: &[u8]) -> Option<String> {
    let title = String::from_utf8_lossy(bytes).trim().to_string();

    if title.is_empty() { None } else { Some(title) }
}
