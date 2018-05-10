#!/bin/bash
set -e +x

clean_project() {
    # Remove compiled files that might cause conflicts
    rm -rf /io/.cache /io/.eggs /io/build /io/*.egg-info
    find /io/ -name "__pycache__" -type d -print0 | xargs -0 rm -rf
    find /io/ -name "*.pyc" -type f -print0 | xargs -0 rm -rf
    find /io/ -name "*.so" -type f -print0 | xargs -0 rm -rf
}

# Install rustup
curl -SsL https://sh.rustup.rs | sh -s -- -y --no-modify-path --default-toolchain nightly

# Add cargo to path
export CARGO_HOME="$HOME/.cargo"
export PATH="$CARGO_HOME/bin:$PATH"
export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$CARGO_HOME/lib"
export SODIUM_STATIC="true"
export SODIUM_LIB_DIR="$CARGO_HOME/lib"
export SCCACHE_DIR="/var/cache/sccache"
export RUSTC_WRAPPER="sccache"

# Install sccache
LATEST=$(cargo search -q sccache | grep sccache | cut -f2 -d"\"")
URL="https://github.com/mozilla/sccache/releases/download/${LATEST}/sccache-${LATEST}-x86_64-unknown-linux-musl.tar.gz"
echo -e "\e[32m\e[1m Downloading\e[0m sccache v$LATEST"
curl -SsL $URL | tar xzvC /tmp
mv "/tmp/sccache-${LATEST}-x86_64-unknown-linux-musl/sccache" "${CARGO_HOME}/bin/sccache"
mkdir -p "$SCCACHE_DIR"

# Compile wheels
echo -e "\e[32m\e[1m    Building\e[0m wheel for $(python --version)"
pip wheel . -w ./dist
