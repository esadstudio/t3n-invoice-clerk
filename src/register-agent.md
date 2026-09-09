# Register Invoice Clerk (CLI checklist)

Official guide: [Register a Public Agent](https://docs.terminal3.io/developers/agents/register-agent).

This project uses **one API key** for both tenant handshake and agent CLI. Do not claim a second key. `T3N_API_KEY` and `T3N_AGENT_KEY` are the same value.

The DID printed by `whoami` is for the Superteam Earn form later. It is not required to install, build, or run `src/quickstart.ts`.

## 0. One key, two aliases

```bash
# Claim once: https://go.terminal3.io/adk-community
export T3N_API_KEY="<the key from the claim page>"
export T3N_AGENT_KEY="$T3N_API_KEY"   # same as T3N_API_KEY
# AGENT_KEY is another alias some samples use — also the same value if set
```

The CLI that ships with `@terminal3/t3n-sdk` reads `T3N_API_KEY` (or `--api-key`). It never needs a separate `AGENT_KEY` in this repo.

## 1. Confirm identity (testnet)

```bash
npx @terminal3/t3n-sdk whoami --env testnet
# did:t3n:…

export AGENT_DID="$(npx @terminal3/t3n-sdk whoami --env testnet)"
```

Always read the DID back from `whoami`. Never hard-code or derive it from the key. The Earn-form DID for this tenant (documentation only) is `did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9`.

## 2. Scaffold / edit the card

A skeleton already lives at `src/agent/agent-card.json` (ERC-8004 registration-v1, under 16 KiB).

To regenerate from the CLI:

```bash
npx @terminal3/t3n-sdk agent create-card \
  --did "$AGENT_DID" \
  --name "Invoice Clerk" \
  --description "Enterprise vendor invoice intake on Terminal 3. Ledger facts only — never payment authority." \
  --out src/agent/agent-card.json \
  --force
```

Then set the `DID` service endpoint to `$AGENT_DID`. Keep the card under **16 KiB** — T3N rejects larger bodies on `host-card`.

## 3. Host the card on T3N (testnet)

```bash
npx @terminal3/t3n-sdk agent host-card \
  --file src/agent/agent-card.json \
  --env testnet
# card published: https://<node>/api/agent-card/did:t3n:…
```

If you prefer to skip hosting until a public A2A URL exists, stop after `whoami`. The card file in this repo is enough to keep building.

## 4. Verify (optional)

```bash
npx @terminal3/t3n-sdk agent registry "$AGENT_DID" --env testnet
```

Resolution is public — no key required to `GET /api/agent-card/<did>`.

## Notes

- `--env testnet` on every network command. This repo is testnet / free-token only.
- Registration writes consume credits against **this** DID (the same identity as the tenant key).
- Official docs describe a second agent key with its own credits. This challenge uses one key by design. Do not generate a key outside the claim page (zero credits).
- Hosting a card is identity + discovery. It does **not** grant contract access. That is a later [Member Delegation](https://docs.terminal3.io/developers/adk/get-started/member-delegation) step, signed by the data owner — not in scope for this scaffold.
