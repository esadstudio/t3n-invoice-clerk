# Register Invoice Clerk (single-key checklist)

Official: [Register a Public Agent](https://docs.terminal3.io/developers/agents/register-agent).

One claim. `T3N_API_KEY` and `AGENT_KEY` are the **same** value in local `.env` (Windows: `D:\DEV\t3n-invoice-clerk\.env`). Never commit `.env`.

Card in repo: [`src/agent/agent-card.json`](agent/agent-card.json) (ERC-8004, &lt;16 KiB). DID service is the Earn-form reminder `did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9`. After `whoami`, replace it if the session DID differs.

## Sponsor bug (CLI hits it too)

`t3n whoami` / `host-card` use the same testnet trust-manifest. Until the sponsor adds `rtmr1_allowlist`, those commands may throw:

```
Error: Trust manifest at https://cn-api.sg.testnet.t3n.terminal3.io/api/trust-manifest is malformed.
```

`T3N_UNSAFE_TRUST=1` only affects **this repo’s** `src/quickstart.ts`, not the upstream CLI. Card file is complete without `host-card`. Contact: Telegram / `devrel@terminal3.io`. See [docs/BUGS.md](../docs/BUGS.md).

## Checklist

- [ ] `.env` has `T3N_API_KEY=` and `AGENT_KEY=` (same). Never commit
- [ ] Local/demo connect: `T3N_UNSAFE_TRUST=1` then `npx tsx src/quickstart.ts` → `Connected as: did:t3n:…`
- [ ] Screenshot that line → `docs/earn-screenshots/03-quickstart.png` ([how](../docs/earn-screenshots/README.md))
- [ ] If `whoami` works (after sponsor fix): `npm run whoami` → paste DID into the card if needed
- [ ] `npm exec -- t3n agent host-card --file src/agent/agent-card.json --env testnet` (when CLI trust works)
- [ ] Do **not** Earn-submit until you have a `Connected as:` capture (workaround labeled) or an official-trust capture

## Commands

```bash
export T3N_API_KEY="<claim key>"
export AGENT_KEY="$T3N_API_KEY"

npm install
npx tsx src/quickstart.ts

# After sponsor republishes rtmr1_allowlist:
npm run whoami
export AGENT_DID="$(npm exec -- t3n whoami --env testnet)"
npm exec -- t3n agent create-card --did "$AGENT_DID" --name "Invoice Clerk" \
  --description "Enterprise vendor invoice intake. Ledger facts only." \
  --out src/agent/agent-card.json --force
npm exec -- t3n agent host-card --file src/agent/agent-card.json --env testnet
npm exec -- t3n agent registry "$AGENT_DID" --env testnet
```

`--env testnet` only. No second key. Hosting a card is discovery, not payment authority.
