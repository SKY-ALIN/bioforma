testing_build:
	maturin develop
test:
	pytest -s -vv
lint:
	cargo clippy
	cargo fmt --all --check
check: testing_build test lint
