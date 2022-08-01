use hdk::prelude::*;
use contacts_integrity_types::*;

#[derive(Clone, Deserialize, PartialEq, Serialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryIO {
    // id: EntryHashB64,
    pub id: EntryHash,
    pub name: String,
    pub agents: Vec<AgentPubKey>,
}

#[derive(Clone, Deserialize, PartialEq, Serialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContactOutput {
    // id: AgentPubKeyB64,
    pub id: AgentPubKey,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub category: Option<CategoryWithId>,
}

#[derive(Deserialize, Serialize, SerializedBytes, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AliasIO {
    // id: AgentPubKeyB64,
    pub id: AgentPubKey,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}