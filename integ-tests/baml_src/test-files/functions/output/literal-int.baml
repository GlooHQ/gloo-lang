function FnOutputLiteralInt(input: string) -> 5 {
  client GPT35
  prompt #"
    Return an integer: {{ ctx.output_format}}
  "#
}

test FnOutputLiteralInt {
  functions [FnOutputLiteralInt]
  args {
    input "example input"
  }
}
