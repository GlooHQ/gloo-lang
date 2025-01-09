#!/bin/bash
set -e

# TypeScript Tests
cd typescript
pnpm install
pnpm run build:debug
pnpm run generate
pnpm run integ-tests
cd ..

# Python Tests
cd python
poetry install
poetry run maturin develop --manifest-path ../../engine/language_client_python/Cargo.toml
poetry run baml-cli generate --from ../baml_src
poetry run pytest
cd ..

# Ruby Tests
cd ruby
bundle install
rake generate
rake test
cd ..