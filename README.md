# cash

> **catch-all server for analyzing HTTP requests**

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).

---

![Screenshot of cash](/docs/_include/assets/screenshot.png)

---

## Build and run

Prefer prebuilt binaries? Download for Windows, Linux, and macOS (including ARM64) from Releases:
https://github.com/kaans/catch-all-server-http/releases

- Windows (PowerShell): `./catch-all-server-http.exe --help`
- Linux/macOS: `./catch-all-server-http --help`

Build the executable `catch-all-server-http(.exe)` yourself (find it in `target/release`):

```bash
cargo build --release
```

Or run the program directly:

```bash
cargo run --release
```

For help, list all command line parameters:

```bash
cargo run --release -- -h
```

## Docker

The [Dockerfile](Dockerfile) produces and image that can be used
to start a container in which the *cash* server runs.

Build the image and tag the image as "cash":

```bash
docker build -t cash .
```

Start a container and expose *cash* on port 8093:

```bash
docker run -p 127.0.0.1:8093:8092 cash
```

## Docker Compose

Run the container (in the background `-d`) using [docker compose](docker-compose.yaml):

```bash
docker compose up -d
```

## Usage

Send HTTP requests to the cash server's endpoint and cash will list all
details about the incoming request.

By default, cash is listening on your localhost on port 8092:

```
http://localhost:8092
```

The port and other settings can be configured using cli parameters or environment variables
(see [Build and run](#build-and-run) how to get them).

### Transport encryption using TLS or SSL

TLS or SSL is not supported because the server is intended for debugging purposes only. Use plain `http://` connection only.

### Encoding of the request body

By default, the body of the request is being parsed as UTF-8 and displayed as string. In case the
body contains bytes which are not parseable as UTF-8, the payload
will be displayed as base64 encoded.

The body can be converted to other formats using the `BODY_FORMAT` environment variable or the `--body-format` cli argument.
Currently, the following formats are supported:

- text
- base64
- hex

### Output format

The response body can be formatted in different formats using the `Accept` header.
Currently, these formats are supported:

- application/json
- text/html

Set the value of the `Accept` header to one of these to format
the response body in the corresponding format.
