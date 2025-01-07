function TestAzure(input: string) -> string {
  client GPT35Azure
  prompt #"
    Write a nice haiku about {{ input }}
  "#
}

client GPT35AzureFailed {
  provider azure-openai
  options {
    resource_name "west-us-azure-baml-incorrect-suffix"
    deployment_id "gpt-35-turbo-default"
    api_key env.AZURE_OPENAI_API_KEY
  }
}

function TestAzureFailure(input: string) -> string {
  client GPT35AzureFailed
  prompt #"
    Write a nice haiku about {{ input }}
  "#
}