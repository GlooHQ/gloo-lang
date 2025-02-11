class DynamicClassOne {
  @@dynamic
}

enum DynEnumOne {
  @@dynamic
}

enum DynEnumTwo {
  @@dynamic
}

class SomeClassNestedDynamic {
  hi string
  @@dynamic

}

class DynamicClassTwo {
  hi string
  some_class SomeClassNestedDynamic
  status DynEnumOne
  @@dynamic
}

function DynamicFunc(input: DynamicClassOne) -> DynamicClassTwo {
  client GPT35
  prompt #"
    Please extract the schema from
    {{ input }}

    {{ ctx.output_format }}
  "#
}

class DynInputOutput {
  testKey string
  @@dynamic
}

function DynamicInputOutput(input: DynInputOutput) -> DynInputOutput {
  client GPT35
  prompt #"
    Here is some input data:
    ----
    {{ input }}
    ----

    Extract the information.
    {{ ctx.output_format }}
  "#
}

function DynamicListInputOutput(input: DynInputOutput[]) -> DynInputOutput[] {
  client GPT35
  prompt #"
    Here is some input data:
    ----
    {{ input }}
    ----

    Extract the information.
    {{ ctx.output_format }}
  "#
}



class DynamicOutput {
  @@dynamic
}

function MyFunc(input: string) -> DynamicOutput {
  client GPT35
  prompt #"
    Given a string, extract info using the schema:

    {{ input}}

    {{ ctx.output_format }}
  "#
}

function ClassifyDynEnumTwo(input: string) -> DynEnumTwo {
  client GPT35
  prompt #"
    Given a string, extract info using the schema:

    {{ input}}

    {{ ctx.output_format }}
  "#
}

class DynamicTypeBuilderClass {
  a int
  b int
  c int
  @@dynamic
}

enum DynamicTypeBuilderCategory {
  FeatureRequest
  CancelOrder
  @@dynamic
}

function UseDynamicTbClass(input: string) -> DynamicTypeBuilderClass {
  client GPT35
  prompt #"
    Extract information from this input:

    {{ input}}

    {{ ctx.output_format }}
  "#
}

function UseDynamicTbEnum(input: string) -> DynamicTypeBuilderCategory {
  client GPT35
  prompt #"
    Classify the input:

    {{ input}}

    {{ ctx.output_format }}
  "#
}
