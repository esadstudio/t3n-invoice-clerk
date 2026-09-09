# T3N Invoice Clerk

Enterprise vendor invoice intake on [Terminal 3](https://docs.terminal3.io/developers/adk/get-started/quickstart) (T3N). **Ledger lines only — never payment authority.** Testnet. Free-token path.

Listing: [T3N Agent Build Challenge](https://superteam.fun/earn/listing/t3n-agent-build-challenge/) (290 USDC, due 2026-09-16).

## 5-minute path

One claim. One key. The DID is for the Earn form later — not required to install or run Quickstart.

### 1. Claim (once)

Open [https://go.terminal3.io/adk-community](https://go.terminal3.io/adk-community), sign in, copy the key (shown once).

### 2. Local `.env` (same key, two names)

Copy `.env.example` → `.env` (Windows: `D:\DEV\t3n-invoice-clerk\.env`). Fill both with the **same** claim key:

```
T3N_API_KEY=
AGENT_KEY=
T3N_DID=did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9
```

`T3N_DID` is optional (Earn form reminder). Quickstart does not read it. **Never commit `.env` or `*.pem`.** `.gitignore` already excludes them. You can also `export` `T3N_API_KEY` / `AGENT_KEY` in the shell.

### 3. Install and connect

```bash
npm install
npx tsx src/quickstart.ts
# Connected as: did:t3n:…
```

That script is the official Quickstart: `setEnvironment("testnet")`, `loadWasmComponent`, `fetchTrustedManifest("testnet")`, `eth_get_address`, `metamask_sign`, handshake, authenticate.

Official trust is the default (`fetchTrustedManifest("testnet")` on **pinned** `@terminal3/t3n-sdk@5.2.0`). Earn must **not** use `T3N_UNSAFE_TRUST`. Prove the fetch with `npm run check:trust` (no key). SDK 5.3.0+ rejects the live testnet manifest — see [docs/BUGS.md](docs/BUGS.md). Use `npm exec -- t3n` / `npm run whoami`, not unpinned `npx @terminal3/t3n-sdk`.

### 4. Register the agent (optional)

Same key. Checklist: [`src/register-agent.md`](src/register-agent.md) — `whoami`, `create-card`, `host-card --env testnet`.

Card skeleton: [`src/agent/agent-card.json`](src/agent/agent-card.json) (&lt;16 KiB).

### 5. Contract notes (stubs)

Invoice facts under `z:<tid>:invoice-ledger` (amount, asset, payee, due date, source ref). See [`src/contract/`](src/contract/). Follow Terminal 3 docs; do not invent APIs. Rust/WASM build is a later step (vendor host WIT from the official reference crate).

## Earn-form DID (documentation only)

`T3N_DID=did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9`

Documented in `.env.example` and [docs/EARN.md](docs/EARN.md). Not a runtime secret. Quickstart identity is whatever `authenticate` returns.

## Constraints

- Testnet only. No production. No buy. No payments.
- Never commit secrets, keys, `.env`, or `*.pem`.
- CI is `npm ci` + typecheck + card-size only. Do **not** run Quickstart / live handshake in CI.
- Plain Node + TypeScript (`"type": "module"`). Avoid Next/Vite for the SDK WASM.

## Docs

- [Earn form notes](docs/EARN.md)
- [Google Doc outline](docs/GOOGLE-DOC-OUTLINE.md)
- [Bugs / pitfalls](docs/BUGS.md)
- [Handover](docs/HANDOVER.md)
