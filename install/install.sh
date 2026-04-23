#!/usr/bin/env sh
# Mailify installer — POSIX sh, no bashisms.
#
# Usage:
#   curl -fsSL https://mailify.donilite.me/install.sh | sh
#   curl -fsSL https://mailify.donilite.me/install.sh | sh -s -- --version v0.2.0 --dir ~/bin
#
# Env overrides:
#   MAILIFY_VERSION   pin a specific tag (default: latest)
#   MAILIFY_INSTALL_DIR   install path (default: $HOME/.local/bin)
#   MAILIFY_REPO      GitHub repo (default: donilite/mailify)
#   MAILIFY_NO_VERIFY=1   skip checksum verification (not recommended)

set -eu

REPO="${MAILIFY_REPO:-donilite/mailify}"
VERSION="${MAILIFY_VERSION:-latest}"
INSTALL_DIR="${MAILIFY_INSTALL_DIR:-$HOME/.local/bin}"

log() { printf '→ %s\n' "$*"; }
err() { printf '✗ %s\n' "$*" >&2; exit 1; }

# --- parse flags ---
while [ $# -gt 0 ]; do
  case "$1" in
    --version) VERSION="$2"; shift 2 ;;
    --dir) INSTALL_DIR="$2"; shift 2 ;;
    -h|--help)
      cat <<EOF
Mailify installer

Options:
  --version <TAG>     Pin version (e.g. v0.2.0). Default: latest
  --dir <PATH>        Install directory. Default: \$HOME/.local/bin
  -h, --help          Show this help
EOF
      exit 0 ;;
    *) err "unknown flag: $1" ;;
  esac
done

# --- detect OS ---
OS="$(uname -s)"
case "$OS" in
  Linux)  OS=linux ;;
  Darwin) OS=darwin ;;
  *) err "unsupported OS: $OS" ;;
esac

# --- detect arch ---
ARCH="$(uname -m)"
case "$ARCH" in
  x86_64|amd64) ARCH=x86_64 ;;
  aarch64|arm64) ARCH=aarch64 ;;
  *) err "unsupported arch: $ARCH" ;;
esac

# --- resolve target triple ---
case "$OS-$ARCH" in
  linux-x86_64)  TARGET=x86_64-unknown-linux-gnu ;;
  linux-aarch64) TARGET=aarch64-unknown-linux-gnu ;;
  darwin-x86_64) TARGET=x86_64-apple-darwin ;;
  darwin-aarch64) TARGET=aarch64-apple-darwin ;;
  *) err "no prebuilt target for $OS-$ARCH" ;;
esac

# --- resolve version ---
if [ "$VERSION" = "latest" ]; then
  log "Resolving latest release tag from github.com/$REPO"
  VERSION="$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" \
    | sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p')"
  [ -n "$VERSION" ] || err "could not resolve latest version (API rate limit? set MAILIFY_VERSION=vX.Y.Z)"
fi

VERSION_NOV="${VERSION#v}"
ARCHIVE="mailify-${VERSION_NOV}-${TARGET}.tar.gz"
URL="https://github.com/$REPO/releases/download/$VERSION/$ARCHIVE"
SHA_URL="$URL.sha256"

log "Downloading $ARCHIVE"
TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

curl -fsSL "$URL" -o "$TMPDIR/$ARCHIVE" || err "download failed: $URL"

# --- checksum ---
if [ "${MAILIFY_NO_VERIFY:-0}" != "1" ]; then
  log "Verifying SHA256"
  curl -fsSL "$SHA_URL" -o "$TMPDIR/$ARCHIVE.sha256" || err "could not fetch checksum"
  (
    cd "$TMPDIR"
    if command -v sha256sum >/dev/null 2>&1; then
      sha256sum -c "$ARCHIVE.sha256" >/dev/null || err "checksum mismatch"
    else
      EXPECTED="$(awk '{print $1}' "$ARCHIVE.sha256")"
      ACTUAL="$(shasum -a 256 "$ARCHIVE" | awk '{print $1}')"
      [ "$EXPECTED" = "$ACTUAL" ] || err "checksum mismatch (expected $EXPECTED, got $ACTUAL)"
    fi
  )
fi

log "Extracting"
tar -xzf "$TMPDIR/$ARCHIVE" -C "$TMPDIR"
STAGE="$TMPDIR/mailify-${VERSION_NOV}-${TARGET}"

mkdir -p "$INSTALL_DIR"
install -m 0755 "$STAGE/mailify" "$INSTALL_DIR/mailify"

# --- templates go to XDG data dir ---
DATA_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/mailify"
mkdir -p "$DATA_DIR"
rm -rf "$DATA_DIR/templates"
cp -R "$STAGE/templates" "$DATA_DIR/templates"

log "Installed mailify $VERSION → $INSTALL_DIR/mailify"
log "Templates → $DATA_DIR/templates"
log "Set MAILIFY_TEMPLATES__PATH=$DATA_DIR/templates or put path in Mailify.toml"

case ":$PATH:" in
  *":$INSTALL_DIR:"*) ;;
  *) log "Warning: $INSTALL_DIR is not in \$PATH. Add this to your shell rc:"
     log "  export PATH=\"$INSTALL_DIR:\$PATH\"" ;;
esac
