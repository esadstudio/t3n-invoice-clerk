# Known issues and pitfalls

Checked against official Terminal 3 docs and this scaffold. Say so plainly when something is a platform issue — do not invent a fix.

## Sponsor-known: trust-manifest vs SDK ≥ 5.3.0

**Status: official Earn path is green on pinned `@terminal3/t3n-sdk@5.2.0`.** Newer SDKs are a sponsor/API mismatch — do not use them for Earn until the cluster republishes the field below.

| | |
| --- | --- |
| Symptom | `Error: Trust manifest at https://cn-api.sg.testnet.t3n.terminal3.io/api/trust-manifest is malformed.` from `fetchTrustedManifest("testnet")` |
| URL | https://cn-api.sg.testnet.t3n.terminal3.io/api/trust-manifest |
| Live body (2026-09-09) | HTTP 200 JSON: `cluster`, `version`, `peer_ids`, `rtmr3_allowlist`, `signed_at`, `signature` |
| Missing field | **`rtmr1_allowlist`** (required, non-empty, on SDK `SignedTrustManifest` / `TrustAnchor` since 5.3.0 — SP-003) |
| Not the cause | Bad `T3N_API_KEY`, network 4xx, or unsigned empty body |

### Version matrix (live manifest, `fetchTrustedManifest("testnet")` only — no handshake)

| `@terminal3/t3n-sdk` | Result |
| --- | --- |
| 4.46.0, 5.0.0, 5.1.0, **5.2.0** | OK — returns `expected_peer_ids` + `rtmr3_allowlist` |
| 5.3.0, 5.4.0, 5.5.0, 5.8.0, 5.10.0–5.14.0 (latest) | throws `… is malformed.` |

This repo **pins `5.2.0` exactly** so the official Quickstart / Earn path does **not** need `T3N_UNSAFE_TRUST`. `npm run check:trust` re-verifies the official fetch.

When the sponsor republishes a signed manifest that includes `rtmr1_allowlist`, bump the SDK and drop the pin. Until then, `npx @terminal3/t3n-sdk` (unpinned latest) will fail — use `npm exec -- t3n` / `npm run whoami`.

`T3N_UNSAFE_TRUST=1` → `{ unsafe_trust_server: true }` remains **opt-in local/debug only**. Never default. Never Earn. Never CI. Never a silent `catch`. Docs: [Verify the trust anchor](https://docs.terminal3.io/developers/adk/tips/verify-trust-anchor). Report cluster drift to `devrel@terminal3.io` / [developer Telegram](https://t.me/terminal3developer).

## This repo (by design)

| Item | Notes |
| --- | --- |
| One key | Official [Register a Public Agent](https://docs.terminal3.io/developers/agents/register-agent) describes a **separate** agent key. This repo: `T3N_API_KEY` and `AGENT_KEY` are the **same** value in a local `.env`. Never commit `.env`. |
| `InsufficientCreditError` | Metered calls charge the **calling** DID. One key ⇒ same DID as the tenant; claim-page credits apply. |
| Contract WASM not built | `src/contract/` is a walkthrough stub. Vendor `wit/deps/` from [Terminal-3/z-tenant-flight](https://github.com/Terminal-3/z-tenant-flight) before `wasm32-wasip2` build. |
| `T3N_DID` unused at runtime | Optional Earn-form reminder (`did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9`). Session DID from `authenticate()` / `whoami` wins. |

## Official ADK pitfalls (from the docs)

| Symptom | Likely cause | Fix |
| --- | --- | --- |
| `Top-level await is currently not supported with the "cjs" output format` | Missing `"type": "module"` | This repo sets it. |
| `Invalid Ethereum private key` | `T3N_API_KEY` not exported in **this** shell | Local `.env` or `export T3N_API_KEY=…` |
| `T3nClient: trustAnchor is required` | Omitted `fetchTrustedManifest` | Already in `src/quickstart.ts` |
| WASM parse / module-loading under Next, Vite, Webpack | Bundler processes the SDK WASM | Use this plain Node script. |
| `tenant not found` | DID was hardcoded or derived | Read `did.value` after `authenticate` |
| `AccessDenied` on KV | `readers` omitted on `maps.create` | Set `readers` and `writers` to the new `contract_id` |
| Map path misses | `tenant_did()` used as a string | Hex-encode the raw bytes once |
| `host/http.egress_denied` | Outbound HTTP without a user grant | Invoice Clerk stub has no HTTP. |
| `version is not higher than current version` | Re-register same tail/version | Bump `version` |
| Generic HTTP 500 | Egress/ACL or platform | Save `request_id`; retry once; then Telegram / `devrel@terminal3.io` |

## Do not invent

These showed up in community code and are **not** in the current SDK ([Reference](https://docs.terminal3.io/developers/adk/reference)): `buildDelegationCredential`, `DelegationCustodialClient`, envelope-style per-call credentials. Use `member-delegation-update` / `updateMemberDelegation` when you need grants.

`setEnvironment` accepts `"testnet"` \| `"production"` (`"sandbox"` is an alias of testnet). This repo stays on **testnet**.
