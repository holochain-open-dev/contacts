use hdk::prelude::*;

use crate::list_blocked::list_blocked_handler;

pub fn in_blocked_handler(agent_pubkey: AgentPubKey) -> ExternResult<bool> {
    let blocked_list = list_blocked_handler()?;
    if blocked_list.len() == 0 {
        Ok(false)
    } else {
        if blocked_list
            .iter()
            .any(|contact| contact.id == agent_pubkey)
        {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
