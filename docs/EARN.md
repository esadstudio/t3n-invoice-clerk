# Superteam Earn notes

Listing: [T3N Agent Build Challenge](https://superteam.fun/earn/listing/t3n-agent-build-challenge/) (290 USDC, due 2026-09-16).

**Do not Earn-submit yet.**

## Sponsor-known blocker

Official `fetchTrustedManifest("testnet")` on `@terminal3/t3n-sdk` **5.14.0** throws:

```
Error: Trust manifest at https://cn-api.sg.testnet.t3n.terminal3.io/api/trust-manifest is malformed.
```

Cause: testnet JSON is missing **`rtmr1_allowlist`** (SDK 5.14.0 requires it). Not a bad key. **Do not pin an older SDK for Earn.**

Local/demo only — not default, not CI — in `.env`:

```
T3N_UNSAFE_TRUST=1
```

Contact: [Telegram](https://t.me/terminal3developer) / `devrel@terminal3.io`. Full write-up: [BUGS.md](BUGS.md).

## Form fields

| Field | Value |
| --- | --- |
| GitHub | https://github.com/esadstudio/t3n-invoice-clerk |
| PR | https://github.com/esadstudio/t3n-invoice-clerk/pull/1 |
| `T3N_DID` | `did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9` |
| Keys | One claim. `T3N_API_KEY` == `AGENT_KEY` in local `.env` only |

Never paste keys or `.env` into the Earn form or Google Doc. Session DID from `Connected as:` / `whoami` wins if it differs.

## Local `.env` (not in git)

`D:\DEV\t3n-invoice-clerk\.env`

```
T3N_API_KEY=
AGENT_KEY=
T3N_DID=did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9
T3N_UNSAFE_TRUST=1
```

Last line is the **workaround**, not the product default.

## Demo commands (after workaround)

```bash
npm install
npx tsx src/quickstart.ts
# Connected as: did:t3n:…
```

Then [src/register-agent.md](../src/register-agent.md). Screenshots: [earn-screenshots/README.md](earn-screenshots/README.md). Paste-ready write-up: [GOOGLE-DOC-OUTLINE.md](GOOGLE-DOC-OUTLINE.md).
