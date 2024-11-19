enum MapKey {
  A
  B
  C
}

function InOutEnumMapKey(i1: map<MapKey, string>, i2: map<MapKey, string>) -> map<MapKey, string> {
  client "openai/gpt-4o"
  prompt #"
    Merge these: {{i1}} {{i2}}

    {{ ctx.output_format }}
  "#
}

function InOutLiteralStringMapKey(
  i1: map<"one" | "two" | ("three" | "four"), string>, 
  i2: map<"one" | "two" | ("three" | "four"), string>
) -> map<"one" | "two" | ("three" | "four"), string> {
  client "openai/gpt-4o"
  prompt #"
    Merge these:
    
    {{i1}}
    
    {{i2}}

    {{ ctx.output_format }}
  "#
}

function InOutLiteralIntMapKey(
  i1: map<1 | 2 | (3 | 4), string>, 
  i2: map<1 | 2 | (3 | 4), string>
) -> map<1 | 2 | (3 | 4), string> {
  client "openai/gpt-4o"
  prompt #"
    Merge these:
    
    {{i1}}
    
    {{i2}}

    {{ ctx.output_format }}
  "#
}
