use super::list_blocked::list_blocked_handler;
use hdk::prelude::*;
use holo_hash::AgentPubKeyB64;

pub fn in_blocked_handler(agent_pubkey: AgentPubKeyB64) -> ExternResult<bool> {
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
