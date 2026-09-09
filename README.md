# T3N Invoice Clerk

Enterprise vendor invoice intake on [Terminal 3](https://docs.terminal3.io/developers/adk/get-started/quickstart) (T3N). **Ledger lines only — never payment authority.** Testnet. Free-token path.

Listing: [T3N Agent Build Challenge](https://superteam.fun/earn/listing/t3n-agent-build-challenge/) (290 USDC, due 2026-09-16).

**Do not Earn-submit yet.**

## Sponsor-known blocker (official trust)

Default Quickstart calls `fetchTrustedManifest("testnet")` and fails on current testnet:

```
Error: Trust manifest at https://cn-api.sg.testnet.t3n.terminal3.io/api/trust-manifest is malformed.
```

Cause: the HTTP 200 body has `cluster`, `version`, `peer_ids`, `rtmr3_allowlist`, `signed_at`, `signature` — **no `rtmr1_allowlist`**. `@terminal3/t3n-sdk` **5.14.0** requires that field. This is a **sponsor-side API/SDK mismatch**, not a bad API key. We are **not** pinning an older SDK for Earn.

**Local/demo workaround only** (not default, not CI): add to `.env` (never commit `.env`):

```
T3N_UNSAFE_TRUST=1
```

That sets `{ unsafe_trust_server: true }` and prints a loud warning. Contact: [developer Telegram](https://t.me/terminal3developer) / `devrel@terminal3.io`. Details: [docs/BUGS.md](docs/BUGS.md).

## 5-minute path

One claim. One key. `T3N_DID` is for the Earn form later — not required to install.

### 1. Claim (once)

[https://go.terminal3.io/adk-community](https://go.terminal3.io/adk-community) — key shown once.

### 2. Local `.env`

Copy `.env.example` → `.env` (Windows: `D:\DEV\t3n-invoice-clerk\.env`):

```
T3N_API_KEY=
AGENT_KEY=
T3N_DID=did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9
```

Same claim key in `T3N_API_KEY` and `AGENT_KEY`. **Never commit `.env` or `*.pem`.**

Until the sponsor republishes `rtmr1_allowlist`, add `T3N_UNSAFE_TRUST=1` for a local/demo connect only.

### 3. Install and connect

```bash
npm install
npx tsx src/quickstart.ts
# Connected as: did:t3n:…
```

Official code path: `setEnvironment("testnet")`, `loadWasmComponent`, `fetchTrustedManifest("testnet")` unless `T3N_UNSAFE_TRUST=1`. Capture the `Connected as:` line — see [docs/earn-screenshots/README.md](docs/earn-screenshots/README.md).

### 4. Agent card

[`src/agent/agent-card.json`](src/agent/agent-card.json) (&lt;16 KiB). Checklist: [`src/register-agent.md`](src/register-agent.md).

### 5. Contract notes (stubs)

Invoice facts under `z:<tid>:invoice-ledger`. See [`src/contract/`](src/contract/).

## Earn-form DID (documentation only)

`T3N_DID=did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9` — [docs/EARN.md](docs/EARN.md). Runtime identity is `authenticate()` / `whoami`.

## Constraints

- Testnet only. No production. No buy. No payments.
- Never commit secrets, keys, `.env`, or `*.pem`.
- CI: `npm ci` + typecheck + card-size. No Quickstart. No `T3N_UNSAFE_TRUST`.

## Docs

- [Earn notes](docs/EARN.md)
- [Google Doc outline](docs/GOOGLE-DOC-OUTLINE.md)
- [Bugs](docs/BUGS.md)
- [Handover](docs/HANDOVER.md)
