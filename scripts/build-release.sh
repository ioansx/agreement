#!/bin/bash

set -e

cargo build --release
wasm-pack build agreement-client --release --out-dir pkg-release --target web
