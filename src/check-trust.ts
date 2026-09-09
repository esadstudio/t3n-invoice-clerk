/**
 * Official trust path only — no API key, no handshake, no T3N_UNSAFE_TRUST.
 * Fails if fetchTrustedManifest("testnet") rejects the live cluster manifest.
 */
import { setEnvironment, fetchTrustedManifest } from "@terminal3/t3n-sdk";

setEnvironment("testnet");

const anchor = await fetchTrustedManifest("testnet");
const source = anchor.source;

console.log("Official trust path OK");
console.log("manifest_url:", source?.url ?? "(none)");
console.log("manifest_version:", source?.manifest_version ?? "(none)");
console.log("signed_at:", source?.signed_at ?? "(none)");
console.log("peer_ids:", anchor.expected_peer_ids.length);
console.log("rtmr3_allowlist:", anchor.rtmr3_allowlist.length);
