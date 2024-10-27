function TestNamedArgsLiteralString(myString: "My String") -> string {
  client GPT35
  prompt #"
    Return this value back to me: {{myString}}
  "#
}

test TestFnNamedArgsLiteralString {
  functions [TestNamedArgsLiteralString]
  args {
    myString "My String"
  }
}