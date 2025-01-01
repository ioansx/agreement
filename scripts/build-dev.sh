#!/bin/bash

set -e

cargo build
wasm-pack build agreement-client --out-dir pkg-dev --target web
