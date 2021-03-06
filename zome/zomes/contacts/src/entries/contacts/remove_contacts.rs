use hdk::prelude::*;
use holo_hash::AgentPubKeyB64;

use super::helpers::{b64_to_agent_pk, check_latest_state};

use super::{Contact, ContactType};

pub fn remove_contacts_handler(
    agent_ids: Vec<AgentPubKeyB64>,
) -> ExternResult<Vec<AgentPubKeyB64>> {
    let agent_ids_raw = b64_to_agent_pk(agent_ids.clone());
    check_latest_state(&agent_ids_raw, ContactType::Remove)?;

    let removed_contact = Contact::new(sys_time()?, agent_ids_raw, ContactType::Remove, None);
    create_entry(&removed_contact)?;
    Ok(agent_ids)
}
