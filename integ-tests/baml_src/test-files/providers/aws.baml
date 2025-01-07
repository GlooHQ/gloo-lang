function TestAws(input: string) -> string {
  client AwsBedrock
  prompt #"
    Write a nice short story about {{ input }}
  "#
}

function TestAwsInvalidRegion(input: string) -> string {
  client AwsBedrockInvalidRegion
  prompt #"
    Write a nice short story about {{ input }}
  "#
}

function TestAwsInvalidAccessKey(input: string) -> string {
  client AwsBedrockInvalidAccessKey
  prompt #"
    Write a nice short story about {{ input }}
  "#
}

function TestAwsInvalidProfile(input: string) -> string {
  client AwsBedrockInvalidProfile
  prompt #"
    Write a nice short story about {{ input }}
  "#
}

function TestAwsInvalidSessionToken(input: string) -> string {
  client AwsBedrockInvalidSessionToken
  prompt #"
    Write a nice short story about {{ input }}
  "#
}