#!/bin/bash

set -e

./scripts/build-dev.sh
cp -v agreement-client/pkg-dev/*.js agreement-client/pkg-dev/*.wasm agreement-web/static/js
