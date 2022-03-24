use hdk::prelude::*;
use holo_hash::{AgentPubKeyB64, EntryHashB64};
use timestamp::Timestamp;

pub mod add_contacts;
pub mod add_to_category;
pub mod block_contacts;
pub mod create_category;
pub mod helpers;
pub mod in_blocked;
pub mod in_contacts;
pub mod list_added;
pub mod list_alias;
pub mod list_blocked;
pub mod list_category;
pub mod remove_contacts;
pub mod remove_from_category;
pub mod unblock_contacts;
pub mod update_alias;

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
pub struct CategoryIO {
    id: EntryHashB64,
    name: String,
    agents: Vec<AgentPubKey>,
}

#[derive(Clone, Deserialize, PartialEq, Serialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryWithId {
    id: EntryHashB64,
    name: String,
}

#[derive(Clone, Deserialize, PartialEq, Serialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContactOutput {
    id: AgentPubKeyB64,
    first_name: Option<String>,
    last_name: Option<String>,
    category: Option<CategoryWithId>,
}

#[derive(Deserialize, Serialize, SerializedBytes, Debug, Clone)]
pub struct Contact {
    agent_ids: Vec<AgentPubKey>,
    created: Timestamp,
    contact_type: ContactType,
    category: Option<CategoryWithId>,
}

entry_def!(Contact
    EntryDef {
        id: "contact".into(),
        visibility: EntryVisibility::Private,
        crdt_type: CrdtType,
        required_validations: RequiredValidations::default(),
        required_validation_type: RequiredValidationType::Element
    }
);

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

#[derive(Deserialize, Serialize, SerializedBytes, Debug, Clone)]
pub struct Category {
    name: String,
}

entry_def!(Category
    EntryDef {
        id: "category".into(),
        visibility: EntryVisibility::Private,
        crdt_type: CrdtType,
        required_validations: RequiredValidations::default(),
        required_validation_type: RequiredValidationType::Element
    }
);

impl Category {
    pub fn new(name: String) -> Self {
        Category { name }
    }
}

#[derive(Deserialize, Serialize, SerializedBytes, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AliasIO {
    id: AgentPubKeyB64,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Deserialize, Serialize, SerializedBytes, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Alias {
    id: AgentPubKey,
    first_name: Option<String>, // this could be a BtreeMap too if more (customizeable) fields are needed
    last_name: Option<String>,
    created: Timestamp, // to determine the latest alias set for a particular contact
}

entry_def!(Alias
    EntryDef {
        id: "alias".into(),
        visibility: EntryVisibility::Private,
        crdt_type: CrdtType,
        required_validations: RequiredValidations::default(),
        required_validation_type: RequiredValidationType::Element
    }
);
