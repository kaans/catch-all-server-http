---
 title: Introduction
---

cash — catch‑all HTTP server for inspecting requests
====================================================

cash is a lightweight, zero‑dependency tool to receive any HTTP request and immediately see everything about it. It’s ideal for debugging webhooks, testing clients, and exploring what your app is actually sending over the wire.

![Project screenshot placeholder](assets/screenshot.png)

What it does
------------
- Listens on a configurable host and port and accepts any HTTP method
- Prints detailed information about every incoming request:
  - Method, path, query, headers
  - Client address
  - Body (with configurable representation)
- Supports multiple body display formats:
  - text (UTF‑8, with safe fallback)
  - base64
  - hex
- Content negotiation for response formatting via Accept header:
  - application/json
  - text/html
- Safe by design for local debugging: HTTP only (no TLS) to avoid confusion in production‑like setups
- Docker support and a ready‑to‑use docker‑compose example, see [Docker & Docker Compose](docker.md)


Quick start
-----------
- Download a prebuilt binary (Windows, Linux, macOS incl. ARM64): https://github.com/kaans/catch-all-server-http/releases
- Run: `./catch-all-server-http` (or `./catch-all-server-http.exe` on Windows)
- Help: `./catch-all-server-http --help`
- Or build from source: `cargo build --release`

Configuration via CLI or environment
------------------------------------
You can configure cash using command‑line flags or environment variables. See the full reference with examples here:
[Configuration guide](config.md)

Key options include:
- --port / PORT (default: 8092)
- --host / HOST (default: 0.0.0.0)
- --use-color / USE_COLOR (default: true)
- --max-size / MAX_SIZE (default: 262144)
- --body-format / BODY_FORMAT [text|base64|hex] (default: text)

License
-------
This project is dual‑licensed under Apache 2.0 and MIT. You may use either license at your option:
- [Apache-2.0](https://github.com/kaans/catch-all-server-http/blob/main/LICENSE-APACHE)
- [MIT](https://github.com/kaans/catch-all-server-http/blob/main/LICENSE-MIT)
