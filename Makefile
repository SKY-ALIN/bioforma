SHELL := /bin/bash

build:
	maturin build --release --out dist --interpreter 3.7 3.8 3.9 3.10 3.11
install:
	pip install bioforma --no-index --no-deps --find-links dist --force-reinstall
test:
	@if [[ "$$OSTYPE" == "linux-gnu" ]]; then \
	    mv bioforma/ bioforma_moved; \
	fi
	python -m pytest -s -vv || true
	@if [[ "$$OSTYPE" == "linux-gnu" ]]; then \
	    mv bioforma_moved/ bioforma; \
	fi
lint:
	cargo clippy
	cargo fmt --all --check
check:
	$(MAKE) build
	$(MAKE) install
	$(MAKE) lint
	$(MAKE) test
fix:
	cargo fmt --all
