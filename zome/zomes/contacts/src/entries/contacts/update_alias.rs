use hdk::prelude::*;

use super::{in_contacts::in_contacts_handler, Alias, AliasIO, EntryTypes};
use crate::utils::error;

pub fn update_alias_handler(io: AliasIO) -> ExternResult<AliasIO> {
    let agent_pubkey = io.id.clone();
    let added = in_contacts_handler(agent_pubkey)?;
    if let true = added {
        let alias = Alias {
            id: io.id.clone().into(),
            first_name: io.first_name.clone(),
            last_name: io.last_name.clone(),
            created: sys_time()?,
        };
        // create_entry(alias.clone())?;
        // return Ok(io);
        match create_entry(&EntryTypes::Alias(alias.clone())) {
            Ok(_) => return Ok(io),
            Err(_) => return error("problems were encountered during creation of entry"),
        }
    } else {
        return error("the agent is not added");
    };
}
