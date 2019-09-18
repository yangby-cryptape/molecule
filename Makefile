ci: ci-example ci-rust ci-c

ci-rust:
	set -eu; \
	export RUSTFLAGS='-F warnings'; \
	for dir in \
			examples/ci-tests\
			bindings/rust \
			tools/codegen \
			tools/compiler \
			; do \
		cd "$${dir}"; \
		cargo clean; \
		cargo fmt --all -- --check; \
		cargo clippy --all --all-targets --all-features; \
		cargo test --all --verbose; \
		cd ../..; \
	done

ci-c:
	set -eu; \
	echo "TODO: not finished yet."

ci-example:
	set -eu; \
	cd examples/ci-tests; \
	make clean test
