# BAML Integration Tests

This directory contains integration tests for all BAML client libraries. These tests verify the functionality of BAML across different programming languages and ensure consistent behavior.

## Supported Languages

- [TypeScript](./typescript/README.md)
- [Python](./python/README.md)
- [Ruby](./ruby/README.md)

## Prerequisites

### Required for All Languages
- BAML CLI installed
- Infisical CLI installed (for managing environment variables)
- Rust toolchain (for building native clients)
- VSCode with [BAML Extension](https://marketplace.visualstudio.com/items?itemName=boundary.baml)

### Language-Specific Requirements
- **TypeScript**: Node.js and pnpm
- **Python**: Python 3.8+ and Poetry
- **Ruby**: mise (via Homebrew)

## Quick Start

1. Install the BAML VSCode extension and CLI:
```bash
# Install BAML CLI
curl -fsSL https://baml.dev/install.sh | sh

# Keep client libraries in sync
baml update-client
```

2. Set up environment variables using Infisical or .env files

3. Run all integration tests:
```bash
# From the integ-tests directory
./run-tests.sh
```

## BAML Source Files

The `baml_src/` directory contains all BAML definitions used by the tests. See [BAML Source README](./baml_src/README.md) for detailed information about:
- Test organization and structure
- How to add new tests
- Using generators
- Common test patterns
- Best practices

### Key Components

1. **Client Tests** (`clients.baml`)
   - Main client definitions
   - Core functionality tests

2. **Provider Tests** (`test-files/providers/`)
   - Provider-specific implementations
   - API integration tests

3. **Pipeline Tests** (`test-files/testing_pipeline/`)
   - Chain functionality
   - Fallback scenarios

4. **Generators** (`generators.baml`)
   - Reusable test patterns
   - Common test scenarios

## Running Tests by Language

### TypeScript
```bash
cd typescript
pnpm install
pnpm build:debug
pnpm generate
infisical run --env=test -- pnpm integ-tests
```

### Python
```bash
cd python
poetry install
env -u CONDA_PREFIX poetry run maturin develop --manifest-path ../../engine/language_client_python/Cargo.toml
poetry run baml-cli generate --from ../baml_src
infisical run --env=test -- poetry run pytest
```

### Ruby
```bash
cd ruby
mise exec -- bundle install
(cd ../../engine/language_client_ruby && mise exec -- rake compile)
mise exec -- baml-cli generate --from ../baml_src
infisical run --env=test -- mise exec -- rake test
```

## Project Structure

```
integ-tests/
├── baml_src/          # BAML source files and test definitions
│   ├── clients.baml   # Main client definitions
│   ├── generators.baml # Reusable test patterns
│   └── test-files/    # Organized test cases
├── typescript/        # TypeScript integration tests
├── python/           # Python integration tests
├── ruby/             # Ruby integration tests
├── openapi/          # OpenAPI integration tests
└── run-tests.sh      # Script to run all tests
```

## Development Workflow

1. **Write BAML Tests**
   - Add test cases in appropriate BAML files
   - Use generators for common patterns
   - Follow best practices in BAML Source README

2. **Generate Client Code**
   ```bash
   baml-cli generate --from ./baml_src
   ```

3. **Update Client Libraries**
   ```bash
   baml update-client
   ```

4. **Run Tests**
   - Use language-specific test commands (see above)
   - Or run all tests with `./run-tests.sh`

## Debugging

Each language has its own debugging setup in VSCode:

- **TypeScript**: Jest Runner extension
- **Python**: Python Test Explorer
- **Ruby**: Ruby Test Explorer

See individual language READMEs for detailed debugging instructions.

## Environment Variables

Tests can be run with environment variables in two ways:

1. **Using Infisical (Recommended)**
   ```bash
   infisical run --env=test -- [test command]
   ```

2. **Using .env Files**
   - Create a `.env` file in the root directory
   - Run tests without Infisical

## Common Issues

1. **Client Generation Failures**
   - Ensure BAML CLI is up to date
   - Verify BAML source files in `baml_src/`
   - Check language-specific client generation logs

2. **Build Issues**
   - Each language requires its native client to be built
   - See language-specific READMEs for build troubleshooting

3. **Environment Variables**
   - Ensure all required API keys are set
   - Verify Infisical configuration
   - Check .env file if using local environment

4. **Test Timeouts**
   - Each language has its own timeout configuration
   - See language-specific READMEs for timeout adjustments

## Getting Help

- Check the language-specific README for detailed troubleshooting
- Review test output and logs
- Enable debug logging with `BAML_LOG=trace`
- Report issues on [Github](https://github.com/boundaryml/baml)
- Join our [Discord](https://discord.gg/BTNBeXGuaS) for community support

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add or modify tests in `baml_src/`
4. Generate client code and run tests
5. Ensure all tests pass in all languages
6. Submit a pull request

## Deployment

The `baml_client` folder in each language contains all necessary files for deployment. You don't need the BAML compiler in production environments.
