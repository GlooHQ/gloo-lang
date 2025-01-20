function ExtractResume(from_text: string) -> Resume { 
  client "openai/gpt-4o-mini"
  prompt #"Hello"#
  type_builder {
    class Foo {
      foo string
    }
    dynamic Bar {
      bar int
    }
  }
}

// error: Error validating: Only tests may have a type_builder block.
//   -->  tests/invalid_dynamic_types.baml:4
//    | 
//  3 |   prompt #"Hello"#
//  4 |   type_builder {
//  5 |     class Foo {
//  6 |       foo string
//  7 |     }
//  8 |     dynamic Bar {
//  9 |       bar int
// 10 |     }
// 11 |   }
//    | 
