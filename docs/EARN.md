# Superteam Earn notes

Listing: [T3N Agent Build Challenge](https://superteam.fun/earn/listing/t3n-agent-build-challenge/) (290 USDC, due 2026-09-16).

**Do not Earn-submit yet.** Official trust path is the default (`fetchTrustedManifest("testnet")` on `@terminal3/t3n-sdk@5.2.0`). Do **not** rely on `T3N_UNSAFE_TRUST` for the submission.

## Form fields

| Field | Value |
| --- | --- |
| GitHub | https://github.com/esadstudio/t3n-invoice-clerk |
| PR | https://github.com/esadstudio/t3n-invoice-clerk/pull/1 |
| `T3N_DID` (Earn form) | `did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9` |
| Keys | One claim. `T3N_API_KEY` only. `AGENT_KEY` is the same value in local `.env` |

`T3N_DID` is optional in `.env.example` as a reminder for the form. Quickstart does **not** read it. Do not hardcode it into runtime as an identity or secret. Session DID from `authenticate()` / `whoami` is the live value.

Never paste `T3N_API_KEY` / `AGENT_KEY` / `.env` into the Earn form or Google Doc.

## Official trust (Earn default)

```bash
npm install
npm run check:trust
# Official trust path OK
npm run quickstart
# Connected as: did:t3n:…
```

Pinned SDK **5.2.0** (exact). 5.3.0+ reject the live testnet manifest (missing `rtmr1_allowlist`). See [BUGS.md](BUGS.md). Use `npm exec -- t3n …` / `npm run whoami` — not `npx @terminal3/t3n-sdk`.

`T3N_UNSAFE_TRUST=1` stays local/debug only. Not for Earn.

## Local env (not in git)

Windows: `D:\DEV\t3n-invoice-clerk\.env`

```
T3N_API_KEY=
AGENT_KEY=
T3N_DID=did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9
```

Leave `T3N_UNSAFE_TRUST` unset.

## Screenshots

Placeholders: [earn-screenshots/README.md](earn-screenshots/README.md). Paste into the Google Doc when captured.

Write-up skeleton: [GOOGLE-DOC-OUTLINE.md](GOOGLE-DOC-OUTLINE.md).
