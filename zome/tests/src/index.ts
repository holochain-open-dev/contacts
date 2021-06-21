import {
  Orchestrator,
  Config,
  InstallAgentsHapps,
  Player,
} from "@holochain/tryorama";
import path from "path";

const conductorConfig = Config.gen();

// Construct proper paths for your DNAs
const calendarEvents = path.join(__dirname, "../../workdir/dna/sample.dna");

const sleep = (ms) => new Promise((resolve) => setTimeout(() => resolve(null), ms));



const orchestrator = new Orchestrator();

// create an InstallAgentsHapps array with your DNAs to tell tryorama what
// to install into the conductor.
const installation: InstallAgentsHapps = [
  // agent 0
  [
    // happ 0
    [calendarEvents],
  ],
  [
    // happ 0
    [calendarEvents],
  ],
];

orchestrator.registerScenario(
  "create and get a calendar event",
  async (s, t) => {
    const [alice, bob] = await s.players([conductorConfig, conductorConfig]);

    const [[alice_happ]] = await alice.installAgentsHapps(installation);

    const [[bob_happ]] = await bob.installAgentsHapps(installation);

    const alice_calendar = alice_happ.cells[0];
    const bob_calendar = bob_happ.cells[0];

  }
);

orchestrator.run();
