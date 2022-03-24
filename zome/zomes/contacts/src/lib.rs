use entries::contacts::add_to_category::add_to_category_handler;
use entries::contacts::create_category::create_category_handler;
use entries::contacts::list_alias::list_alias_handler;
use entries::contacts::list_category::list_category_handler;
use entries::contacts::remove_from_category::remove_from_category_handler;
use entries::contacts::update_alias::update_alias_handler;
use hdk::prelude::*;
use holo_hash::AgentPubKeyB64;
use std::collections::HashMap;

mod entries;
mod utils;

use contacts::{Alias, AliasIO, Category, CategoryIO, CategoryWithId, Contact, ContactOutput};
use entries::contacts::{self};

use contacts::add_contacts::add_contacts_handler;
use contacts::block_contacts::block_contacts_handler;
use contacts::in_blocked::in_blocked_handler;
use contacts::in_contacts::in_contacts_handler;
use contacts::list_added::list_added_handler;
use contacts::list_blocked::list_blocked_handler;
use contacts::remove_contacts::remove_contacts_handler;
use contacts::unblock_contacts::unblock_contacts_handler;

entry_defs![
    Contact::entry_def(),
    Category::entry_def(),
    Alias::entry_def()
];

#[hdk_extern]
fn add_contacts(agent_ids: Vec<AgentPubKeyB64>) -> ExternResult<Vec<AgentPubKeyB64>> {
    add_contacts_handler(agent_ids)
}

#[hdk_extern]
fn remove_contacts(agent_ids: Vec<AgentPubKeyB64>) -> ExternResult<Vec<AgentPubKeyB64>> {
    remove_contacts_handler(agent_ids)
}

#[hdk_extern]
fn block_contacts(agent_ids: Vec<AgentPubKeyB64>) -> ExternResult<Vec<AgentPubKeyB64>> {
    block_contacts_handler(agent_ids)
}

#[hdk_extern]
fn unblock_contacts(agent_ids: Vec<AgentPubKeyB64>) -> ExternResult<Vec<AgentPubKeyB64>> {
    unblock_contacts_handler(agent_ids)
}

#[hdk_extern]
fn list_added(_: ()) -> ExternResult<Vec<ContactOutput>> {
    list_added_handler()
}

#[hdk_extern]
fn list_blocked(_: ()) -> ExternResult<Vec<ContactOutput>> {
    list_blocked_handler()
}

#[hdk_extern]
fn in_contacts(agent_pubkey: AgentPubKeyB64) -> ExternResult<bool> {
    in_contacts_handler(agent_pubkey)
}

#[hdk_extern]
fn in_blocked(agent_pubkey: AgentPubKeyB64) -> ExternResult<bool> {
    in_blocked_handler(agent_pubkey)
}

#[hdk_extern]
fn create_category(name: String) -> ExternResult<CategoryWithId> {
    create_category_handler(name)
}

#[hdk_extern]
fn add_to_category(io: CategoryIO) -> ExternResult<CategoryIO> {
    add_to_category_handler(io)
}

#[hdk_extern]
fn remove_from_category(io: CategoryIO) -> ExternResult<CategoryIO> {
    remove_from_category_handler(io)
}

#[hdk_extern]
fn list_category(_: ()) -> ExternResult<Vec<CategoryWithId>> {
    list_category_handler()
}

#[hdk_extern]
fn update_alias(input: AliasIO) -> ExternResult<AliasIO> {
    update_alias_handler(input)
}

#[hdk_extern]
fn list_alias(_: ()) -> ExternResult<HashMap<String, Option<AliasIO>>> {
    list_alias_handler()
}
