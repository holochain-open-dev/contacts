use crate::utils::error;
use hdk::prelude::*;
use holo_hash::AgentPubKeyB64;
use std::collections::{hash_map, HashMap};

use super::{
    list_alias::list_alias_handler, AgentPubKey, AliasIO, Contact, ContactOutput, ContactType,
};

pub fn check_latest_state(
    agent_pubkeys: &Vec<AgentPubKey>,
    check_for: ContactType,
) -> ExternResult<()> {
    let mut agents_to_contact_type: HashMap<AgentPubKey, Option<Contact>> =
        std::collections::HashMap::new();
    let sorted_contacts: Vec<Contact> = query_contacts()?;

    for agent in agent_pubkeys {
        let maybe_contacts = sorted_contacts
            .clone()
            .into_iter()
            .filter_map(|c| {
                if c.agent_ids.contains(&agent) {
                    Some(c)
                } else {
                    None
                }
            })
            .collect::<Vec<Contact>>();

        // can return index 0 as query() is ordered from latest to oldest in query_contacts().
        // max_by_key() could be used which is more accurate but also expensive
        // NOTE: can break with breaking change on query

        if maybe_contacts.get(0).is_some() {
            let contact: Contact = maybe_contacts.get(0).unwrap().clone();
            agents_to_contact_type.insert(agent.to_owned(), Some(contact));
        } else {
            agents_to_contact_type.insert(agent.to_owned(), None);
        }
    }

    match check_for {
        ContactType::Add => {
            for agent_contact in agents_to_contact_type {
                if let Some(contact) = agent_contact.1 {
                    match contact.contact_type {
                        ContactType::Add => return error("agent already added"),
                        ContactType::Block => return error("agent is blocked"),
                        _ => (),
                    }
                }
            }
        }
        ContactType::Remove => {
            for agent_contact in agents_to_contact_type {
                if let Some(contact) = agent_contact.1 {
                    match contact.contact_type {
                        ContactType::Remove | ContactType::Unblock => {
                            return error("agent is not added")
                        }
                        ContactType::Block => return error("agent is blocked"),
                        _ => (),
                    }
                } else {
                    return error("agent is not added");
                }
            }
        }
        ContactType::Block => {
            for agent_contact in agents_to_contact_type {
                if let Some(contact) = agent_contact.1 {
                    if let ContactType::Block = contact.contact_type {
                        return error("this agent is already blocked");
                    }
                }
            }
        }
        ContactType::Unblock => {
            for agent_contact in agents_to_contact_type {
                if let Some(contact) = agent_contact.1 {
                    match contact.contact_type {
                        ContactType::Block => (),
                        _ => return error("agent is not blocked"),
                    }
                } else {
                    return error("agent is not blocked");
                }
            }
        }
        ContactType::AddToCategory => {
            for agent_contact in agents_to_contact_type {
                if let Some(contact) = agent_contact.1 {
                    match contact.contact_type {
                        ContactType::Remove | ContactType::Block | ContactType::Unblock => {
                            return error("agent is not added")
                        }
                        _ => (),
                    }
                } else {
                    return error("agent is not added");
                }
            }
        }
        ContactType::RemoveFromCategory => {
            for agent_contact in agents_to_contact_type {
                if let Some(contact) = agent_contact.1 {
                    match contact.contact_type {
                        ContactType::Remove | ContactType::Block | ContactType::Unblock => {
                            return error("agent is not added")
                        }
                        ContactType::RemoveFromCategory => {
                            return error("agent does not have a cateogry")
                        }
                        _ => (),
                    }
                } else {
                    return error("agent is not added");
                }
            }
        }
    }

    Ok(())
}

pub fn query_contacts() -> ExternResult<Vec<Contact>> {
    let filter = QueryFilter::new()
        .entry_type(EntryType::App(AppEntryType::new(
            EntryDefIndex::from(0),
            // zome_info()?.id,
            EntryVisibility::Private,
        )))
        .include_entries(true)
        .action_type(ActionType::Create);

    let mut contacts: Vec<Contact> = query(filter)?
        .into_iter()
        .filter_map(|e| {
            if let Ok(Some(contact)) = e.into_inner().1.to_app_option::<Contact>() {
                return Some(contact);
            } else {
                None
            }
        })
        .collect::<Vec<Contact>>();
    contacts.reverse();
    Ok(contacts)
}

pub fn list_added_or_blocked(filter: ContactType) -> ExternResult<Vec<ContactOutput>> {
    let mut agents_to_contact_types: HashMap<AgentPubKey, Vec<Contact>> =
        std::collections::HashMap::new();
    let sorted_contacts: Vec<Contact> = query_contacts()?;

    for contact in sorted_contacts {
        for agent_id in &contact.agent_ids {
            let maybe_agent_contact = agents_to_contact_types.entry(agent_id.to_owned());
            match maybe_agent_contact {
                hash_map::Entry::Occupied(o) => {
                    let contact_types: &mut Vec<Contact> = o.into_mut();
                    contact_types.push(contact.clone());
                }
                hash_map::Entry::Vacant(v) => {
                    let mut new_contact_types: Vec<Contact> = Vec::new();
                    new_contact_types.insert(0, contact.clone());
                    v.insert(new_contact_types);
                }
            }
        }
    }
    let alias_per_agent: HashMap<String, Option<AliasIO>> = list_alias_handler()?;

    let filtered_agents = agents_to_contact_types
        .into_iter()
        .filter_map(|agent_contact_types| {
            let contacts = agent_contact_types.1.to_owned();
            let latest_status = contacts.into_iter().max_by_key(|c| c.created);
            if let Some(c) = latest_status {
                let contact_output: Option<ContactOutput>;
                if ContactType::Add == filter
                    && (ContactType::Add == c.contact_type
                        || ContactType::AddToCategory == c.contact_type
                        || ContactType::RemoveFromCategory == c.contact_type)
                {
                    let mut _alias_per_agent = alias_per_agent.clone();
                    let maybe_alias =
                        _alias_per_agent.entry(agent_contact_types.0.to_owned().to_string());
                    match maybe_alias {
                        hash_map::Entry::Occupied(o) => {
                            let maybe_alias: &mut Option<AliasIO> = o.into_mut();
                            if let Some(alias) = maybe_alias {
                                contact_output = Some(ContactOutput {
                                    id: agent_contact_types.0.to_owned().into(),
                                    first_name: alias.first_name.clone(),
                                    last_name: alias.last_name.clone(),
                                    category: c.category,
                                })
                            } else {
                                contact_output = Some(ContactOutput {
                                    id: agent_contact_types.0.to_owned().into(),
                                    first_name: None,
                                    last_name: None,
                                    category: c.category,
                                })
                            }
                        }
                        hash_map::Entry::Vacant(_) => {
                            contact_output = Some(ContactOutput {
                                id: agent_contact_types.0.to_owned().into(),
                                first_name: None,
                                last_name: None,
                                category: c.category,
                            })
                        }
                    }
                    return Some(contact_output);
                } else if ContactType::Block == filter && ContactType::Block == c.contact_type {
                    contact_output = Some(ContactOutput {
                        id: agent_contact_types.0.into(),
                        first_name: None,
                        last_name: None,
                        category: c.category,
                    });
                    return Some(contact_output);
                } else {
                    None
                }
            } else {
                None
            }
        })
        .flatten()
        .collect();

    Ok(filtered_agents)
}

pub fn b64_to_agent_pk(keys: Vec<AgentPubKeyB64>) -> Vec<AgentPubKey> {
    keys.into_iter()
        .map(|key| {
            let b64: AgentPubKey = key.into();
            return b64;
        })
        .collect()
}
