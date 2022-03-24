use hdk::prelude::holo_hash::AgentPubKeyB64;
use hdk::prelude::*;

use super::helpers::{b64_to_agent_pk, check_latest_state};

use super::{Contact, ContactType};

pub fn add_contacts_handler(agent_ids: Vec<AgentPubKeyB64>) -> ExternResult<Vec<AgentPubKeyB64>> {
    let agent_ids_raw: Vec<AgentPubKey> = b64_to_agent_pk(agent_ids.clone());
    check_latest_state(&agent_ids_raw, ContactType::Add)?;
    let added_contact = Contact::new(sys_time()?, agent_ids_raw, ContactType::Add, None);
    create_entry(&added_contact)?;
    Ok(agent_ids)
}
