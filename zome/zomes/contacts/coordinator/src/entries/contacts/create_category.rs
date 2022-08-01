use hdk::prelude::*;

use contacts_integrity::*;
use contacts_integrity_types::*;

use crate::utils::error;

pub fn create_category_handler(name: String) -> ExternResult<CategoryWithId> {
    let category = Category::new(name.clone());
    // create_entry(&category)?;
    match create_entry(&EntryTypes::Category(category.clone())) {
        Ok(_) => {
            let id: EntryHash = hash_entry(category.clone())?.into();
            let category_with_id = CategoryWithId { id, name };
            Ok(category_with_id)
        },
        Err(_) => error("problems were encountered during creation of entry"),
    }
    
}
