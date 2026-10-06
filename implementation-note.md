# Implementation notes

## README (2026-10-06)

- Added `README.md` with tool overview, config layering, and startup paths: local HTTP, stdio + Cursor example, cargo release, Docker Compose, plain `docker run`, HTTP client URL.

## Docker (2026-10-06)

- **HTTP only in containers:** `transport_mode = "http"` in `config/docker.toml` (default `APP_ENV=docker`). Stdio MCP expects a parent process on stdin/stdout and is not suitable for a detached server container.
- **Secrets:** API key via `TYPESAFE_API_KEY` in `.env` → `APP_TYPESAFE_API_KEY`. Non-secrets in committed `config/docker.toml`.
- **Healthcheck:** `wget --spider` against `/mcp/jev/v1/` (needs a reachable HTTP response; adjust if rmcp returns non-2xx on GET root).
- **OpenSSL:** build stage uses `libssl-dev`; runtime uses `libssl3` for `reqwest` native-tls.

## Sample config / gitignore (2026-10-02)

- Added `config/dev.toml.example` with placeholder TypeSafe fields; real values live in `config/{APP_ENV}.toml` (see `Settings::load`).
- `.gitignore` excludes `config/dev.toml`, `config/prod.toml`, and `config/local.toml` so keys are not committed. `config/default.toml` and `*.example` stay tracked.

## Score criteria bounds (2026-09-30)

**Source:** [TypeSafe API — Score](https://docs.typesafe.ai/api#score)

- API: `criteria` is a required ordered array of level descriptions; **at least 2**, **at most 10**.
- Enforced two ways:
  1. **JSON Schema** via `#[schemars(length(min = 2, max = 10))]` so MCP clients see `minItems` / `maxItems`.
  2. **Runtime** check in `run_jev_score` (schema is advisory for many MCP clients).
- Removed `#[serde(default)]` on `criteria` — empty default would violate the min of 2; the field is required.

**Tool copy:** Title/description rewritten from the Score docs (“rates along a rubric / ordered levels”, probability-weighted value that can land between levels). Dropped the incorrect “score in [0, 1]” claim (that’s Noul; Score is weighted across level indices `0..n-1`).

**Also:**

- Added `TypesafeError::Validation` for local param checks.
- Wired missing `jev_score` / `jev_choice` imports in `server.rs` and filled in missing imports in `jev_score.rs`.
- Minimal `jev_choice` path fix earlier so the crate compiled (`TypesafeState` import).

## Choice criteria (2026-09-30)

**Source:** [TypeSafe API — Choice](https://docs.typesafe.ai/api#choice)

- API: `criteria` is a **required** `map<option, string | object | array | null>`; use `null` when an option needs no extra detail; **max 255** options.
- Params type is now `HashMap<String, Option<TypesafeState>>` (matches `Question::Choice` and the API).
- Removed wrapping `Option<...>` / `#[serde(default)]` — the map itself is required.
- Enforced **2–255** options via `#[schemars(length(min = 2, max = 255))]` and a runtime check in `run_jev_choice`.
- **Min 2 is a local requirement**, not stated in the public Choice API (which only documents max 255). Zero or one option is not a meaningful Choice, so we reject before calling the API and then document that bound in the tool description.
- Tool title/description updated to match Choice docs (chosen option + distribution), not a fake “[0, 1] classification”.

## MCP tool trace logging (2026-10-06)

End-to-end traces are JSON lines on **stderr** only (stdio MCP keeps stdout for the protocol). `Settings.log_level` (default `info` in `config/default.toml`) is applied via `tracing-subscriber` `EnvFilter`; there is no `RUST_LOG` merge.

Each MCP tool call gets a UUID v4 `invocation_id` shared across:

| `event`             | Fields                                                                                     |
| ------------------- | ------------------------------------------------------------------------------------------ |
| `mcp_tool_start`    | `invocation_id`, `tool_name`, `mcp_input`                                                  |
| `jev_request`       | `invocation_id`, `tool_name`, `jev_types`, `jev_request`                                   |
| `jev_response`      | `invocation_id`, `jev_response`                                                            |
| `jev_error`         | `invocation_id`, `error` (HTTP/API failures inside `TypesafeClient::evaluate`)             |
| `mcp_tool_complete` | `invocation_id`, `tool_name`, `tool_result` (`ok` payload or `err` message), `duration_ms` |

`jev_types` is `"noul"` / `"choice"` / `"score"` for single-question tools, and a map of question id → type for `jev_decide`. Local validation failures (score/choice/decide bounds) emit `mcp_tool_start` then `mcp_tool_complete` with `tool_result.err` and **no** `jev_request`.

The TypeSafe API key is not logged (header only). Request `state` and instructions **are** logged in full; treat stderr as sensitive.

The JSON subscriber flattens event fields. Nested payloads live under `payload` as a JSON string. Filter one call:

```bash
jq -c 'select(.invocation_id=="<id>") | {event, payload}'
```

Or parse nested fields:

```bash
jq 'select(.event=="jev_request") | .payload | fromjson | {invocation_id, jev_types, jev_request}'
```
