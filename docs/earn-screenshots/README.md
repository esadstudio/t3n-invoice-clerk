# Screenshot capture (Earn)

This cloud agent **cannot** take live GUI screenshots on Esad’s Windows machine. Leave the PNG names empty until you capture locally. Do not commit `.env`. Do not Earn-submit without at least the Quickstart frame (or keep the Google Doc placeholder text).

## 03-quickstart.png — Connected as DID (required for the write-up)

```bat
cd /d D:\DEV\t3n-invoice-clerk
copy .env.example .env
rem Fill T3N_API_KEY and AGENT_KEY with the SAME claim key.
rem While the sponsor trust-manifest bug stands, also set:
rem T3N_UNSAFE_TRUST=1
npm install
npx tsx src/quickstart.ts
```

Expect: `Connected as: did:t3n:…`

Win+Shift+S → save as `docs/earn-screenshots/03-quickstart.png`.

If the DID is `did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9`, that matches the Earn-form `T3N_DID`. If not, use the printed session DID on the form.

Label the slide: official `fetchTrustedManifest` is blocked (missing `rtmr1_allowlist`); this capture used `T3N_UNSAFE_TRUST=1` if that line was set.

## 01-whoami.png / 02-registry.png

```bat
npm run whoami
npm exec -- t3n agent host-card --file src/agent/agent-card.json --env testnet
npm exec -- t3n agent registry %AGENT_DID% --env testnet
```

These use the upstream CLI and may hit the same malformed-manifest error until the sponsor republishes `rtmr1_allowlist`. Placeholders in the Google Doc are OK until then.
