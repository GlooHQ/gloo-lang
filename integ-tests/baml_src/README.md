# BAML Source Files

This directory contains the BAML source files used by all integration tests. It defines the clients, generators, and test cases that are used across all language implementations.

## Directory Structure

```
baml_src/
├── clients.baml         # Main client definitions
├── generators.baml      # Generator functions
├── test-files/         # Test-specific BAML files
│   ├── providers/      # Provider-specific tests
│   ├── testing_pipeline/ # Pipeline test cases
│   └── ...
├── formatter/          # Formatter-specific tests
└── fiddle-examples/    # Example BAML files for testing
```

## Adding New Tests

### 1. Client Tests
Add new client tests in `clients.baml`:
```baml
client<llm> TestGPT {
  provider openai
  retry_policy TestRetry
  options {
    model gpt-4
    api_key env.OPENAI_API_KEY
  }
}

retry_policy TestRetry {
  max_retries 3
  strategy {
    type exponential_backoff
  }
}
```

### 2. Provider Tests
Add provider-specific tests in `test-files/providers/`:
```baml
// 1. First, define your client
client<llm> TestAnthropic {
  provider anthropic
  options {
    model claude-3-haiku-20240307
    api_key env.ANTHROPIC_API_KEY
    max_tokens 1000
  }
}

// 2. Create a function that uses the client
function TestAnthropicCompletion(input: string) -> string {
  client TestAnthropic
  prompt #"""
    Respond to this input with a simple response.

    Input: {{input}}
  """#
}

// 3. Create a test for the function
test TestAnthropicCompletion {
  functions [TestAnthropicCompletion]
  args {
    input #"What is the capital of France?"#
  }
  assert response == "Paris"
}

// You can also test error cases
function TestAnthropicError(input: string) -> string {
  client TestAnthropic
  prompt #"""
    This is a test for error handling.
    {{input}}
  """#
}

test TestAnthropicError {
  functions [TestAnthropicError]
  args {
    input #"Test input"#
  }
  expect_error true
}
```

## Using Generators

Generators in `generators.baml` are used to define language-specific output:

```baml
generator lang_typescript {
  output_type typescript
  output_dir "../typescript"
  version "0.72.0"
}

generator lang_python {
  output_type python/pydantic
  output_dir "../python"
  version "0.72.0"
}

generator lang_ruby {
  output_type ruby/sorbet
  output_dir "../ruby"
  version "0.72.0"
}
```

## Test Categories

1. **Provider Tests**
   - Located in `test-files/providers/`
   - Test specific provider implementations (OpenAI, Anthropic, etc.)
   - Each provider has its own BAML file

2. **Pipeline Tests**
   - Located in `test-files/testing_pipeline/`
   - Test BAML pipeline functionality
   - Include chain tests, fallback tests, etc.

3. **Formatter Tests**
   - Located in `formatter/`
   - Test BAML's formatting capabilities
   - Include structured output tests


## Best Practices

1. **Organizing Tests**
   - Group related tests in the same file
   - Use descriptive names for test functions
   - Add comments explaining test purpose

2. **Writing Test Cases**
   - Test both success and failure cases
   - Include validation rules
   - Test edge cases and error handling

3. **Using Generators**
   - Define output types for each language
   - Specify correct output directories
   - Maintain version consistency

4. **Adding New Providers**
   - Create new file in `test-files/providers/`
   - Test provider-specific features
   - Include authentication tests

## Common Patterns

1. **Fallback Testing**
```baml
client<llm> Resilient {
  provider baml-fallback
  options {
    strategy [
      GPT4
      GPT35
      Claude
    ]
  }
}
```

2. **Round-Robin Testing**
```baml
client<llm> RoundRobin {
  provider baml-round-robin
  options {
    start 0
    strategy [
      GPT35
      Claude
    ]
  }
}
```

3. **Retry Policies**
```baml
retry_policy TestRetry {
  max_retries 3
  strategy {
    type exponential_backoff
  }
}

client<llm> RetryTest {
  provider openai
  retry_policy TestRetry
  options {
    model gpt-4
    api_key env.OPENAI_API_KEY
  }
}
```