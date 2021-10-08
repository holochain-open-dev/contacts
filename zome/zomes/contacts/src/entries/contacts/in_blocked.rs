use hdk::prelude::*;

use super::list_blocked::list_blocked_handler;

use super::AgentPubKey;

pub fn in_blocked_handler(agent_pubkey: AgentPubKey) -> ExternResult<bool> {
    let blocked_list = list_blocked_handler()?;
    if blocked_list.len() == 0 {
        Ok(false)
    } else {
        if blocked_list.iter().any(|pubkey| pubkey == &agent_pubkey) {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
