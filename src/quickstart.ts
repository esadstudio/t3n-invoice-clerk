/**
 * Official T3N Quickstart — https://docs.terminal3.io/developers/adk/get-started/quickstart
 *
 * One claim, one key. Tenant handshake uses T3N_API_KEY only.
 * Agent CLI aliases (T3N_AGENT_KEY / AGENT_KEY) are the same value.
 */
import {
  T3nClient,
  setEnvironment,
  loadWasmComponent,
  fetchTrustedManifest,
  eth_get_address,
  metamask_sign,
  createEthAuthInput,
} from "@terminal3/t3n-sdk";

// Single-key path: read T3N_API_KEY; treat T3N_AGENT_KEY / AGENT_KEY as aliases.
if (!process.env.T3N_API_KEY && process.env.T3N_AGENT_KEY) {
  process.env.T3N_API_KEY = process.env.T3N_AGENT_KEY;
}
if (!process.env.T3N_API_KEY && process.env.AGENT_KEY) {
  process.env.T3N_API_KEY = process.env.AGENT_KEY;
}
process.env.T3N_AGENT_KEY =
  process.env.T3N_AGENT_KEY || process.env.AGENT_KEY || process.env.T3N_API_KEY;

if (!process.env.T3N_API_KEY) {
  console.error(
    "Export T3N_API_KEY from a single claim at https://go.terminal3.io/adk-community (shown once). T3N_AGENT_KEY is the same value — do not claim a second key.",
  );
  process.exit(1);
}

setEnvironment("testnet"); // the public SDK defaults to testnet — set it explicitly so your target cluster is unambiguous (and switch to "production" when you go live)

const T3N_API_KEY = process.env.T3N_API_KEY!;
const wasmComponent = await loadWasmComponent(); // all crypto runs inside this component
const address = eth_get_address(T3N_API_KEY);

const t3n = new T3nClient({
  // Verifies you're really talking to a genuine T3N enclave, not just
  // whatever the server claims — see "Verify the trust anchor" below.
  // Node URL comes from setEnvironment above.
  trustAnchor: await fetchTrustedManifest("testnet"),
  wasmComponent,
  handlers: {
    EthSign: metamask_sign(address, undefined, T3N_API_KEY),
  },
});

await t3n.handshake();
const did = await t3n.authenticate(createEthAuthInput(address));
const tenantDid = did.value; // did:t3n:... — you'll reuse this exact variable in every later step

console.log("Connected as:", tenantDid);
