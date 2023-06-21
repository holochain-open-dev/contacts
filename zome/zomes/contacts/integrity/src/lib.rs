use hdi::prelude::*;
use contacts_integrity_types::*;

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_def(required_validations = 5, visibility = "private")]
    Contact(Contact),
    #[entry_def(required_validations = 5, visibility = "private")]
    Category(Category),
    #[entry_def(required_validations = 5, visibility = "private")]
    Alias(Alias),
}