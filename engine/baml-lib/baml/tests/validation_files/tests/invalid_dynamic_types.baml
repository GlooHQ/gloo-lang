function TypeBuilderFn(from_text: string) -> Resume { 
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


test MultipleTypeBuilderBlocks {
  functions [TypeBuilderFn]
  type_builder {
    class Foo {
      foo string
    }
    dynamic Bar {
      bar int
    }
  }
  type_builder {
    class A {
      a string
    }
    dynamic B {
      b int
    }
  }
  args {
    from_text "Test"
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
// error: Error validating: Definition of multiple `type_builder` blocks in the same parent block
//   -->  tests/invalid_dynamic_types.baml:25
//    | 
// 24 |   }
// 25 |   type_builder {
// 26 |     class A {
// 27 |       a string
// 28 |     }
// 29 |     dynamic B {
// 30 |       b int
// 31 |     }
// 32 |   }
//    | 
