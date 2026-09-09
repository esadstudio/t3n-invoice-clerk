# Handover — T3N Invoice Clerk

Operations-oriented. One key. Testnet only. No payments.

## Status

Scaffold is ready to merge to `main`. `npm install` succeeds. Quickstart matches the official ADK sample plus a single-key alias. Agent card and TEE stubs are in-repo. Live handshake / `host-card` need the claim key in the **local** shell — never in git.

## 5-minute path (operator)

1. Claim **once**: https://go.terminal3.io/adk-community (key shown once).
2. Put the **same** claim key in a local `.env` (Windows: `D:\DEV\t3n-invoice-clerk\.env`):

   ```
   T3N_API_KEY=
   AGENT_KEY=
   T3N_DID=did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9
   ```

   `T3N_DID` is optional (Earn form). Leave the two key lines empty in git; fill keys only in local `.env`.

   Never commit `.env`. Then:

   ```bash
   npm install
   npm run check:trust
   npx tsx src/quickstart.ts
   ```

   CI runs typecheck + card-size + official `check:trust` (public manifest only). Do **not** run Quickstart in CI. SDK is pinned to **5.2.0** so official trust works. Leave `T3N_UNSAFE_TRUST` unset for Earn. See [BUGS.md](BUGS.md).

3. Expect `Connected as: did:t3n:…`.
4. Agent card: [src/register-agent.md](../src/register-agent.md) — `npm run whoami` → create-card → host-card `--env testnet`. Use local `t3n`, not unpinned `npx @terminal3/t3n-sdk`.
5. Earn form: paste `T3N_DID` `did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9` — see [EARN.md](EARN.md). Do **not** Earn-submit until official-trust screenshots exist.

## Files

| Path | Role |
| --- | --- |
| `src/quickstart.ts` | Official tenant handshake |
| `src/register-agent.md` | CLI checklist, same key |
| `src/agent/agent-card.json` | ERC-8004 card (&lt;16 KiB) |
| `src/contract/` | Invoice ledger TEE stubs + notes |
| `.env.example` | Empty `T3N_API_KEY=` / `AGENT_KEY=`. Optional `T3N_DID=` (Earn form, not a secret). |
| `docs/EARN.md` | Earn form DID + listing notes |
| `docs/GOOGLE-DOC-OUTLINE.md` | Earn write-up skeleton |
| `docs/BUGS.md` | Pitfalls |

## Next tasks (not in this PR)

1. Run Quickstart with the real key; paste the printed DID into the Earn form if it differs from the documented one (session DID wins).
2. `host-card` when you have a public A2A URL, or leave placeholders.
3. Vendor `wit/deps` from `Terminal-3/z-tenant-flight`, `rustup target add wasm32-wasip2`, build, `tenant.contracts.register` + `maps.create` for `invoice-ledger`.
4. Record a ≤3 min demo from the Google Doc outline.
5. Submit on Superteam Earn before 2026-09-16.

## Hard rules

- Never commit `.env`, `*.pem`, keys, or WASM cache
- No buy, no production, no payment authority
- Do not invent T3N APIs — [llms.txt](https://docs.terminal3.io/llms.txt) and [Reference](https://docs.terminal3.io/developers/adk/reference)
- Developer Telegram: https://t.me/terminal3developer — `devrel@terminal3.io`
