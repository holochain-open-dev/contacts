use super::{Category, CategoryWithId};
use hdk::prelude::*;
use holo_hash::EntryHashB64;

pub fn create_category_handler(name: String) -> ExternResult<CategoryWithId> {
    let category = Category::new(name.clone());
    create_entry(&category)?;
    let id: EntryHashB64 = hash_entry(category.clone())?.into();
    let category_with_id = CategoryWithId { id, name };
    Ok(category_with_id)
}
