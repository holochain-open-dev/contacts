use hdk::prelude::*;

use super::list_added::list_added_handler;

use super::AgentPubKey;

pub fn in_contacts_handler(agent_pubkey: AgentPubKey) -> ExternResult<bool> {
    let contacts_list = list_added_handler()?;
    if contacts_list.len() == 0 {
        Ok(false)
    } else {
        if contacts_list
            .iter()
            .any(|contact| contact.id == agent_pubkey)
        {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
