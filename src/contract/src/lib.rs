//! Invoice ledger TEE stub — facts only, never payment authority.
//!
//! Host calls used here are the ones documented in
//! https://docs.terminal3.io/developers/adk/get-started/walkthrough/write-contract
//! and confirmed on `host:interfaces/kv-store` (`get`, `put`):
//!
//! - `tenant_context::tenant_did()` → raw bytes; hex-encode once for `z:<tid>:…`
//! - `kv_store::get(map, key)` / `kv_store::put(map, key, value)`
//! - `logging::info`
//!
//! This crate does not compile until `wit/deps/` is vendored from
//! `Terminal-3/z-tenant-flight`. Do not invent host interfaces.

#![allow(dead_code)]

extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub const CONTRACT_VERSION: &str = "0.1.0";
pub const MAP_TAIL: &str = "invoice-ledger";

/// Ledger line stored under `z:<tid>:invoice-ledger`. Facts only.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct InvoiceLine {
    pub id: String,
    pub amount: String,
    pub asset: String,
    pub payee: String,
    pub due_date: String,
    pub source_ref: String,
}

/// Documented map-path construction: hex-encode `tenant_did()` bytes once.
pub fn invoice_ledger_map(tid: &[u8]) -> String {
    format!("z:{}:{MAP_TAIL}", hex::encode(tid))
}

pub fn validate_line(line: &InvoiceLine) -> Result<(), String> {
    for (name, value) in [
        ("id", line.id.as_str()),
        ("amount", line.amount.as_str()),
        ("asset", line.asset.as_str()),
        ("payee", line.payee.as_str()),
        ("due_date", line.due_date.as_str()),
        ("source_ref", line.source_ref.as_str()),
    ] {
        if value.trim().is_empty() {
            return Err(format!("{name} is required"));
        }
    }
    Ok(())
}

pub fn encode_line(line: &InvoiceLine) -> Result<Vec<u8>, String> {
    serde_json::to_vec(line).map_err(|e| e.to_string())
}

pub fn decode_line(bytes: &[u8]) -> Result<InvoiceLine, String> {
    serde_json::from_slice(bytes).map_err(|e| e.to_string())
}

// Bindgen + Guest impl only on wasm32 after wit/deps is present.
// Native `cargo test` covers the JSON / path helpers above without the host ABI.
#[cfg(all(target_arch = "wasm32", feature = "wit-guest"))]
mod guest {
    use super::*;

    wit_bindgen::generate!({
        world: "invoice-ledger",
        path: "wit",
        additional_derives: [serde::Deserialize, serde::Serialize],
        generate_all,
    });

    use crate::host::{interfaces::kv_store, interfaces::logging, tenant::tenant_context};

    struct Component;

    fn map_name() -> String {
        invoice_ledger_map(&tenant_context::tenant_did())
    }

    impl exports::z::invoice_ledger::contracts::Guest for Component {
        fn record_line(
            req: exports::z::invoice_ledger::contracts::GenericInput,
        ) -> Result<Vec<u8>, String> {
            let input = req.input.ok_or("record-line: missing input")?;
            let line = decode_line(&input)?;
            validate_line(&line)?;
            let map = map_name();
            kv_store::put(&map, line.id.as_bytes(), &encode_line(&line)?)
                .map_err(|e| format!("kv put: {e}"))?;
            let _ = logging::info(&format!("recorded invoice line {}", line.id));
            serde_json::to_vec(&serde_json::json!({ "ok": true, "id": line.id }))
                .map_err(|e| e.to_string())
        }

        fn get_line(
            req: exports::z::invoice_ledger::contracts::GenericInput,
        ) -> Result<Vec<u8>, String> {
            let input = req.input.ok_or("get-line: missing input")?;
            let v: serde_json::Value =
                serde_json::from_slice(&input).map_err(|e| e.to_string())?;
            let id = v
                .get("id")
                .and_then(|x| x.as_str())
                .ok_or("get-line: missing id")?;
            let map = map_name();
            let bytes = kv_store::get(&map, id.as_bytes())
                .map_err(|e| format!("kv get: {e}"))?
                .ok_or("invoice line not found")?;
            Ok(bytes)
        }
    }

    export!(Component);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_path_is_z_tid_tail() {
        let path = invoice_ledger_map(&[0xab, 0xcd]);
        assert_eq!(path, "z:abcd:invoice-ledger");
    }

    #[test]
    fn rejects_empty_amount() {
        let line = InvoiceLine {
            id: "inv-1".into(),
            amount: "  ".into(),
            asset: "USDC".into(),
            payee: "did:t3n:example".into(),
            due_date: "2026-09-16".into(),
            source_ref: "po-99".into(),
        };
        assert!(validate_line(&line).is_err());
    }

    #[test]
    fn round_trips_json() {
        let line = InvoiceLine {
            id: "inv-1".into(),
            amount: "290".into(),
            asset: "USDC".into(),
            payee: "vendor-1".into(),
            due_date: "2026-09-16".into(),
            source_ref: "earn-listing".into(),
        };
        let decoded = decode_line(&encode_line(&line).unwrap()).unwrap();
        assert_eq!(decoded.amount, "290");
        assert_eq!(decoded.asset, "USDC");
    }
}
