function TestNamedArgsLiteralInt(myInt: 1) -> string {
  client GPT35
  prompt #"
    Return this value back to me: {{myInt}}
  "#
}

test TestFnNamedArgsLiteralInt {
  functions [TestNamedArgsLiteralInt]
  args {
    myInt 1
  }
}