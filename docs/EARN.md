# Superteam Earn notes

Listing: [T3N Agent Build Challenge](https://superteam.fun/earn/listing/t3n-agent-build-challenge/) (290 USDC, due 2026-09-16).

## Form fields

| Field | Value |
| --- | --- |
| GitHub | https://github.com/esadstudio/t3n-invoice-clerk |
| `T3N_DID` (Earn form) | `did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9` |
| Keys | One claim. `T3N_API_KEY` only. `AGENT_KEY` is the same value in local `.env` |

`T3N_DID` is optional in `.env.example` as a reminder for the form. Quickstart does **not** read it. Do not hardcode it into runtime as an identity or secret. Session DID from `authenticate()` is the live value.

Never paste `T3N_API_KEY` / `AGENT_KEY` / `.env` into the Earn form or Google Doc.

## Local env (not in git)

Windows: `D:\DEV\t3n-invoice-clerk\.env`

```
T3N_API_KEY=
AGENT_KEY=
T3N_DID=did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9
```

Write-up skeleton: [GOOGLE-DOC-OUTLINE.md](GOOGLE-DOC-OUTLINE.md).
