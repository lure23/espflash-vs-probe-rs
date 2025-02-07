#!/bin/bash
set -e

_CARGO_CONFIG=.cargo/config.toml
_CARGO_TOML=Cargo.toml

cat ${_CARGO_CONFIG} | sed -E 's/riscv32im[a]?c/riscv32imac/g' > .tmp
mv .tmp ${_CARGO_CONFIG}

cat ${_CARGO_TOML} | sed -E 's/"esp32c[[:digit:]]"/"esp32c6"/g' > .tmp
mv .tmp ${_CARGO_TOML}
