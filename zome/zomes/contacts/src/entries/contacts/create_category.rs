use hdk::prelude::*;
use holo_hash::EntryHashB64;

use super::{Category, CategoryWithId, EntryTypes};
use crate::utils::error;

pub fn create_category_handler(name: String) -> ExternResult<CategoryWithId> {
    let category = Category::new(name.clone());
    // create_entry(&category)?;
    match create_entry(&EntryTypes::Category(category.clone())) {
        Ok(_) => {
            let id: EntryHashB64 = hash_entry(category.clone())?.into();
            let category_with_id = CategoryWithId { id, name };
            Ok(category_with_id)
        },
        Err(_) => error("problems were encountered during creation of entry"),
    }
    
}
