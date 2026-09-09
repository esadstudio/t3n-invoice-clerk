# Known issues and pitfalls

Checked against official Terminal 3 docs and this scaffold. Say so plainly when something is a platform issue — do not invent a fix.

## This repo (by design)

| Item | Notes |
| --- | --- |
| One key | Official [Register a Public Agent](https://docs.terminal3.io/developers/agents/register-agent) and [Invoke](https://docs.terminal3.io/developers/adk/get-started/walkthrough/invoke-contract) describe a **separate** agent key with its own credits. This repo: `T3N_API_KEY` and `AGENT_KEY` are the **same** value in a local `.env`. Do not claim a second key. Never commit `.env`. |
| `InsufficientCreditError` | Metered calls charge the **calling** DID. With one key, tenant and agent are the same DID, so tenant test credits apply. A key generated outside the claim page starts at zero. |
| Contract WASM not built | `src/contract/` is a walkthrough stub. `cargo build --target wasm32-wasip2` needs Rust + vendored `wit/deps/` from [Terminal-3/z-tenant-flight](https://github.com/Terminal-3/z-tenant-flight). Host interface versions in the public walkthrough (`@1.2.0` / `@2.2.0`) may differ from the reference repo (`@1.0.0` / `@2.1.0`) — vendor what the **target cluster** provides. |
| `T3N_DID` unused at runtime | Optional in `.env.example` / [docs/EARN.md](EARN.md). Earn-form reminder only (`did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9`). Quickstart always prints the session DID. Do not treat it as a runtime secret or identity fallback. |

## Official ADK pitfalls (from the docs)

| Symptom | Likely cause | Fix |
| --- | --- | --- |
| `Top-level await is currently not supported with the "cjs" output format` | Missing `"type": "module"` | This repo sets it. |
| `Invalid Ethereum private key` | `T3N_API_KEY` not exported in **this** shell | `export T3N_API_KEY=…` then `npx tsx src/quickstart.ts` |
| `T3nClient: trustAnchor is required` | Omitted `fetchTrustedManifest` | Already in `src/quickstart.ts` |
| WASM parse / module-loading under Next, Vite, Webpack | Bundler processes the SDK WASM | Use this plain Node script. Do not move Quickstart into a bundler without an external-packages exception. |
| `tenant not found` | DID was hardcoded or derived | Read `did.value` after `authenticate` |
| `AccessDenied` on KV | `readers` omitted on `maps.create` | Set `readers` and `writers` to the new `contract_id` |
| Map path misses | `tenant_did()` used as a string | Hex-encode the raw bytes once |
| `host/http.egress_denied` | Outbound HTTP without a user grant | Invoice Clerk stub has no HTTP. If you add it, the **data owner** signs `member-delegation-update`. |
| `version is not higher than current version` | Re-register same tail/version | Bump `version` |
| Generic HTTP 500 | Egress/ACL or platform | Save `request_id`; retry once; then Telegram / `devrel@terminal3.io` |

## Do not invent

These showed up in community code and are **not** in the current SDK ([Reference](https://docs.terminal3.io/developers/adk/reference)): `buildDelegationCredential`, `DelegationCustodialClient`, envelope-style per-call credentials. Use `member-delegation-update` / `updateMemberDelegation` when you need grants.

`setEnvironment` accepts `"testnet"` \| `"production"` (`"sandbox"` is an alias of testnet). This repo stays on **testnet**.
