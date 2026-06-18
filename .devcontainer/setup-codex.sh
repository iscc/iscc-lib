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
#
# Deliberately NOT `set -e`: this is a convenience seeder, not a hard dependency
# of the dev environment. It is invoked via `bash` in postCreateCommand (so it
# does not depend on a working-tree exec bit, which a Windows bind mount cannot
# preserve), and it always exits 0 so a transient hiccup can never abort the
# rest of postCreate (mise trust, uv sync, ...).

codex_home="$HOME/.codex"
host="$HOME/.codex-host"
config="$codex_home/config.toml"

mkdir -p "$codex_home"

# Copy a host file into the volume, retrying to ride out transient I/O errors on
# the Windows->container bind mount (gRPC-FUSE/virtiofs). Returns non-zero only
# if every attempt fails. Never aborts the script.
seed_from_host() {
    src="$1"
    dst="$2"
    for attempt in 1 2 3; do
        if cp "$src" "$dst" 2>/dev/null; then
            return 0
        fi
        sleep 1
    done
    echo "codex: WARNING — could not read $src after retries (bind-mount hiccup?)"
    return 1
}

# Seed auth from the host login, but only when the volume has no credentials yet
# so a container-refreshed token is never clobbered by a staler host copy.
if [ ! -s "$codex_home/auth.json" ] && [ -s "$host/auth.json" ]; then
    if seed_from_host "$host/auth.json" "$codex_home/auth.json"; then
        chmod 600 "$codex_home/auth.json" 2>/dev/null || true
        echo "codex: seeded auth.json from host login"
    fi
fi

# Seed config.toml from the host on first create to preserve model/MCP settings.
if [ ! -f "$config" ] && [ -f "$host/config.toml" ]; then
    seed_from_host "$host/config.toml" "$config" && echo "codex: seeded config.toml from host"
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

# Always succeed: a seeding hiccup must never abort the rest of postCreate.
exit 0
