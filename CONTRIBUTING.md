# Contributing to BAML

Welcome to BAML! This guide will help you get started with contributing to the project. Whether you're an open source contributor or a new BAML employee, we're excited to have you on board.

## Quick Links
- [Discord Community](https://discord.gg/BTNBeXGuaS) - Join our `#contributing` channel
- [Documentation](https://docs.boundaryml.com)
- [Example Projects](https://github.com/BoundaryML/baml-examples)

## Getting Started

1. **Join the Community**
   - Introduce yourself in our [Discord](https://discord.gg/BTNBeXGuaS) `#contributing` channel
   - For BAML employees: Check the internal onboarding doc (shared during onboarding)

2. **Find Something to Work On**
   - Browse [good first issues](https://github.com/BoundaryML/baml/issues?q=is:issue+is:open+label:"good+first+issue")
   - Check [discussion board](https://github.com/BoundaryML/baml/discussions)

3. **Development Setup**
   ```bash
   # Clone repository
   git clone https://github.com/boundaryml/baml.git
   cd baml

   # Install core dependencies
   brew install mise direnv  # macOS
   # or
   apt install direnv       # Ubuntu

   # Configure shell (add to ~/.zshrc or ~/.bashrc)
   eval "$(direnv hook zsh)"
   eval "$(mise activate zsh)"

   # Install language-specific tools
   mise install            # Sets up Python, Ruby, Node.js versions

   #Install BAML CLI with Python
   pip install baml-py

   #Install BAML CLI with Ruby
   gem install baml

   #Install BAML CLI with TypeScript
   npm install @boundaryml/baml
   ```

## Development Setup

Before you begin contributing, familiarize yourself with these key documents:
- [Architecture Overview](docs/architecture.md)
- [Code Generation Guide](docs/code-generation.md) - Essential for working with client libraries
- [Coding Standards](docs/coding-standards.md)

## Project Architecture

BAML consists of several key components:

```
baml/
├── engine/          # Core Rust implementation
│   ├── baml-lib/    # Compiler and type system
│   ├── baml-runtime/# Runtime engine
│   └── cli/         # CLI tool
├── integ-tests/     # Integration tests
└── docs/           # Documentation
```

See [Architecture Overview](docs/architecture.md) for details.

## Development Workflow

### 1. Set Up Your Environment
- Follow language-specific setup:
  - [Rust Development](engine/README.md#development)
  - [Python Client](engine/language_client_python/README.md#development)
  - [TypeScript Client](engine/language_client_typescript/README.md#development)
  - [Ruby Client](engine/language_client_ruby/README.md#development)

### 2. Understanding the Codebase
- Start with the [Engine Overview](engine/README.md)
- Review component documentation:
  - [Core Library](engine/baml-lib/README.md)
  - [Runtime](engine/baml-runtime/README.md)
  - [CLI](engine/cli/README.md)

### 3. Making Changes

#### Core Engine Changes
1. Read [Engine Development Guide](engine/README.md#development)
2. Make changes in relevant component
3. Run component tests:
   ```bash
   cd ./engine
   cargo test -p baml-lib
   cargo test -p baml-runtime
   cargo test -p baml-cli
   ```
4. Run integration tests:
   ```bash
   cd ./integ-tests
   ./run-tests.sh
   ```

#### Language Client Changes
1. Read relevant client guide:
   - [Python Client Guide](engine/language_client_python/README.md)
   - [TypeScript Client Guide](engine/language_client_typescript/README.md)
   - [Ruby Client Guide](engine/language_client_ruby/README.md)
2. Make changes
3. Run language-specific tests (see client guides)

### 4. Testing
See [Testing Guide](integ-tests/README.md) for:
- Running test suites
- Adding new tests
- Debugging tests
- Common issues

### 5. Documentation Changes

#### Updating Fern Documentation

The BAML documentation is built using Fern. Here's how to update it:

1. **Setup Fern Environment**
   ```bash
   # Install Fern CLI globally
   npm install -g fern-api

   # Navigate to Fern directory
   cd fern
   ```

2. **Local Development**
   ```bash
   # Start local development server
   fern docs dev

   # View at http://localhost:3000
   ```

3. **Documentation Structure**
   ```
   fern/
   ├── pages/          # Main documentation content
   ├── 01-guide/       # Getting started guides
   ├── 02-examples/    # Code examples
   ├── 03-reference/   # API reference
   ├── assets/         # Images and other assets
   └── snippets/       # Code snippets
   ```

4. **Making Changes**
   - Edit MDX files in respective directories
   - Add new pages in appropriate sections
   - Update navigation in `docs.yml`
   - Test changes locally

4. Run the integration tests.

## Running Integration Tests

The integration tests verify BAML's functionality across multiple programming languages. Each language has its own test suite in the `integ-tests/` directory.

### Prerequisites for All Tests

- [Rust toolchain](https://rustup.rs/) (for building native clients)
- [BAML CLI](https://github.com/boundaryml/baml#installation)

#### Environment Variables
You can set up environment variables in two ways:

1. **Using .env file (Recommended for external contributors)**:
   - Create a `.env` file in the `integ-tests` directory
   - Required variables:
     ```bash
     OPENAI_API_KEY=your_key_here
     # Add other provider keys as needed:
     # ANTHROPIC_API_KEY=your_key_here
     # AWS_ACCESS_KEY_ID=your_key_here
     # etc.
     ```

2. **Using Infisical (BAML internal use only)**:
   - Install [Infisical CLI](https://infisical.com/docs/cli/overview)
   - Use the `infisical run` commands shown in examples below
   - External contributors should replace `infisical run --env=test --` with `dotenv -e ../.env --` in all commands

### TypeScript Integration Tests

1. Install prerequisites:
   - [Node.js](https://nodejs.org/) (Latest LTS recommended)
   - [pnpm](https://pnpm.io/installation) package manager

2. Build the TypeScript runtime:
```bash
cd engine/language_client_typescript
pnpm build:debug
```

3. Set up and run tests:
```bash
cd integ-tests/typescript
pnpm install
pnpm generate
dotenv -e ../.env -- pnpm integ-tests  # or use infisical for internal BAML devs
```

### Python Integration Tests

1. Install prerequisites:
   - [Python](https://www.python.org/downloads/) 3.8 or higher
   - [Poetry](https://python-poetry.org/docs/#installation) package manager

2. Set up the environment:
```bash
cd integ-tests/python
poetry install
```

3. Build and install the Python client:
```bash
# Note: env -u CONDA_PREFIX is needed if using Conda
env -u CONDA_PREFIX poetry run maturin develop --manifest-path ../../engine/language_client_python/Cargo.toml
```

4. Generate client code and run tests:
```bash
poetry run baml-cli generate --from ../baml_src
dotenv -e ../.env -- poetry run pytest  # or use infisical for internal BAML devs
```

### Ruby Integration Tests

1. Install prerequisites:
   - [mise](https://mise.jdx.dev/getting-started.html) for Ruby version management:
     ```bash
     brew install mise  # on macOS
     # or
     curl https://mise.run | sh  # other platforms
     ```
   - Rust toolchain (installed above)

2. Set up mise and build the Ruby client:
```bash
cd integ-tests/ruby
mise install  # This will install Ruby version from .mise.toml
(cd ../../engine/language_client_ruby && mise exec -- rake compile)
```

3. Install dependencies and generate client:
```bash
mise exec -- bundle install
mise exec -- baml-cli generate --from ../baml_src
```

4. Run tests:
```bash
dotenv -e ../.env -- mise exec -- rake test  # or use infisical for internal BAML devs
```

### Adding New Tests

1. Define your BAML files in `integ-tests/baml_src/`:
   - Add clients in `clients.baml`
   - Add functions and tests in `test-files/providers/`
   - See [BAML Source README](integ-tests/baml_src/README.md) for details

2. Generate client code for each language:
```bash
# TypeScript
cd integ-tests/typescript && pnpm generate

# Python
cd integ-tests/python && poetry run baml-cli generate --from ../baml_src

# Ruby
cd integ-tests/ruby && mise exec -- baml-cli generate --from ../baml_src
```

3. Create language-specific test files:
   - Follow the patterns in existing test files
   - Use language-appropriate testing frameworks (Jest, pytest, Minitest)
   - Include both success and error cases
   - Test edge cases and timeouts

4. Run the tests in each language to ensure cross-language compatibility

### Debugging Tests

Each language has its own debugging setup in VS Code:

1. **TypeScript**:
   - Install Jest Runner extension
   - Use launch configuration from TypeScript README
   - Set `BAML_LOG=trace` for detailed logs

2. **Python**:
   - Install Python Test Explorer
   - Use launch configuration from Python README
   - Use `-s` flag to show print statements

3. **Ruby**:
   - Install Ruby Test Explorer
   - Use launch configuration from Ruby README
   - Use verbose mode for detailed output

### Common Issues and Solutions

1. **Environment Setup**:
   - For external contributors:
     - Create a `.env` file with required API keys
     - Use `dotenv` commands instead of Infisical
   - For BAML internal developers:
     - Ensure Infisical is configured correctly
     - Use `infisical run` commands

2. **Client Generation**:
   - Ensure BAML CLI is up to date: `baml update-client`
   - Check BAML source files for errors
   - Regenerate client code after changes

3. **Build Issues**:
   - Clean and rebuild language clients
   - Check language-specific toolchain versions
   - Verify all dependencies are installed

4. **Environment Variables**:
   - Set up Infisical correctly
   - Verify API keys are present
   - Check `.env` file if not using Infisical

5. **Test Timeouts**:
   - Adjust timeout settings in test configurations
   - Consider rate limiting for API calls
   - Use appropriate test annotations/decorators

### OpenAPI Server Testss

1. Navigate to the test directory:
   - `cd engine/baml-runtime/tests/`

2. Run tests with:

- `cargo test --features internal`

This will run the baml-serve server locally, and ping it. You may need to change the PORT variable for your new test to use a different port (we don’t have a good way of autoselecting a port).

> Instructions for testing a particular OpenAPI client are TBD.

## Grammar Testing

1. Test new syntax in the [pest playground](https://pest.rs/).

2. Update the following:

   - **Pest grammar**: Modify the `.pest` file.
   - **AST parsing**: Update the AST parsing of the new grammar.
   - **IR**: Modify the Intermediate Representation (IR).

3. Ensure all tests pass:

   - Run `cargo test` in `engine/baml-lib/`
   - Ensure integration tests still pass.

4. Modify the grammar for the [PromptFiddle.com](http://PromptFiddle.com) syntax rendering that uses Lezer, if necessary.


## VSCode Extension Testing

This requires a macos or linux machine, since we symlink some playground files between both [PromptFiddle.com](http://PromptFiddle.com) website app, and the VSCode extension itself.

**Note:** If you are just making changes to the VSCode extension UI, you may want to go to the section: [Testing PromptFiddle.com](#testing-prompfiddlecom).

1. Navigate to the TypeScript directory:
   - `cd typescript/`

2. Install dependencies:
   - `pnpm i`

3. Build and launch the extension:
   - `npx turbo build --force`
   - Open VSCode and go to the Run and Debug section (play button near the extensions button).
   - Select "Launch VSCode Extension" and press the play button.
     - This will open a new VSCode window in Debug mode.
     - You can open a simple BAML project in this window (refer to our quickstart guide to set up a simple project, or clone the `baml-examples` repository).

4. Reload the extension:
   - Use `Command + Shift + P` to reload the extension when you change any core logic.
   - Alternatively, close and reopen the playground if you rebuild the playground.


To rebuild the playground UI:

1. `cd typescript/vscode-ext/packages/web-panel`
2. `pnpm build`
3. Close and open the playground in your “Debug mode VSCode window”

## Testing [promptfiddle.com](http://promptfiddle.com)

This is useful if you want to iterate faster on the Extension UI, since it supports hot-reloading.

1. Navigate to the Fiddle Frontend directory:
   - `cd typescript/fiddle-frontend`

2. Start the dev server:
   - `pnpm dev`

3. Modify the files in `typescript/playground-common`

4. Use the `vscode-` prefixed tailwind classes to get proper colors.
