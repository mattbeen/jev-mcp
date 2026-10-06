# jev-mcp-rust

A [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) server that exposes **Jev** judgment tools backed by the [TypeSafe System One API](https://docs.typesafe.ai/api). Host applications (for example Cursor) call these tools to run calibrated Noul, Score, Choice, and batch Decide evaluations over shared state and instructions.

## Tools

| Tool         | Purpose                                                         |
| ------------ | --------------------------------------------------------------- |
| `jev_noul`   | Binary proposition — estimated probability a statement is true  |
| `jev_score`  | Ordered rubric — score on 2–10 levels with distribution         |
| `jev_choice` | Pick one option from 2–255 labeled choices                      |
| `jev_decide` | Batch 1–32 mixed Noul / Score / Choice questions in one request |

The model name is configured on the server (`model` in config), not per tool call.

## Prerequisites

- **Rust** 1.85+ (edition 2024) for local builds
- **TypeSafe API key** — [TypeSafe](https://typesafe.ai) account with access to System One
- **Docker** (optional) for container deployment

## Configuration

Settings are loaded in order (later wins):

1. `config/default.toml` — host, port, HTTP path, timeouts, default transport
2. `config/{APP_ENV}.toml` — environment overlay (`APP_ENV` defaults to `dev`)
3. Environment variables with prefix `APP_` (for example `APP_TYPESAFE_API_KEY`)

Copy the sample env file and add secrets locally:

```bash
cp config/dev.toml.example config/dev.toml
# Edit config/dev.toml — set typesafe_api_key (this file is gitignored)
```

Committed templates:

| File                      | Use                                                              |
| ------------------------- | ---------------------------------------------------------------- |
| `config/dev.toml.example` | Local development                                                |
| `config/docker.toml`      | Default overlay when `APP_ENV=docker` (used in the Docker image) |
| `.env.example`            | Docker Compose secrets (`TYPESAFE_API_KEY`)                      |

### Main settings

| Key                             | Description                                     |
| ------------------------------- | ----------------------------------------------- |
| `transport_mode`                | `stdio` or `http`                               |
| `service_host` / `service_port` | HTTP bind address (default `0.0.0.0:8766`)      |
| `app_prefix`                    | MCP HTTP mount path (default `/mcp/jev/v1`)     |
| `typesafe_api_url`              | System One API base URL                         |
| `typesafe_api_key`              | Your API key                                    |
| `model`                         | Server-side model id (for example `jev-latest`) |
| `log_level`                     | Tracing filter (for example `info`, `debug`)    |

Run from the **repository root** so relative `config/` paths resolve correctly.

---

## How to run

### 1. Local development (HTTP)

Default config uses streamable HTTP — suitable for remote MCP clients and matches Docker.

```bash
cp config/dev.toml.example config/dev.toml
# Set typesafe_api_key in config/dev.toml

cargo run
```

Server listens at:

```text
http://127.0.0.1:8766/mcp/jev/v1
```

Release build:

```bash
cargo build --release
./target/release/jev-mcp-rust   # Windows: target\release\jev-mcp-rust.exe
```

Override via environment without editing files:

```bash
export APP_TYPESAFE_API_KEY="your_key"
export APP_LOG_LEVEL=debug
cargo run
```

### 2. Local stdio (spawned by an MCP host)

Use when the host starts the server as a subprocess and talks over stdin/stdout (typical desktop MCP setup).

Set transport to stdio in `config/dev.toml`:

```toml
transport_mode = "stdio"
```

Or for one run:

```bash
# Linux/macOS
APP_TRANSPORT_MODE=stdio cargo run

# Windows PowerShell
$env:APP_TRANSPORT_MODE = "stdio"; cargo run
```

**Cursor** (`mcp.json` or MCP settings) example:

```json
{
  "mcpServers": {
    "jev": {
      "command": "C:\\path\\to\\jev-mcp-rust\\target\\release\\jev-mcp-rust.exe",
      "cwd": "C:\\path\\to\\jev-mcp-rust",
      "env": {
        "APP_TRANSPORT_MODE": "stdio",
        "APP_TYPESAFE_API_KEY": "your_key"
      }
    }
  }
}
```

Use `cargo run` as the command during development if you prefer; keep `cwd` at the repo root.

### 3. Docker Compose (HTTP)

```bash
cp .env.example .env
# Set TYPESAFE_API_KEY in .env

docker compose up --build
```

Same MCP URL as local HTTP: `http://localhost:8766/mcp/jev/v1` (override host port with `HOST_PORT` in `.env`).

Optional: mount `config/dev.toml` instead of `.env`:

```yaml
# docker-compose override
services:
  jev-mcp:
    environment:
      APP_ENV: dev
    volumes:
      - ./config/dev.toml:/app/config/dev.toml:ro
```

### 4. Docker image only

```bash
docker build -t jev-mcp-rust .
docker run --rm -p 8766:8766 -e APP_TYPESAFE_API_KEY="your_key" jev-mcp-rust
```

The image sets `APP_ENV=docker` and includes `config/default.toml` plus `config/docker.toml`.

### 5. Connect an HTTP MCP client

Point a client that supports **MCP streamable HTTP** at:

```text
http://<host>:8766/mcp/jev/v1
```

Exact client configuration depends on the host; use the same URL you would use for other streamable-HTTP MCP servers.

---

## Transport choice

| Mode      | When to use                                                      |
| --------- | ---------------------------------------------------------------- |
| **stdio** | IDE or CLI spawns the binary; no open port                       |
| **http**  | Docker, shared service, or clients that connect over the network |

Do not run stdio mode in a detached Docker container expecting network access — there is no stdin/stdout peer.

---

## Project layout

```text
src/
  main.rs           # Entry: load config, start stdio or HTTP transport
  server.rs         # MCP tool definitions
  tools/            # jev_noul, jev_score, jev_choice, jev_decide
  clients/          # TypeSafe HTTP client
  config/           # Settings loader
config/             # TOML config layers
Dockerfile
docker-compose.yml
```

Implementation notes and tradeoffs: `implementation-note.md`.

## License

Not specified in-repo; add a `LICENSE` file if you plan to distribute this project.
