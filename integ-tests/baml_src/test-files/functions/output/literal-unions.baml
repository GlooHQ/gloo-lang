function LiteralUnionsTest(input: string) -> 1 | true | "string output" {
  client GPT35
  prompt #"
    Return one of these values: 
    {{ctx.output_format}}
  "#
}

test LiteralUnionsTest {
  functions [LiteralUnionsTest]
  args {
    input "example input"
  }
}
