function TestOllama(input: string) -> string {
  client Ollama
  prompt #"
    Write a nice haiku about {{ input }}
  "#
}