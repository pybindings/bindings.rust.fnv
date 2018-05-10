#!/bin/sh
# The purpose of this file is to install rustup, libsodium and sccache in the
# Travis CI environment. Outside this environment, you would probably not want
# to install them like this.

set -e +x

# Setup rustup
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly

# Setup sccache
LATEST=$(cargo search -q sccache | grep sccache | cut -f2 -d"\"")
URL="https://github.com/mozilla/sccache/releases/download/${LATEST}/sccache-${LATEST}-x86_64-unknown-linux-musl.tar.gz"
/bin/echo -e "\e[32m\e[1m Downloading\e[0m sccache v$LATEST"
curl -SsL $URL | tar xzvC /tmp
mv "/tmp/sccache-${LATEST}-x86_64-unknown-linux-musl/sccache" "${CARGO_HOME}/bin/sccache"
mkdir -p "$SCCACHE_DIR"
