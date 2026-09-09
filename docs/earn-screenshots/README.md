# Earn screenshot placeholders

Fill these from a **local official-trust** run (`T3N_UNSAFE_TRUST` unset). Do not commit `.env`. Do not Earn-submit until the files exist.

| File | Capture |
| --- | --- |
| `01-whoami.png` | `npm run whoami` → `did:t3n:…` |
| `02-registry.png` | `npm exec -- t3n agent registry "$AGENT_DID" --env testnet` |
| `03-quickstart.png` | `npm run quickstart` → `Connected as: did:t3n:…` |

Expected Earn-form DID (if session matches): `did:t3n:53a6ae350a77d94b524b7ce345205a7d6afdf7c9`.
