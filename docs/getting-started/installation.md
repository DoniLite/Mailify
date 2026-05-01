---
title: Installation
description: Install Mailify via Docker, the universal install script, cargo install, or a prebuilt binary.
sidebar:
  order: 1
---

Mailify ships four install paths. Pick the one that matches your target.

## 1. Docker (recommended)

The Docker image bundles the binary + compiled templates + distroless runtime (~20 MB, non-root).

```bash
docker run -d \
  --name mailify \
  -p 8080:8080 \
  -e MAILIFY_DATABASE__URL=postgres://user:pass@host:5432/mailify \
  -e MAILIFY_SMTP__HOST=smtp.example.com \
  -e MAILIFY_SMTP__PORT=587 \
  -e MAILIFY_SMTP__USERNAME=postmaster@example.com \
  -e MAILIFY_SMTP__PASSWORD=secret \
  -e MAILIFY_SMTP__TLS=starttls \
  donighost/mailify:latest
```

See [Docker deployment guide](../guides/deploy-docker.md) for a full `docker-compose.yml` with Postgres attached.

### Available tags

| Tag | Meaning |
|-----|---------|
| `latest` | Latest release from the default branch |
| `0.1`, `0.1.2` | Semver-pinned (major.minor or major.minor.patch) |
| `sha-<short>` | Exact commit SHA |

Multi-arch: `linux/amd64` and `linux/arm64` published side-by-side — pull the same tag everywhere, Docker picks the right one.

## 2. Universal install script

One-liner that downloads the right prebuilt archive for your OS + arch from the latest GitHub Release, verifies the SHA256, installs the binary to `~/.local/bin` and the template bundle to `~/.local/share/mailify/templates`.

**Linux / macOS:**

```bash
curl -fsSL https://mailify.donilite.me/install.sh | sh
```

**Windows (PowerShell):**

```powershell
irm https://mailify.donilite.me/install.ps1 | iex
```

### Script options

| Flag / env | Effect |
|-----------|--------|
| `--version v0.2.0` / `MAILIFY_VERSION` | Pin a specific tag (default: latest) |
| `--dir ~/bin` / `MAILIFY_INSTALL_DIR` | Change install directory |
| `MAILIFY_NO_VERIFY=1` | Skip checksum (not recommended) |

After install, point Mailify at its templates:

```bash
export MAILIFY_TEMPLATES__PATH="$HOME/.local/share/mailify/templates"
mailify
```

## 3. `cargo install` (Rust toolchain)

:::caution[Planned, not yet released]
Publishing to crates.io is gated on the lib crates being deemed API-stable. Track progress in [`TODO.md §4.1`](https://github.com/donilite/mailify/blob/master/TODO.md).
:::

When ready:

```bash
cargo install mailify-api --locked
```

This installs the `mailify` binary into `$CARGO_HOME/bin`.

:::note
`cargo install` does **not** ship the compiled template bundle — you'll either build them from source (see [Template contract](../reference/template-contract.md)) or point `MAILIFY_TEMPLATES__PATH` at a pre-built bundle extracted from a GitHub Release archive.
:::

## 4. Build from source

```bash
git clone https://github.com/donilite/mailify.git
cd mailify

# 1. Build React Email templates (requires Bun ≥ 1.3)
cd templates-parser && bun install && bun run build && cd ..

# 2. Build the binary (requires Rust ≥ 1.88)
cargo build --release --bin mailify

# 3. Run it
./target/release/mailify
```

## Next steps

- [Send your first email →](./quickstart.md)
- [Understand the moving parts →](./concepts.md)
- [Configure SMTP for your provider →](../guides/configure-smtp.md)
- [Brand your mail →](../guides/configure-theme.md)
