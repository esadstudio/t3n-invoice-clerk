# Register Invoice Clerk (CLI checklist)

Official guide: [Register a Public Agent](https://docs.terminal3.io/developers/agents/register-agent).

**Use the local pinned CLI** (`@terminal3/t3n-sdk@5.2.0` from this repo). Do **not** run `npx @terminal3/t3n-sdk …` — that pulls latest 5.14.0 and `fetchTrustedManifest("testnet")` throws `Trust manifest … is malformed` (missing `rtmr1_allowlist`). See [docs/BUGS.md](../docs/BUGS.md).

```bash
npm exec -- t3n --help
# or: npm run whoami
```

One claim, one key. `T3N_API_KEY` and `AGENT_KEY` are the same value (local `.env`, never commit).

Earn-form reminder (`T3N_DID`, not used at runtime): `did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9`. After `whoami`, if the session DID matches, paste it into the card. Session DID always wins.

Official trust is the default. Do **not** set `T3N_UNSAFE_TRUST` for Earn.

## Checklist

- [ ] Official trust green: `npm run check:trust` (no key; no `T3N_UNSAFE_TRUST`)
- [ ] Local `.env` has `T3N_API_KEY=` and `AGENT_KEY=` (same). Never commit `.env`
- [ ] `npm run whoami` prints `did:t3n:…` (screenshot: `docs/earn-screenshots/01-whoami.png`)
- [ ] Card edited: `src/agent/agent-card.json` DID service = session DID, still &lt;16 KiB
- [ ] `npm exec -- t3n agent host-card --file src/agent/agent-card.json --env testnet`
- [ ] `npm exec -- t3n agent registry "$AGENT_DID" --env testnet` (screenshot: `docs/earn-screenshots/02-registry.png`)
- [ ] Do **not** Earn-submit until those screenshots exist from an official-trust run

## 0. One key, two aliases

```bash
# Claim once: https://go.terminal3.io/adk-community
# Or load from local .env (D:\DEV\t3n-invoice-clerk\.env) — never commit it
export T3N_API_KEY="<the key from the claim page>"
export AGENT_KEY="$T3N_API_KEY"   # same as T3N_API_KEY
```

The local `t3n` CLI reads `T3N_API_KEY` (or `--api-key`).

## 1. Confirm identity (testnet)

```bash
npm run whoami
# did:t3n:…

export AGENT_DID="$(npm exec -- t3n whoami --env testnet)"
```

Always read the DID back from `whoami`. Never hard-code or derive it from the key.

Screenshot placeholder: drop the terminal capture at `docs/earn-screenshots/01-whoami.png` when the official-trust run succeeds.

## 2. Scaffold / edit the card

Skeleton: [`src/agent/agent-card.json`](agent/agent-card.json) (ERC-8004 registration-v1, under 16 KiB).

```bash
npm exec -- t3n agent create-card \
  --did "$AGENT_DID" \
  --name "Invoice Clerk" \
  --description "Enterprise vendor invoice intake on Terminal 3. Ledger facts only — never payment authority." \
  --out src/agent/agent-card.json \
  --force
```

Set the `DID` service endpoint to `$AGENT_DID`. Keep the card under **16 KiB**.

## 3. Host the card on T3N (testnet)

```bash
npm exec -- t3n agent host-card \
  --file src/agent/agent-card.json \
  --env testnet
# card published: https://<node>/api/agent-card/did:t3n:…
```

If you prefer to skip hosting until a public A2A URL exists, stop after `whoami`. The card file in this repo is enough to keep building.

## 4. Verify (optional)

```bash
npm exec -- t3n agent registry "$AGENT_DID" --env testnet
```

Resolution is public — no key required to `GET /api/agent-card/<did>`.

Screenshot placeholder: `docs/earn-screenshots/02-registry.png`.

## Notes

- `--env testnet` on every network command. This repo is testnet / free-token only.
- Registration writes consume credits against **this** DID (the same identity as the tenant key).
- Official docs describe a second agent key with its own credits. This challenge uses one key by design.
- Hosting a card is identity + discovery. It does **not** grant contract access ([Member Delegation](https://docs.terminal3.io/developers/adk/get-started/member-delegation) is out of scope).
- Do not Earn-submit from this checklist until official-trust screenshots are filled in.
