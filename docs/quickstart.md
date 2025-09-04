---
title: Quickstart
---

Quickstart
=============

Get up and running in under a minute. Pick one of the options below.

Prerequisites
-------------
- Option A: Rust toolchain (cargo) installed
- Option B: Docker or Docker Desktop
- Option D: None (download prebuilt binary)

Option A — Run locally with Cargo
---------------------------------
1) Build and run with defaults (host 0.0.0.0, port 8092):
```bash
cargo run
```

2) Override host/port as needed:
```bash
cargo run -- --host 127.0.0.1 --port 8080
```

3) Or use environment variables (PowerShell example):
```powershell
$env:HOST = "127.0.0.1"; $env:PORT = "8080"; cargo run
```

Option B — Run with Docker
--------------------------
- Build image and run (maps host 8093 -> container 8092):
```bash
docker build -t cash .
docker run --rm -p 127.0.0.1:8093:8092 cash
```

Option C — One‑liner with Docker Compose
----------------------------------------
- Start in background and expose on 127.0.0.1:8093:
```bash
docker compose up -d
```

Option D — Download prebuilt binaries
-------------------------------------
- Windows, Linux, macOS (including ARM64) binaries are available on the Releases page:
  https://github.com/kaans/catch-all-server-http/releases
- Download the asset for your platform, extract if needed, then run the binary:
  - Windows (PowerShell): `./catch-all-server-http.exe --help`
  - Linux/macOS: `./catch-all-server-http --help`

Verify it works
---------------
- Send a request and see details in the terminal/logs:
```bash
curl http://127.0.0.1:8092    # using default values
```

Next steps
----------
- [Configuration reference](config.md)
- [Docker & Compose details](docker.md)
