use hdi::prelude::*;

#[derive(Clone, Deserialize, PartialEq, Serialize, SerializedBytes, Debug)]
pub enum ContactType {
    Add,
    Remove,
    Block,
    Unblock,
    AddToCategory,
    RemoveFromCategory,
}

#[derive(Clone, Deserialize, PartialEq, Serialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryWithId {
    //id: EntryHashB64,
    pub id: EntryHash,
    pub name: String,
}


#[derive(Clone)]
#[hdk_entry_helper]
pub struct Contact {
    pub agent_ids: Vec<AgentPubKey>,
    pub created: Timestamp,
    pub contact_type: ContactType,
    pub category: Option<CategoryWithId>,
}

impl Contact {
    pub fn new(
        timestamp: Timestamp,
        agent_ids: Vec<AgentPubKey>,
        contact_type: ContactType,
        category: Option<CategoryWithId>,
    ) -> Self {
        Contact {
            agent_ids,
            created: timestamp,
            contact_type,
            category,
        }
    }
}

#[derive(Clone)]
#[hdk_entry_helper]
pub struct Category {
    pub name: String,
}

impl Category {
    pub fn new(name: String) -> Self {
        Category { name }
    }
}

#[derive(Clone)]
#[hdk_entry_helper]
pub struct Alias {
    pub id: AgentPubKey,
    pub first_name: Option<String>, // this could be a BtreeMap too if more (customizeable) fields are needed
    pub last_name: Option<String>,
    pub created: Timestamp, // to determine the latest alias set for a particular contact
}