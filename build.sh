#!/bin/sh

cargo build --release
mkdir -p "$XDG_SBIN_HOME"
mv -f ./target/release/clazzy "$XDG_SBIN_HOME"