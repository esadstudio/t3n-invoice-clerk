# Invoice ledger TEE stubs

Facts only. **Never payment authority.** Testnet / free-token path.

These files follow the official walkthrough — [Write your TEE contract](https://docs.terminal3.io/developers/adk/get-started/walkthrough/write-contract) — and stop at documented APIs. They are stubs, not a registered WASM build.

## What this contract is allowed to store

Ledger lines under `z:<tid>:invoice-ledger` (tail `invoice-ledger`). Each line is JSON:

| Field | Meaning |
| --- | --- |
| `amount` | Invoice amount as a string (exact decimal the vendor sent) |
| `asset` | Asset / currency code (e.g. `USDC`) |
| `payee` | Payee identifier (DID, vendor id, or account ref — a fact, not a payment instruction) |
| `due_date` | ISO-8601 date |
| `source_ref` | Upstream invoice / PO / email message id |

No bank numbers, no card PAN, no payment intent, no `outbox`, no `ap2/mandate`, no `signing`.

## z-namespace (documented)

`tid` is the 40-hex suffix of the tenant DID (`did:t3n:<tid>`). The host builds canonical names as `z:<tid>:<tail>`. Pass **only the tail** to the TypeScript SDK (`invoice-ledger`, `secrets`). Inside WASM, `kv-store` takes the **full** `z:<tid>:…` name. Hex-encode `tenant_context::tenant_did()` once — it returns raw bytes.

See [What is z-namespace?](https://docs.terminal3.io/developers/adk/get-started/walkthrough/what-is-z-namespace) and [Storage Namespaces](https://docs.terminal3.io/t3n/how-t3n-works/z-namespace).

## Host capabilities this stub imports

Only interfaces named in the official walkthrough / Host API table:

- `host:tenant/tenant-context` — `tenant_did()`
- `host:interfaces/logging`
- `host:interfaces/kv-store` — `get` / `put` (confirmed in `Terminal-3/z-tenant-flight` WIT)

Do not import `http`, `http-with-placeholders`, `outbox`, or `ap2` here. Invoice Clerk does not dial vendors and does not pay.

## Build / register (when you are ready)

This folder does **not** vendor `wit/deps/`. Clone the reference crate as a **sibling** of this Node app and copy its vendored host WIT packages — do not invent ABI:

```bash
# from the repo parent (not inside this Node project)
git clone https://github.com/Terminal-3/z-tenant-flight.git
# then copy z-tenant-flight/wit/deps/ into src/contract/wit/deps/
```

Toolchain (from [Set Up Dev Env](https://docs.terminal3.io/developers/adk/get-started/prerequisites/set-up-dev-env) and [Build](https://docs.terminal3.io/developers/adk/get-started/walkthrough/build-contract)):

```bash
rustup target add wasm32-wasip2
cd src/contract
cargo build --target wasm32-wasip2 --release
# artifact: target/wasm32-wasip2/release/z_invoice_ledger.wasm
```

Register from the same authenticated `TenantClient` as Quickstart + Set Up Dev Env. Documented call:

```typescript
const result = await tenant.contracts.register({
  tail: "invoice-ledger",
  version: "0.1.0",
  wasm: wasmBytes,
});
const contractId = result.contract_id;
```

Create the map **after** you have `contractId` ([Create Tenant KV Maps](https://docs.terminal3.io/developers/adk/tips/create-kv-maps)):

```typescript
await tenant.maps.create({
  tail: "invoice-ledger",
  visibility: "private",
  writers: { only: [contractId] },
  readers: { only: [contractId] },
});
```

`readers` must be set — the KV governor defaults to deny.

## Exported functions (intended)

| WIT export | Input JSON | Output |
| --- | --- | --- |
| `record-line` | `{ id, amount, asset, payee, due_date, source_ref }` | `{ ok, id }` |
| `get-line` | `{ id }` | the stored line or an error string |

Each export takes `generic-input` and returns `result<list<u8>, string>` — the official 3-field envelope. There is no `dispatch` function.

## What this scaffold does not do

- Compile or register WASM (needs Rust + vendored host WIT + `T3N_API_KEY`)
- Seed secrets (Invoice Clerk has none)
- Member delegation / agent `execute` (separate key is **not** required; one key aliases)
- Payments
