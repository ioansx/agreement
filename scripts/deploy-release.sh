#!/bin/bash

set -e

./scripts/build-release.sh
cp -v agreement-client/pkg-release/*.js agreement-client/pkg-release/*.wasm agreement-web/static/js
