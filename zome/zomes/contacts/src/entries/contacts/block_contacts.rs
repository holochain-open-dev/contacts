use hdk::prelude::*;

use super::helpers::check_latest_state;
use crate::utils::error;

use super::{Contact, ContactType};

pub fn block_contacts_handler(agent_ids: Vec<AgentPubKey>) -> ExternResult<Vec<AgentPubKey>> {
    let me = agent_info()?.agent_latest_pubkey;
    // return err right away if trying to block oneself
    if let true = agent_ids.contains(&me) {
        return error("cannot block yourself");
    }

    check_latest_state(&agent_ids, ContactType::Block)?;
    let blocked_contact = Contact::new(sys_time()?, agent_ids.clone(), ContactType::Block);
    create_entry(&blocked_contact)?;
    Ok(agent_ids)
}
