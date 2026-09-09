import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

/**
 * Load KEY=VALUE pairs from a local .env if present.
 * Does not override existing process.env. Never logs values.
 */
export function loadLocalEnv(file = ".env"): void {
  const path = resolve(process.cwd(), file);
  if (!existsSync(path)) {
    return;
  }

  const text = readFileSync(path, "utf8");
  for (const raw of text.split(/\r?\n/)) {
    const line = raw.trim();
    if (!line || line.startsWith("#")) {
      continue;
    }
    const eq = line.indexOf("=");
    if (eq <= 0) {
      continue;
    }
    const key = line.slice(0, eq).trim();
    let value = line.slice(eq + 1).trim();
    if (
      (value.startsWith('"') && value.endsWith('"')) ||
      (value.startsWith("'") && value.endsWith("'"))
    ) {
      value = value.slice(1, -1);
    }
    if (process.env[key] === undefined) {
      process.env[key] = value;
    }
  }
}

/** One claim, one key. AGENT_KEY / T3N_AGENT_KEY alias T3N_API_KEY. */
export function aliasSingleKey(): void {
  if (!process.env.T3N_API_KEY && process.env.AGENT_KEY) {
    process.env.T3N_API_KEY = process.env.AGENT_KEY;
  }
  if (!process.env.T3N_API_KEY && process.env.T3N_AGENT_KEY) {
    process.env.T3N_API_KEY = process.env.T3N_AGENT_KEY;
  }
  const key = process.env.T3N_API_KEY;
  if (key) {
    process.env.AGENT_KEY = process.env.AGENT_KEY || key;
    process.env.T3N_AGENT_KEY = process.env.T3N_AGENT_KEY || process.env.AGENT_KEY;
  }
}
