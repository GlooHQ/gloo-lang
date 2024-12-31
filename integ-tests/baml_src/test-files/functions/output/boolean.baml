function FnOutputBool(input: string) -> bool {
  client GPT35
  prompt #"
    Return "true"
  "#
}

test FnOutputBool {
  functions [FnOutputBool]
  args {
    input "example input"
  }
}
