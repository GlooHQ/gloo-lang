function TypeBuilderFn() -> string { 
  client "openai/gpt-4o-mini"
  prompt #"Hello"#
}

class NonDynamic {
  a string
  b string
}

test AttemptToModifyNonDynamicClass {
  functions [TypeBuilderFn]
  type_builder {
    dynamic NonDynamic {
      c string
    }
  }
  args {
    from_text "Test"
  }
}

// error: Error validating: Type 'NonDynamic' does not contain the `@@dynamic` attribute so it cannot be modified in a type builder block
//   -->  tests/wrong_dynamic_block_target.baml:14
//    | 
// 13 |   type_builder {
// 14 |     dynamic NonDynamic {
// 15 |       c string
// 16 |     }
//    | 
