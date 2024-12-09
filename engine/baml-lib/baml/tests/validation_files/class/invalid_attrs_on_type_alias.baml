type DescNotAllowed = string @description("This is not allowed")

type AliasNotAllowed = float @alias("Alias not allowed")

type SkipNotAllowed = float @skip

// error: Error validating: type aliases may only have check and assert attributes
//   -->  class/invalid_attrs_on_type_alias.baml:1
//    | 
//    | 
//  1 | type DescNotAllowed = string @description("This is not allowed")
//    | 
// error: Error validating: type aliases may only have check and assert attributes
//   -->  class/invalid_attrs_on_type_alias.baml:3
//    | 
//  2 | 
//  3 | type AliasNotAllowed = float @alias("Alias not allowed")
//    | 
// error: Error validating: type aliases may only have check and assert attributes
//   -->  class/invalid_attrs_on_type_alias.baml:5
//    | 
//  4 | 
//  5 | type SkipNotAllowed = float @skip
//    | 
