function FnOutputInt(input: string) -> int {
  client GPT35
  prompt #"
    Return the integer 5: {{ ctx.output_format}}
  "#
}

test FnOutputInt {
  functions [FnOutputInt]
  args {
    input "example input"
  }
}
