# Contract notes (facts only)

Do not invent SDK or host methods. Everything below is cited from Terminal 3 docs.

## Tenant session (already in `src/quickstart.ts`)

After Quickstart prints `Connected as: did:t3n:…`, append [Set Up Dev Env](https://docs.terminal3.io/developers/adk/get-started/prerequisites/set-up-dev-env):

```typescript
import { TenantClient, getNodeUrl } from "@terminal3/t3n-sdk";

const tenant = new TenantClient({
  t3n,
  baseUrl: getNodeUrl(),
  tenantDid, // did.value from authenticate — never hardcode
});

await tenant.tenant.me();
```

`me()` is on `tenant.tenant`, not `tenant`. Always pass `baseUrl: getNodeUrl()`.

## Register + map

Documented in [Register your TEE contract](https://docs.terminal3.io/developers/adk/get-started/walkthrough/register-contract) and [Create Tenant KV Maps](https://docs.terminal3.io/developers/adk/tips/create-kv-maps):

- `tenant.contracts.register({ tail, version, wasm })` → `contract_id`
- Tail `invoice-ledger` becomes `z:<tid>:invoice-ledger`
- Tail regex: `/^[a-zA-Z0-9_-][a-zA-Z0-9_.-]{0,127}$/` — no `/`, do not include `z:<tid>:`
- Then `tenant.maps.create({ tail: "invoice-ledger", visibility: "private", writers: { only: [contractId] }, readers: { only: [contractId] } })`

Re-register at the same tail requires a **higher** `version`. Keep every `contract_id` — map ACLs are scoped to that numeric id.

## Invoke

[Invoke your TEE contract](https://docs.terminal3.io/developers/adk/get-started/walkthrough/invoke-contract) uses `T3nClient.executeAndDecode` with `contract_id` = `z:<tid>:invoice-ledger` and `function_name` `record-line` / `get-line`.

This repo aliases `T3N_AGENT_KEY` to `T3N_API_KEY`. Official samples name a separate `AGENT_KEY`; do not claim a second key for this challenge.

This stub has **no outbound HTTP**, so a user egress grant is not required for record/get. If you later add `http`, the **data owner** must sign `member-delegation-update` with `allowed_hosts` — the contract cannot authorize its own egress.

## Control-plane writes

Map owner can seed entries with `tenant.executeControl("map-entry-set", { map_name: tenant.canonicalName("invoice-ledger"), key, value })` ([Seed API key](https://docs.terminal3.io/developers/adk/tips/seed-api-key)). Prefer the contract `record-line` path so lines stay in the audited dispatch log (`client.getActivityLog()`).
