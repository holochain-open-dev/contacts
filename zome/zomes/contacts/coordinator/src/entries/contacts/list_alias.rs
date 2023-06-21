use hdk::prelude::*;
use std::collections::{hash_map, HashMap};

use contacts_integrity_types::*;
use contacts_coordinator_types::*;

pub fn list_alias_handler() -> ExternResult<HashMap<String, Option<AliasIO>>> {
    let mut agents_to_aliases: HashMap<AgentPubKey, Vec<Alias>> = std::collections::HashMap::new();
    let mut agents_to_maybe_alias: HashMap<String, Option<AliasIO>> =
        std::collections::HashMap::new();
    let filter = QueryFilter::new()
        .entry_type(EntryType::App(AppEntryType::new(
            EntryDefIndex::from(2),
            zome_info()?.id,
            EntryVisibility::Private,
        )))
        .include_entries(true)
        .action_type(ActionType::Create);

    query(filter)?.into_iter().for_each(|e| {
        if let Ok(Some(alias)) = e.into_inner().1.to_app_option::<Alias>() {
            let id = alias.id.clone();
            let maybe_agent_contact = agents_to_aliases.entry(id.to_owned());
            match maybe_agent_contact {
                hash_map::Entry::Occupied(o) => {
                    let aliases: &mut Vec<Alias> = o.into_mut();
                    aliases.push(alias.clone());
                }
                hash_map::Entry::Vacant(v) => {
                    let mut new_aliases: Vec<Alias> = Vec::new();
                    new_aliases.insert(0, alias.clone());
                    v.insert(new_aliases);
                }
            }
        }
    });

    agents_to_aliases.into_iter().for_each(|agent_to_alias| {
        let latest_alias = agent_to_alias.1.into_iter().max_by_key(|a| a.created);
        if let Some(alias) = latest_alias {
            let alias_output = AliasIO {
                id: alias.id.into(),
                first_name: alias.first_name,
                last_name: alias.last_name,
            };
            agents_to_maybe_alias.insert(agent_to_alias.0.to_string(), Some(alias_output));
        } else {
            agents_to_maybe_alias.insert(agent_to_alias.0.to_string(), None);
        };
    });

    Ok(agents_to_maybe_alias)
}
