# agreement

This project aims to generate an API client for a backend API using WASM.

Goals:
- auto-generated WASM client;
- common validation, wherever possible.

## Validation

It's not always possible to validate the payload, generally for data which
is unavailable on the client. For example, validation which require:
- database calls;
- external api calls;
- secrets.

## Insights

The lib works for any share of an API so the API doesn't have to conform
to REST or SOAP (if anyone still uses that).
I'm gonna first limit it to HTTP for this PoC.

## Installation

Prerequisites
- rustup: https://rustup.rs/
- wasm-pack: https://rustwasm.github.io/wasm-pack/installer/
