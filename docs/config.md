---
title: Configuration
---

Configuration
=============

This page documents all configuration parameters supported by catch-all-server-http. You can configure the server via command-line flags or environment variables. Command-line flags always take precedence over environment variables. Defaults are shown below.

Parameters
----------

### Port

The TCP port the HTTP server listens on.
- CLI: -p, --port <u16>
- Environment: PORT
- Default: 8092

### Host

The network interface or IP address to bind to.
Use 0.0.0.0 to listen on all IPv4 interfaces (the default), or 127.0.0.1 to accept connections only from the local machine.
- CLI: -o, --host <string>
- Environment: HOST
- Default: 0.0.0.0

### Use color

Controls whether colored output is used in logs/terminal. Set this to false to disable ANSI color codes (useful for plain logs or CI environments).
- CLI: -u, --use-color <bool>
- Environment: USE_COLOR
- Default: true
- Notes: Booleans are parsed from common forms like true/false, 1/0, yes/no.

### Max size

The maximum number of bytes of the request body that will be read and printed. Larger requests may be truncated or rejected depending on internal handling.
- CLI: -m, --max-size <usize>
- Environment: MAX_SIZE
- Default: 262144 (256 KiB)

### Body format

Controls how the request body is displayed in the output. Choose a representation that best fits your payloads or tooling.
- CLI: -b, --body-format <text|base64|hex>
- Environment: BODY_FORMAT
- Default: text
- Modes:
  - text: Treat body as UTF-8 text and print directly. If the payload is not valid UTF-8, it may fall back to an encoded representation.
  - base64: Print the raw bytes base64-encoded.
  - hex: Print the raw bytes as a lowercase hexadecimal string.
- Notes: Values are case-insensitive in practice; use the forms shown above for clarity.

Examples
--------

Listen locally on port 3000 without colors
- cargo run -- --host 127.0.0.1 --port 3000 --use-color false

Accept larger payloads (up to 10 MiB) and show body in hex
- cargo run -- --max-size 10485760 --body-format hex

Configure via environment only
- $env:HOST = "127.0.0.1"
- $env:PORT = "9090"
- $env:USE_COLOR = "false"
- $env:MAX_SIZE = "524288"
- $env:BODY_FORMAT = "base64"
- cargo run
