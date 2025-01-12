function TestVertex(input: string) -> string {
  client Vertex
  prompt #"
    Write a nice short story about {{ input }}
  "#
}

function TestVertexWithSystemInstructions() -> string {
  client Vertex
  prompt #"{{_.role("system")}} You are a helpful assistant
  {{_.role("user")}} Write a poem about llamas
  "#
}
