function TestGemini(input: string) -> string {
  client Gemini
  prompt #"
    Write a nice short story about {{ input }}
  "#
}

function TestGeminiSystem(input: string) -> string {
  client Gemini
  prompt #"
    {{ _.role('system') }}

    Write a nice short story about {{ input }}
  "#
}

test TestName {
  functions [TestGeminiSystem]
  args {
    input #"
      hello world
    "#
  }
}
