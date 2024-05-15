
SHELL=/bin/bash
all:
	@make help


.PHONY: help ## Display help commands
help:
	@printf 'Usage:\n'
	@printf '  make <tagert>\n'
	@printf '\n'
	@printf 'Targets:\n'
	@IFS=$$'\n' ; \
    help_lines=(`fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//'`); \
    for help_line in $${help_lines[@]}; do \
        IFS=$$'#' ; \
        help_split=($$help_line) ; \
        help_info=`echo $${help_split[2]} | sed -e 's/^ *//' -e 's/ *$$//'` ; \
		IFS=$$':' ; \
		phony_command=($$help_split); \
        help_command=`echo $${phony_command[1]} | sed -e 's/^ *//' -e 's/ *$$//'` ; \
		printf "  %-50s %s\n" $$help_command $$help_info ; \
    done


# clippy

.PHONY: clippy ## cargo clippy
clippy:
	SKIP_WASM_BUILD=1 cargo clippy --workspace --all-targets --all-features -- -D warnings

.PHONY: clippyfix ## cargo clippy --fix
clippyfix:
	SKIP_WASM_BUILD=1 cargo clippy --allow-dirty --allow-staged --fix --workspace --all-targets --all-features -- -D warnings

.PHONY: cargofix ## cargo fix
cargofix:
	cargo fix --allow-dirty --allow-staged --workspace --all-targets --all-features


# toml && lock check
.PHONY: tomlcheck ## cargo toml check
tomlcheck:
	cargo audit fix --dry-run

# format

.PHONY: fmtcheck ## cargo fmt check
fmtcheck:
	cargo fmt --all -- --check

.PHONY: taplocheck ## taplo fmt check
#  cargo install taplo-cli --locked
taplocheck:
	taplo fmt --check

.PHONY: fmt ## cargo fmt all && taplo fmt all
fmt:
	cd ./script && cargo fmt --all && taplo fmt
	cd ./program && cargo fmt --all && taplo fmt

.POHY : execute  ## execute the proof 
execute:
	cd ./script && RUST_LOG=info cargo run --package fibo-script --bin execute --release

.POHY : compressed  ## compressed ßthe proof 
compressed:
	cd ./script && RUST_LOG=info cargo run --package fibo-script --bin compressed --release

.POHY : groth16  ## generate groth16 proof && verify
groth16:
	cd ./script && RUST_LOG=info cargo run --package fibo-script --bin groth16 --release

.POHY : plonk  ## generate plonk proof && verify
plonk:
	cd ./script && RUST_LOG=info cargo run --package fibo-script --bin plonk --release
