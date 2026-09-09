# Superteam Earn — Google Doc (paste-ready)

Do **not** Earn-submit until screenshots exist or placeholders stay labeled. Do not paste secrets.

## Title

T3N Invoice Clerk — enterprise vendor invoice intake (ledger facts only)

## One-liner

A Terminal 3 testnet agent that records vendor invoice lines (amount, asset, payee, due date, source ref) inside a tenant TEE map. It never holds payment authority.

## Links

- Listing: https://superteam.fun/earn/listing/t3n-agent-build-challenge/
- Repo: https://github.com/esadstudio/t3n-invoice-clerk
- PR: https://github.com/esadstudio/t3n-invoice-clerk/pull/1
- Official Quickstart: https://docs.terminal3.io/developers/adk/get-started/quickstart
- Claim (once): https://go.terminal3.io/adk-community

## Identity

| Field | Value |
| --- | --- |
| `T3N_DID` (Earn form) | `did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9` |
| Keys | One claim. `T3N_API_KEY` = `AGENT_KEY` (local `.env` only) |

## Sponsor-known blocker (say this in the doc)

Official trust fails on current testnet:

```
Error: Trust manifest at https://cn-api.sg.testnet.t3n.terminal3.io/api/trust-manifest is malformed.
```

Cause: body missing `rtmr1_allowlist`; `@terminal3/t3n-sdk` 5.14.0 requires it. We did **not** pin an older SDK for Earn.

Local/demo workaround (not default, not CI): `T3N_UNSAFE_TRUST=1` in `.env`.

Contact: https://t.me/terminal3developer / `devrel@terminal3.io`

## What we shipped

1. Plain Node + TypeScript (`"type": "module"`) + `tsx` + `@terminal3/t3n-sdk`
2. Official Quickstart in `src/quickstart.ts` (`fetchTrustedManifest("testnet")` by default)
3. Agent card: `src/agent/agent-card.json` + `src/register-agent.md`
4. TEE stubs for `z:<tid>:invoice-ledger` — facts only, no payment

## Demo script (≤ 3 minutes)

1. Claim once → local `.env` with the same key in `T3N_API_KEY` and `AGENT_KEY`. Never commit `.env`.
2. Until sponsor fixes the manifest: `T3N_UNSAFE_TRUST=1` (workaround). Then `npm i` and `npx tsx src/quickstart.ts`.
3. Capture **`Connected as: did:t3n:…`** (see screenshots).
4. Agent card checklist (`src/register-agent.md`).
5. Show ledger stub: amount, asset, payee, due date, source ref. No payments.

## Screenshots

Paste images under these headings. If a file is missing, keep the heading and the command.

### 1. Quickstart — Connected as DID

- **File:** `docs/earn-screenshots/03-quickstart.png` — PLACEHOLDER (this agent cannot capture a live GUI)
- **How to capture (Windows):**
  1. In `D:\DEV\t3n-invoice-clerk\.env` set `T3N_API_KEY`, `AGENT_KEY` (same), and `T3N_UNSAFE_TRUST=1` while the sponsor bug stands.
  2. `npm install`
  3. `npx tsx src/quickstart.ts`
  4. Screenshot the line `Connected as: did:t3n:…` (Win+Shift+S). Save as `docs/earn-screenshots/03-quickstart.png`.
- **Expected DID (if session matches):** `did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9`

### 2. whoami

- **File:** `docs/earn-screenshots/01-whoami.png` — PLACEHOLDER
- **Command:** `npm run whoami` (needs key; may hit the same malformed-manifest error until the sponsor republishes `rtmr1_allowlist`)

### 3. Agent registry / host-card

- **File:** `docs/earn-screenshots/02-registry.png` — PLACEHOLDER
- **Command:** `npm exec -- t3n agent registry "$AGENT_DID" --env testnet` after `host-card` (same sponsor caveat)

## Architecture

- One claim key authenticates the tenant session; DID is read from the session
- Ledger facts persist under `z:<tid>:invoice-ledger` via `kv-store` (no payment APIs)
- Agent card is T3N-hosted discovery only

## Constraints

- Testnet / free tokens. No buy, no production, no payments
- Never commit `.env`
- Official trust stays the code default; unsafe hatch is documented workaround only
