import { Config, InstallAgentsHapps } from "@holochain/tryorama";
import path from "path";
import contacts from "./contacts";

const conductorConfig = Config.gen({});

// Construct proper paths for your DNAs
const contactsSampleDna = path.join(
  __dirname,
  "../../workdir/dna/contacts.dna"
);

// create an InstallAgentsHapps array with your DNAs to tell tryorama what
// to install into the conductor.
const installAgent: InstallAgentsHapps = [[[contactsSampleDna]]];

const install2Agents: InstallAgentsHapps = [
  [[contactsSampleDna]],
  [[contactsSampleDna]],
];

const install3Agents: InstallAgentsHapps = [
  [[contactsSampleDna]],
  [[contactsSampleDna]],
  [[contactsSampleDna]],
];

export interface Installables {
  [key: string]: InstallAgentsHapps;
}

const installables: Installables = {
  one: installAgent,
  two: install2Agents,
  three: install3Agents,
};

contacts(conductorConfig, installables);
