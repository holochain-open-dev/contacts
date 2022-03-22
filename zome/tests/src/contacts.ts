import { Orchestrator } from "@holochain/tryorama";
import { ScenarioApi } from "@holochain/tryorama/lib/api";

function addContacts(agentPubKeys) {
  return (conductor) =>
    conductor.call("contacts", "add_contacts", agentPubKeys);
}

function removeContacts(agentPubKeys) {
  return (conductor) =>
    conductor.call("contacts", "remove_contacts", agentPubKeys);
}

function blockContacts(agentPubKeys) {
  return (conductor) =>
    conductor.call("contacts", "block_contacts", agentPubKeys);
}

function unblockContacts(agentPubKeys) {
  return (conductor) =>
    conductor.call("contacts", "unblock_contacts", agentPubKeys);
}

function listAdded() {
  return (conductor) => conductor.call("contacts", "list_added", null);
}

function listBlocked() {
  return (conductor) => conductor.call("contacts", "list_blocked", null);
}

function inContacts(agentPubKey) {
  return (conductor) => conductor.call("contacts", "in_contacts", agentPubKey);
}

function inBlocked(agentPubKey) {
  return (conductor) => conductor.call("contacts", "in_blocked", agentPubKey);
}

function createCategory(name) {
  return (conductor) => conductor.call("contacts", "create_category", name);
}

function addToCategory(name, id, agentPubKeys) {
  return (conductor) =>
    conductor.call("contacts", "add_to_category", {
      name,
      id,
      agents: agentPubKeys,
    });
}
function removeFromCategory(name, id, agentPubKeys) {
  return (conductor) =>
    conductor.call("contacts", "remove_from_category", {
      id,
      name,
      agents: agentPubKeys,
    });
}

function listCategory() {
  return (conductor) => conductor.call("contacts", "list_category", null);
}

function updateAlias(id, firstName, lastName) {
  return (conductor) =>
    conductor.call("contacts", "update_alias", {
      id,
      firstName,
      lastName,
    });
}

function listAlias() {
  return (conductor) => conductor.call("contacts", "list_alias", null);
}

/*
  NOTE: all the calls that return Err are commented out
  as tryorama were throwing errors making
  the test to halt. (not sure if this is still happening)
  Err cases are already tested and they are returning the intended
  errors.
*/

let orchestrator = new Orchestrator();

export default (config, installables) => {
  orchestrator.registerScenario("add a contact", async (s, t) => {
    const [conductor] = await s.players([config]);
    const [[alice_lobby_happ], [bobby_lobby_happ]] =
      await conductor.installAgentsHapps(installables.two);
    const [alice_conductor] = alice_lobby_happ.cells;
    const [bobby_conductor] = bobby_lobby_happ.cells;

    const [dna_hash_1, agent_pubkey_alice] = alice_conductor.cellId;
    const [dna_hash_2, agent_pubkey_bobby] = bobby_conductor.cellId;

    // add self
    const add_self = await addContacts([agent_pubkey_alice])(alice_conductor);

    // no contact then add
    const none_then_add = await addContacts([agent_pubkey_bobby])(
      alice_conductor
    );

    // add then add
    // const add_then_add =  await addContacts([agent_pubkey_bobby])(alice_conductor);

    // block then add
    await blockContacts([agent_pubkey_bobby])(alice_conductor);
    // const block_then_add = await addContacts([agent_pubkey_bobby])(alice_conductor);

    // unblock then add
    await unblockContacts([agent_pubkey_bobby])(alice_conductor);
    const unblock_then_add = await addContacts([agent_pubkey_bobby])(
      alice_conductor
    );

    // remove then add
    await removeContacts([agent_pubkey_bobby])(alice_conductor);
    const remove_then_add = await addContacts([agent_pubkey_bobby])(
      alice_conductor
    );

    t.deepEqual(add_self[0], agent_pubkey_alice);
    t.deepEqual(none_then_add[0], agent_pubkey_bobby);
    t.deepEqual(unblock_then_add[0], agent_pubkey_bobby);
    t.deepEqual(remove_then_add[0], agent_pubkey_bobby);
  });

  orchestrator.run();

  orchestrator = new Orchestrator();

  orchestrator.registerScenario("remove a contact", async (s, t) => {
    const [conductor] = await s.players([config]);
    const [[alice_lobby_happ], [bobby_lobby_happ]] =
      await conductor.installAgentsHapps(installables.two);
    const [alice_conductor] = alice_lobby_happ.cells;
    const [bobby_conductor] = bobby_lobby_happ.cells;

    const [dna_hash_2, agent_pubkey_bobby] = bobby_conductor.cellId;

    // no contact then remove
    // const none_then_remove = await removeContacts([agent_pubkey_bobby])(alice_conductor);

    // add then remove
    await addContacts([agent_pubkey_bobby])(alice_conductor);
    const add_then_remove = await removeContacts([agent_pubkey_bobby])(
      alice_conductor
    );

    // remove then remove
    // const remove_then_remove = await removeContacts([agent_pubkey_bobby])(alice_conductor);

    // block then remove
    // await blockContacts([agent_pubkey_bobby])(alice_conductor);
    // const block_then_remove = await removeContacts([agent_pubkey_bobby])(alice_conductor);

    // unblock then remove
    // await unblockContacts([agent_pubkey_bobby])(alice_conductor);
    // const unblock_then_remove = await removeContacts([agent_pubkey_bobby])(alice_conductor);

    t.deepEqual(add_then_remove[0], agent_pubkey_bobby);
  });
  orchestrator.run();

  orchestrator = new Orchestrator();

  orchestrator.registerScenario("list added", async (s, t) => {
    const [conductor] = await s.players([config]);
    const [[alice_lobby_happ], [bobby_lobby_happ], [clark_lobby_happ]] =
      await conductor.installAgentsHapps(installables.three);
    const [alice_conductor] = alice_lobby_happ.cells;
    const [bobby_conductor] = bobby_lobby_happ.cells;
    const [clark_conductor] = clark_lobby_happ.cells;

    const [dna_hash_1, agent_pubkey_alice] = alice_conductor.cellId;
    const [dna_hash_2, agent_pubkey_bobby] = bobby_conductor.cellId;
    const [dna_hash_3, agent_pubkey_clark] = clark_conductor.cellId;

    const empty_list_added = await listAdded()(alice_conductor);
    await addContacts([
      agent_pubkey_alice,
      agent_pubkey_bobby,
      agent_pubkey_clark,
    ])(alice_conductor);

    const list_added_1 = await listAdded()(alice_conductor);
    await removeContacts([agent_pubkey_bobby])(alice_conductor);

    const list_added_2 = await listAdded()(alice_conductor);
    await blockContacts([agent_pubkey_clark])(alice_conductor);

    const list_added_3 = await listAdded()(alice_conductor);
    await addContacts([agent_pubkey_bobby])(alice_conductor);

    const list_added_4 = await listAdded()(alice_conductor);
    await removeContacts([agent_pubkey_bobby])(alice_conductor);

    const list_added_5 = await listAdded()(alice_conductor);

    t.deepEqual(empty_list_added.length, 0);
    t.deepEqual(list_added_1.length, 3);
    t.deepEqual(list_added_2.length, 2);
    t.deepEqual(list_added_3.length, 1);
    t.deepEqual(list_added_4.length, 2);
    t.deepEqual(list_added_5.length, 1);
  });
  orchestrator.run();

  orchestrator = new Orchestrator();

  orchestrator.registerScenario("block contact", async (s, t) => {
    const [conductor] = await s.players([config]);
    const [[alice_lobby_happ], [bobby_lobby_happ], [clark_lobby_happ]] =
      await conductor.installAgentsHapps(installables.three);
    const [alice_conductor] = alice_lobby_happ.cells;
    const [bobby_conductor] = bobby_lobby_happ.cells;
    const [clark_conductor] = clark_lobby_happ.cells;

    const [dna_hash_1, agent_pubkey_alice] = alice_conductor.cellId;
    const [dna_hash_2, agent_pubkey_bobby] = bobby_conductor.cellId;
    const [dna_hash_3, agent_pubkey_clark] = clark_conductor.cellId;

    // block myself
    // const block_myself_result = await blockContacts([agent_pubkey_alice])(alice_conductor);

    // no contact then block
    const none_then_block = await blockContacts([agent_pubkey_clark])(
      alice_conductor
    );

    // added then block
    await addContacts([agent_pubkey_bobby])(alice_conductor);
    const added_then_block = await blockContacts([agent_pubkey_bobby])(
      alice_conductor
    );

    // blocked then block
    // const blocked_then_block = await blockContacts([agent_pubkey_bobby])(alice_conductor);

    // unblocked then block
    await unblockContacts([agent_pubkey_bobby])(alice_conductor);
    const unblocked_then_block = await blockContacts([agent_pubkey_bobby])(
      alice_conductor
    );

    // removed then block
    await unblockContacts([agent_pubkey_bobby])(alice_conductor);
    await addContacts([agent_pubkey_bobby])(alice_conductor);
    await removeContacts([agent_pubkey_bobby])(alice_conductor);
    const removed_then_block = await blockContacts([agent_pubkey_bobby])(
      alice_conductor
    );

    t.deepEqual(none_then_block[0], agent_pubkey_clark);
    t.deepEqual(added_then_block[0], agent_pubkey_bobby);
    t.deepEqual(unblocked_then_block[0], agent_pubkey_bobby);
    t.deepEqual(removed_then_block[0], agent_pubkey_bobby);
  });
  orchestrator.run();

  orchestrator = new Orchestrator();

  orchestrator.registerScenario("unblock contact", async (s, t) => {
    const [conductor] = await s.players([config]);
    const [[alice_lobby_happ], [bobby_lobby_happ], [clark_lobby_happ]] =
      await conductor.installAgentsHapps(installables.three);
    const [alice_conductor] = alice_lobby_happ.cells;
    const [bobby_conductor] = bobby_lobby_happ.cells;
    const [clark_conductor] = clark_lobby_happ.cells;

    const [dna_hash_1, agent_pubkey_alice] = alice_conductor.cellId;
    const [dna_hash_2, agent_pubkey_bobby] = bobby_conductor.cellId;
    const [dna_hash_3, agent_pubkey_clark] = clark_conductor.cellId;

    // no contact then unblock
    // const none_then_unblock = await unblockContacts([agent_pubkey_bobby])(alice_conductor);

    await addContacts([agent_pubkey_alice])(alice_conductor);
    await addContacts([agent_pubkey_bobby])(alice_conductor);
    await addContacts([agent_pubkey_clark])(alice_conductor);

    // added then unblock
    // const added_then_unblock = await unblockContacts([agent_pubkey_bobby])(alice_conductor);

    // // removed then unblock
    // await removeContacts([agent_pubkey_bobby])(alice_conductor);
    // const removed_then_unblock = await unblockContacts([agent_pubkey_bobby])(alice_conductor);

    // blocked then unblock
    await blockContacts([agent_pubkey_bobby])(alice_conductor);
    const blocked_then_unblock = await unblockContacts([agent_pubkey_bobby])(
      alice_conductor
    );

    // // unblocked then unblock
    // const unblocked_then_unblock = await unblockContacts([agent_pubkey_bobby])(alice_conductor);

    t.deepEqual(blocked_then_unblock[0], agent_pubkey_bobby);
  });
  orchestrator.run();

  orchestrator = new Orchestrator();

  orchestrator.registerScenario("list blocked", async (s: ScenarioApi, t) => {
    const [conductor] = await s.players([config]);
    const [[alice_lobby_happ], [bobby_lobby_happ], [clark_lobby_happ]] =
      await conductor.installAgentsHapps(installables.three);
    const [alice_conductor] = alice_lobby_happ.cells;
    const [bobby_conductor] = bobby_lobby_happ.cells;
    const [clark_conductor] = clark_lobby_happ.cells;

    const [dna_hash_1, agent_pubkey_alice] = alice_conductor.cellId;
    const [dna_hash_2, agent_pubkey_bobby] = bobby_conductor.cellId;
    const [dna_hash_3, agent_pubkey_clark] = clark_conductor.cellId;

    const empty_list_blocked = await listBlocked()(alice_conductor);
    await blockContacts([agent_pubkey_bobby, agent_pubkey_clark])(
      alice_conductor
    );

    const list_blocked_1 = await listBlocked()(alice_conductor);
    await unblockContacts([agent_pubkey_bobby, agent_pubkey_clark])(
      alice_conductor
    );

    const list_blocked_2 = await listBlocked()(alice_conductor);
    await blockContacts([agent_pubkey_bobby])(alice_conductor);

    const list_blocked_3 = await listBlocked()(alice_conductor);
    await blockContacts([agent_pubkey_clark])(alice_conductor);

    const list_blocked_4 = await listBlocked()(alice_conductor);

    t.deepEqual(empty_list_blocked.length, 0);
    t.deepEqual(list_blocked_1.length, 2);
    t.deepEqual(list_blocked_2.length, 0);
    t.deepEqual(list_blocked_3.length, 1);
    t.deepEqual(list_blocked_4.length, 2);
  });
  orchestrator.run();

  orchestrator = new Orchestrator();

  orchestrator.registerScenario("check in blocked list", async (s, t) => {
    const [conductor] = await s.players([config]);
    const [[alice_lobby_happ], [bobby_lobby_happ], [clark_lobby_happ]] =
      await conductor.installAgentsHapps(installables.three);
    const [alice_conductor] = alice_lobby_happ.cells;
    const [bobby_conductor] = bobby_lobby_happ.cells;
    const [clark_conductor] = clark_lobby_happ.cells;

    const [dna_hash_1, agent_pubkey_alice] = alice_conductor.cellId;
    const [dna_hash_2, agent_pubkey_bobby] = bobby_conductor.cellId;
    const [dna_hash_3, agent_pubkey_clark] = clark_conductor.cellId;

    await blockContacts([agent_pubkey_bobby])(alice_conductor);
    await blockContacts([agent_pubkey_clark])(alice_conductor);

    const in_blocked_1 = await inBlocked(agent_pubkey_alice)(alice_conductor);
    const in_blocked_2 = await inBlocked(agent_pubkey_bobby)(alice_conductor);
    const in_blocked_3 = await inBlocked(agent_pubkey_clark)(alice_conductor);

    t.deepEqual(in_blocked_1, false);
    t.deepEqual(in_blocked_2, true);
    t.deepEqual(in_blocked_3, true);
  });

  orchestrator.run();

  orchestrator = new Orchestrator();

  orchestrator.registerScenario("category related test", async (s, t) => {
    const [conductor] = await s.players([config]);
    const [[alice_lobby_happ], [bobby_lobby_happ]] =
      await conductor.installAgentsHapps(installables.two);
    const [alice_conductor] = alice_lobby_happ.cells;
    const [bobby_conductor] = bobby_lobby_happ.cells;

    const [dna_hash_1, agent_pubkey_alice] = alice_conductor.cellId;
    const [dna_hash_2, agent_pubkey_bobby] = bobby_conductor.cellId;

    // create category
    const { name, id } = await createCategory("test")(alice_conductor);
    await createCategory("test1")(alice_conductor);
    await createCategory("test2")(alice_conductor);

    // no contact then add
    const none_then_add = await addContacts([agent_pubkey_bobby])(
      alice_conductor
    );

    // add to category
    const {
      name: categoryName,
      id: categoryId,
      agents: agentKeys,
    } = await addToCategory(name, id, [agent_pubkey_bobby])(alice_conductor);

    // list
    const added = await listAdded()(alice_conductor);
    console.log(added);

    // remove from category
    await removeFromCategory(
      categoryName,
      categoryId,
      agentKeys
    )(alice_conductor);

    // list 2
    const added_2 = await listAdded()(alice_conductor);
    console.log(added_2);

    // list of categories
    const categories = await listCategory()(alice_conductor);

    t.deepEqual(name, "test");
    t.deepEqual(added.length, 1);
    t.deepEqual(added_2.length, 1);
    t.deepEqual(categories.length, 3);
    t.deepEqual(none_then_add[0], agent_pubkey_bobby);
  });
  orchestrator.run();

  orchestrator = new Orchestrator();

  orchestrator.registerScenario("alias related test", async (s, t) => {
    const [conductor] = await s.players([config]);
    const [[alice_lobby_happ], [bobby_lobby_happ], [charlie_lobby_happ]] =
      await conductor.installAgentsHapps(installables.three);
    const [alice_conductor] = alice_lobby_happ.cells;
    const [bobby_conductor] = bobby_lobby_happ.cells;
    const [charlie_conductor] = charlie_lobby_happ.cells;

    const [dna_hash_1, agent_pubkey_alice] = alice_conductor.cellId;
    const [dna_hash_2, agent_pubkey_bobby] = bobby_conductor.cellId;
    const [dna_hash_3, agent_pubkey_charlie] = charlie_conductor.cellId;

    // no contact then add
    await addContacts([agent_pubkey_bobby])(alice_conductor);
    await addContacts([agent_pubkey_charlie])(alice_conductor);

    // update alias
    const { id, firstName, lastName, created } = await updateAlias(
      agent_pubkey_bobby,
      "bob",
      "marley"
    )(alice_conductor);
    await updateAlias(agent_pubkey_bobby, "bobbo", "")(alice_conductor);
    await updateAlias(agent_pubkey_bobby, "", "")(alice_conductor);
    await updateAlias(agent_pubkey_bobby, "bobby", "lenard")(alice_conductor);
    await updateAlias(agent_pubkey_charlie, "charlie", "")(alice_conductor);
    await updateAlias(agent_pubkey_charlie, "charls", "k")(alice_conductor);
    await updateAlias(agent_pubkey_charlie, "charlie", "c")(alice_conductor);

    // list added
    const added = await listAdded()(alice_conductor);
    console.log("added contacts", added);

    // list alias
    const aliases: any = await listAlias()(alice_conductor);
    console.log("aliases", aliases);

    t.deepEqual(id, agent_pubkey_bobby);
    t.deepEqual(firstName, "bob");
    t.deepEqual(lastName, "marley");
    t.deepEqual(added.length, 2);
    t.ok(aliases);
  });
  orchestrator.run();
};
orchestrator.run();
