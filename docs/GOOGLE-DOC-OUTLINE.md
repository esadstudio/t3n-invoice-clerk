# Superteam Earn — Google Doc outline

Use this as the submission write-up. **Do not Earn-submit until official-trust screenshots are in.** Do not paste secrets. DID below is for the Earn form only.

## Title

T3N Invoice Clerk — enterprise vendor invoice intake (ledger facts only)

## One-liner

A Terminal 3 testnet agent that records vendor invoice lines (amount, asset, payee, due date, source ref) inside a tenant TEE map. It never holds payment authority.

## Listing

- Bounty: [T3N Agent Build Challenge](https://superteam.fun/earn/listing/t3n-agent-build-challenge/) (290 USDC, due 2026-09-16)
- Repo: https://github.com/esadstudio/t3n-invoice-clerk
- PR: https://github.com/esadstudio/t3n-invoice-clerk/pull/1
- Docs: https://docs.terminal3.io/developers/adk/get-started/quickstart
- Claim (once): https://go.terminal3.io/adk-community

## Identity (Earn form)

| Field | Value |
| --- | --- |
| `T3N_DID` (Earn form) | `did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9` |
| Keys | One claim. `T3N_API_KEY` == `AGENT_KEY` (local `.env` only — not in git) |

Do not put the API key in the Google Doc.

## Official trust (must be the demo path)

- Default: `fetchTrustedManifest("testnet")` on **pinned** `@terminal3/t3n-sdk@5.2.0`
- `npm run check:trust` — no key, no `T3N_UNSAFE_TRUST`
- Do **not** demo `T3N_UNSAFE_TRUST=1` for Earn (local/debug hatch only)
- If sponsor republishes `/api/trust-manifest` with `rtmr1_allowlist`, we can bump the SDK — see `docs/BUGS.md`

## What we shipped

1. Plain Node + TypeScript (`"type": "module"`) + `tsx` + pinned `@terminal3/t3n-sdk@5.2.0`
2. Official Quickstart in `src/quickstart.ts`: `setEnvironment("testnet")`, `loadWasmComponent`, `fetchTrustedManifest("testnet")`, handshake, authenticate, prints `Connected as: did:t3n:…`
3. Agent card skeleton (`src/agent/agent-card.json`, ERC-8004, &lt;16 KiB)
4. CLI checklist (`src/register-agent.md`) using the **local** `t3n` binary
5. TEE walkthrough stubs for `z:<tid>:invoice-ledger` — documented host APIs only

## Demo script (≤ 3 minutes)

1. Claim page once → local `.env` with `T3N_API_KEY=` and `AGENT_KEY=` (same value). Never commit `.env`. `T3N_UNSAFE_TRUST` unset.
2. `npm i` → `npm run check:trust` → `npx tsx src/quickstart.ts` → `Connected as: did:t3n:…`
3. `npm run whoami` + card + `host-card --env testnet` from `src/register-agent.md`
4. Show contract stub: facts only, no payment, map `invoice-ledger`
5. State constraints: testnet, free tokens, no production, no buy

## Screenshots (placeholders until official-trust live run)

| Slot | File | Status |
| --- | --- | --- |
| Whoami / DID | `docs/earn-screenshots/01-whoami.png` | placeholder — paste after official-trust run |
| Agent registry | `docs/earn-screenshots/02-registry.png` | placeholder |
| Quickstart connected | `docs/earn-screenshots/03-quickstart.png` | placeholder |

## Architecture (3 bullets)

- Tenant session authenticates with the single claim key; DID is read from the session, never derived
- Ledger lines persist in a private tenant map `z:<tid>:invoice-ledger` via `kv-store` inside the TEE
- Agent card is T3N-hosted discovery (identity). Delegation / payments are out of scope

## Constraints we honored

- Never commit `.env` or keys
- No buy, no production, no payments
- One API key; do not block on a second agent key
- Official trust path for Earn — no unsafe hatch in the submission demo
- Do not invent Terminal 3 APIs

## Links to paste

- README 5-minute path
- `docs/EARN.md`
- `docs/BUGS.md`
- `docs/HANDOVER.md`
- Official register-agent and write-contract pages
