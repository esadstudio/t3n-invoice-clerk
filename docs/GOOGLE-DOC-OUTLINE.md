# Superteam Earn — Google Doc outline

Use this as the submission write-up. Do not paste secrets. DID below is for the Earn form only.

## Title

T3N Invoice Clerk — enterprise vendor invoice intake (ledger facts only)

## One-liner

A Terminal 3 testnet agent that records vendor invoice lines (amount, asset, payee, due date, source ref) inside a tenant TEE map. It never holds payment authority.

## Listing

- Bounty: [T3N Agent Build Challenge](https://superteam.fun/earn/listing/t3n-agent-build-challenge/) (290 USDC, due 2026-09-16)
- Repo: https://github.com/esadstudio/t3n-invoice-clerk
- Docs: https://docs.terminal3.io/developers/adk/get-started/quickstart
- Claim (once): https://go.terminal3.io/adk-community

## Identity (Earn form)

| Field | Value |
| --- | --- |
| Tenant / agent DID | `did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9` |
| Keys | One claim. `T3N_API_KEY` == `T3N_AGENT_KEY` |

Do not put the API key in the Google Doc.

## What we shipped

1. Plain Node + TypeScript (`"type": "module"`) + `tsx` + `@terminal3/t3n-sdk` — avoids Next/Vite WASM breakage.
2. Official Quickstart in `src/quickstart.ts`: `setEnvironment("testnet")`, `loadWasmComponent`, `fetchTrustedManifest("testnet")`, `eth_get_address`, `metamask_sign`, handshake, authenticate, prints `Connected as: did:t3n:…`.
3. Agent card skeleton (`src/agent/agent-card.json`, ERC-8004, &lt;16 KiB).
4. CLI checklist (`src/register-agent.md`) using the same key.
5. TEE walkthrough stubs for `z:<tid>:invoice-ledger` — documented host APIs only.

## Demo script (≤ 3 minutes)

1. Claim page once → export `T3N_API_KEY` and `T3N_AGENT_KEY="$T3N_API_KEY"`.
2. `npm i` then `npx tsx src/quickstart.ts` → `Connected as: did:t3n:…`.
3. Show agent card + register checklist (whoami / create-card / host-card testnet).
4. Show contract stub: facts only, no payment, map `invoice-ledger`.
5. State constraints: testnet, free tokens, no production, no buy.

## Architecture (3 bullets)

- Tenant session authenticates with the single claim key; DID is read from the session, never derived.
- Ledger lines persist in a private tenant map `z:<tid>:invoice-ledger` via `kv-store` inside the TEE.
- Agent card is T3N-hosted discovery (identity). Delegation / payments are out of scope.

## Constraints we honored

- Never commit `.env` or keys
- No buy, no production, no payments
- One API key; do not block on a second agent key
- Do not invent Terminal 3 APIs

## Links to paste

- README 5-minute path
- `docs/BUGS.md`
- `docs/HANDOVER.md`
- Official register-agent and write-contract pages
