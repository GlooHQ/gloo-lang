function TestVertex(input: string) -> string {
  client Vertex
  prompt #"
    Write a nice short story about {{ input }}
  "#
}