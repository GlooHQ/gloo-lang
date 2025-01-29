

class MemoryObject {
  id string
  name string
  description string
}

class ComplexMemoryObject {
  id string
  name string
  description string
  metadata (string | int | float)[] @description(#"
    Additional metadata about the memory object, which can be a mix of types.
  "#)
}

class TestMemoryOutput {
  items (MemoryObject | ComplexMemoryObject)[] @description(#"
    Add 10 items, which can be either simple MemoryObjects or more complex MemoryObjects with metadata.
  "#)
  more_items (MemoryObject | ComplexMemoryObject)[] @description(#"
    Add 3 more items, which can be either simple MemoryObjects or more complex MemoryObjects with metadata.
  "#)
}


function TestMemory(input: string) -> TestMemoryOutput {
  client GPT35
  prompt #"
    Return a json blob that matches the schema:
    {{ ctx.output_format }}
  "#
}

test TestName {
  functions [TestMemory]
  args {
    input #"
      hello world
    "#
  }
}
