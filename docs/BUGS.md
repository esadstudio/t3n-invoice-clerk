# Known issues and pitfalls

## Sponsor-known blocker — official trust (do not pin an older SDK)

**This is a sponsor-side API/SDK mismatch.** Earn must not depend on chasing `@terminal3/t3n-sdk` versions.

| | |
| --- | --- |
| Error | `Error: Trust manifest at https://cn-api.sg.testnet.t3n.terminal3.io/api/trust-manifest is malformed.` |
| Where | `fetchTrustedManifest("testnet")` in `src/quickstart.ts` (`trustAnchor`) |
| URL | https://cn-api.sg.testnet.t3n.terminal3.io/api/trust-manifest |
| Live body | HTTP 200: `cluster`, `version`, `peer_ids`, `rtmr3_allowlist`, `signed_at`, `signature` |
| Missing | **`rtmr1_allowlist`** — required by `@terminal3/t3n-sdk` **5.14.0** (`SignedTrustManifest` / `TrustAnchor`) |
| Not the cause | Bad `T3N_API_KEY`, empty response, or our app inventing a schema |

Code default stays `fetchTrustedManifest("testnet")`. No silent `catch`.

**Local/demo workaround** (not default, not CI, not the Earn “official path”): `T3N_UNSAFE_TRUST=1` in `.env` → `{ unsafe_trust_server: true }` + loud console warn. See [Verify the trust anchor](https://docs.terminal3.io/developers/adk/tips/verify-trust-anchor) (opt-out is for local nodes; we use it only because hosted testnet’s signed body is incomplete).

**Contact:** [developer Telegram](https://t.me/terminal3developer) / `devrel@terminal3.io`

When the cluster republishes a signed manifest that includes `rtmr1_allowlist`, remove `T3N_UNSAFE_TRUST` from local `.env` and re-run Quickstart on the official path.

## This repo (by design)

| Item | Notes |
| --- | --- |
| One key | `T3N_API_KEY` and `AGENT_KEY` are the same local value. Never commit `.env`. |
| `T3N_DID` | Earn-form reminder only (`did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9`). |
| Contract WASM | Stubs in `src/contract/`. Vendor host WIT from [z-tenant-flight](https://github.com/Terminal-3/z-tenant-flight) before build. |

## Official ADK pitfalls

| Symptom | Fix |
| --- | --- |
| Missing `"type": "module"` | This repo sets it. |
| `Invalid Ethereum private key` | Fill `T3N_API_KEY` in local `.env` or the shell. |
| Next/Vite WASM errors | Keep Quickstart as a plain Node script. |
| `tenant not found` | Use `did.value` from `authenticate`, do not derive. |

`setEnvironment` is `"testnet"` here. No invented SDK APIs.
