SHELL=/bin/sh

profile := $(if $(findstring r, $(MAKEFLAGS)),release,dev)

app_dir = agreement-web
client_dir = agreement-client

.PHONY: wasm-deploy
wasm-deploy: wasm-build
	cp -v ${client_dir}/pkg/*.js ${client_dir}/pkg/*.wasm ${app_dir}/static/js

.PHONY: wasm-build
wasm-build:
	wasm-pack build ${client_dir} --${profile} --out-dir pkg --target web

.PHONY: wasm-clean
wasm-clean:
	rm -rf ${client_dir}/pkg/

.PHONY: clean
clean: wasm-clean
	cargo clean
	rm -rf ${client_dir}/pkg/
