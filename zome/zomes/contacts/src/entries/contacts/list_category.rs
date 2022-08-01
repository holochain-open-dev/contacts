use hdk::prelude::*;
use holo_hash::EntryHashB64;

use super::{Category, CategoryWithId};

pub fn list_category_handler() -> ExternResult<Vec<CategoryWithId>> {
    let filter = QueryFilter::new()
        .entry_type(EntryType::App(AppEntryType::new(
            EntryDefIndex::from(1),
            // zome_info()?.id,
            EntryVisibility::Private,
        )))
        .include_entries(true)
        .action_type(ActionType::Create);

    let categories = query(filter)?
        .into_iter()
        .filter_map(|e| {
            if let Ok(Some(category)) = e.clone().into_inner().1.to_app_option::<Category>() {
                // we can unwrap here as the assumption is that entry hash always exists
                let entry_hash_b64: EntryHashB64 =
                    e.action().entry_hash().unwrap().to_owned().into();
                let category_with_id = CategoryWithId {
                    id: entry_hash_b64,
                    name: category.name,
                };
                return Some(category_with_id);
            } else {
                None
            }
        })
        .collect::<Vec<CategoryWithId>>();
    Ok(categories)
}
