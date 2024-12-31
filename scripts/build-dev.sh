#!/bin/bash

cargo build
wasm-pack build agreement-client --out-dir pkg-dev --target web
