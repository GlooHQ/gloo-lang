#!/bin/bash

# Run tests for CI

set -euxo pipefail

env -u CONDA_PREFIX poetry run maturin develop --manifest-path ../../engine/language_client_python/Cargo.toml
poetry run baml-cli generate --from ../baml_src

# test_functions.py is excluded because it requires credentials
poetry run pytest "$@" --ignore=tests/test_functions.py
