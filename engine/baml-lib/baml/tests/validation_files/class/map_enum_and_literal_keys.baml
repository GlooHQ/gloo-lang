enum MapKey {
  A
  B
  C
}

class Fields {
  e map<MapKey, string>
  l1 map<"literal", string>
  l2 map<5, string>
  l3 map<"one" | "two" | ("three" | "four"), string>
  l4 map<1 | 2 | (3 | 4), string>

  // This one should fail.
  mixed_types map<1 | "one", string>
}

// error: Error validating: Unions in map keys may only contain literals of the same type.
//   -->  class/map_enum_and_literal_keys.baml:15
//    | 
// 14 |   // This one should fail.
// 15 |   mixed_types map<1 | "one", string>
//    | 
