function FnOutputLiteralString(input: string) -> "example output" {
  client GPT35
  prompt #"
    Return a string: {{ ctx.output_format}}
  "#
}

test FnOutputLiteralString {
  functions [FnOutputLiteralString]
  args {
    input "example input"
  }
}
