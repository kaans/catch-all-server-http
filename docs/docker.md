---
 title: Docker & Docker Compose
---

Docker usage guide
===================

This guide explains how to run cash using Docker or Docker Compose.

Running with plain Docker
-------------------------
1) Build the image and tag it as cash:
```bash
docker build -t cash .
```

2) Run a container and map host port 8093 to the container’s 8092:
```bash
docker run --rm -p 127.0.0.1:8093:8092 cash
```

3) Send a request to the server (from another terminal):
```bash
curl http://127.0.0.1:8093
```

4) Stop the container with Ctrl+C (or it will exit when the process ends if not detached).

Overriding configuration with Docker
------------------------------------
- Change the port published on the host (left side of the mapping):
```bash
docker run --rm -p 127.0.0.1:9000:8092 cash
```

- Change server settings via environment variables:
```bash
docker run --rm -p 127.0.0.1:8093:8092 -e BODY_FORMAT=hex -e MAX_SIZE=1048576 cash
```

- Bind to all host interfaces (be careful; exposes to your network):
```bash
docker run --rm -p 0.0.0.0:8093:8092 cash
```

Running with Docker Compose
---------------------------
There are two ways to run with Docker Compose:

A) Use the published image (pull from GitHub Container Registry)
- This uses the default docker-compose.yaml, which references the prebuilt image:
  - Image: ghcr.io/kaans/catch-all-server-http:latest
  - Published port: 127.0.0.1:8092 -> container 8092
- Commands:
```bash
# start detached using the published image
docker compose up -d

# follow logs
docker compose logs -f

# send a request
curl http://127.0.0.1:8092

# stop and remove
docker compose down
```

B) Use the development override (build from local source)
- This uses docker-compose.dev.yaml to override the image with a local build and a different host port (8093).
- Commands:
```bash
# start detached, building from local Dockerfile and using dev overrides
docker compose -f docker-compose.yaml -f docker-compose.dev.yaml up -d --build

# follow logs
docker compose -f docker-compose.yaml -f docker-compose.dev.yaml logs -f

# send a request (dev override publishes 127.0.0.1:8093)
curl http://127.0.0.1:8093

# stop and remove
docker compose -f docker-compose.yaml -f docker-compose.dev.yaml down
```

Customizing docker-compose.yaml
-------------------------------
- Change the host port published (e.g., 8080 -> 8092 inside):
```yaml
services:
  cash:
    ports:
      - "127.0.0.1:8080:8092"
```

- Override environment variables:
```yaml
services:
  cash:
    environment:
      BODY_FORMAT: base64
      MAX_SIZE: 1048576
```

- Build without cache when iterating locally:
```yaml
services:
  cash:
    build:
      dockerfile: Dockerfile
      no_cache: true
```

Notes and troubleshooting
------------------------
- If you cannot reach the service:
  - Ensure you are calling the correct published host port:
    - Using published image (docker-compose.yaml): 8092
    - Using dev override (docker-compose.dev.yaml): 8093
  - On Linux, 127.0.0.1 binding limits access to the local machine. Use 0.0.0.0 if you need LAN access (only in trusted environments).
- On Windows with WSL2 or Docker Desktop, localhost port publishing generally works as shown.
- TLS/SSL is intentionally not supported; use http:// only.
