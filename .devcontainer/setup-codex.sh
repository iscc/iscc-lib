#!/usr/bin/env bash
# Configure Codex CLI for devcontainer use.
#
# Codex stores its state in SQLite databases under CODEX_HOME (~/.codex), opened
# in WAL mode. WAL needs a shared-memory (-shm) file accessed via mmap + POSIX
# locking, which a Windows->container bind mount (Docker Desktop gRPC-FUSE/
# virtiofs) does not support — codex then fails with "disk I/O error"
# (SQLITE_IOERR_SHMOPEN, code 4618). Sharing the bind mount also lets the host
# and container codex builds collide on migration checksums ("migration 1 was
# previously applied but has been modified"). So ~/.codex lives on a native
# Docker volume (real ext4 -> WAL works, isolated from the host -> no migration
# clash) and the host's ~/.codex is bind-mounted read-only at ~/.codex-host.
# This script seeds the volume from the host config on first create so the host
# ChatGPT login carries over, without putting SQLite on the bind mount.
set -e

codex_home="$HOME/.codex"
host="$HOME/.codex-host"
config="$codex_home/config.toml"

mkdir -p "$codex_home"

# Seed auth from the host login, but only when the volume has no credentials yet
# so a container-refreshed token is never clobbered by a staler host copy.
if [ ! -s "$codex_home/auth.json" ] && [ -s "$host/auth.json" ]; then
    cp "$host/auth.json" "$codex_home/auth.json"
    chmod 600 "$codex_home/auth.json"
    echo "codex: seeded auth.json from host login"
fi

# Seed config.toml from the host on first create to preserve model/MCP settings.
if [ ! -f "$config" ] && [ -f "$host/config.toml" ]; then
    cp "$host/config.toml" "$config"
    echo "codex: seeded config.toml from host"
fi

# Force file-based credential storage so tokens persist in the volume rather
# than a keyring that doesn't survive container rebuilds.
if [ ! -f "$config" ]; then
    printf '# Force file-based credential storage for devcontainer compatibility.\ncli_auth_credentials_store = "file"\n' > "$config"
    echo "codex: created $config with file-based credential storage"
elif ! grep -q 'cli_auth_credentials_store' "$config"; then
    # Prepend at the top so the key lands in the global TOML scope. Appending to
    # EOF would place it inside the trailing [section] table (if any), where
    # codex would silently ignore or misparse it.
    { printf '# Force file-based credential storage for devcontainer compatibility.\ncli_auth_credentials_store = "file"\n\n'; cat "$config"; } > "$config.tmp" && mv "$config.tmp" "$config"
    echo "codex: added file-based credential storage to $config"
elif grep -q 'cli_auth_credentials_store.*=.*"keyring"' "$config"; then
    echo "codex: WARNING — cli_auth_credentials_store is set to \"keyring\"; run 'codex login' in the container if auth fails"
fi

if [ -s "$codex_home/auth.json" ]; then
    echo "codex: credentials available"
else
    echo "codex: no credentials — run 'codex login --device-auth' to authenticate"
fi
