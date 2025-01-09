function PromptTestOpenAI(input: string) -> string {
  client GPT35
  prompt #"
    Write a nice haiku about {{ input }}
  "#
}

function TestOpenAILegacyProvider(input: string) -> string {
  client GPT35LegacyProvider
  prompt #"
    Write a nice haiku about {{ input }}
  "#
}

function TestOpenAIShorthand(input: string) -> string {
  client GPT35
  prompt #"
    Write a nice short story about {{ input }}
  "#
}