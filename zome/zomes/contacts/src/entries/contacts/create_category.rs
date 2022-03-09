use super::{Category, CategoryWithId};
use hdk::prelude::*;

pub fn create_category_handler(name: String) -> ExternResult<CategoryWithId> {
    let category = Category::new(name.clone());
    create_entry(&category)?;
    let id = hash_entry(category.clone())?;
    let category_with_id = CategoryWithId { id, name };
    Ok(category_with_id)
}
