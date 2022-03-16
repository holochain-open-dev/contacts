use hdk::prelude::*;

use crate::utils::error;

use super::{in_contacts::in_contacts_handler, Alias, AliasInput};

pub fn update_alias_handler(input: AliasInput) -> ExternResult<Alias> {
    let agent_pubkey = input.id.clone();
    let added = in_contacts_handler(agent_pubkey)?;
    if let true = added {
        let alias = Alias {
            id: input.id,
            first_name: input.first_name,
            last_name: input.last_name,
            created: sys_time()?,
        };
        create_entry(alias.clone())?;
        return Ok(alias);
    } else {
        return error("the agent is not added");
    };
}
