ci:
	@set -eu; \
	export RUSTFLAGS='-D warnings'; \
	make fmt clippy; \
	make ci-examples ci-crates; \
	echo "Success!"

RUST_PROJS = examples/ci-tests bindings/rust tools/codegen tools/compiler
C_PROJS = examples/ci-tests

clean:
	@set -eu; \
	for dir in ${RUST_PROJS}; do \
		cd "$${dir}"; \
		cargo clean; \
		cd - > /dev/null; \
	done; \
	for dir in ${C_PROJS}; do \
		cd "$${dir}"; \
		make clean; \
		cd - > /dev/null; \
	done

fmt:
	@set -eu; \
	for dir in ${RUST_PROJS}; do \
		cd "$${dir}"; \
		cargo fmt --all -- --check; \
		cd - > /dev/null; \
	done

clippy:
	@set -eu; \
	for dir in ${RUST_PROJS}; do \
		cd "$${dir}"; \
		cargo clean; \
		cargo clippy --all --all-targets --all-features; \
		cd - > /dev/null; \
	done

ci-crates:
	@set -eu; \
	for dir in ${RUST_PROJS}; do \
		cd "$${dir}"; \
		cargo clean; \
		cargo test --all --verbose; \
		cd - > /dev/null; \
	done; \
	git diff --exit-code tools/compiler/Cargo.lock

ci-examples:
	@set -eu; \
	cd examples/ci-tests; \
	make clean test
