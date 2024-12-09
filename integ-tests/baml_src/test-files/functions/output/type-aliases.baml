type Primitive = int | string | bool | float

type List = string[]

type Graph = map<string, string[]>

type Combination = Primitive | List | Graph

function PrimitiveAlias(p: Primitive) -> Primitive {
    client "openai/gpt-4o"
    prompt r#"
      Return the given value back: {{ p }}
    "#
}

function MapAlias(m: Graph) -> Graph {
    client "openai/gpt-4o"
    prompt r#"
      Return the given Graph back:

      {{ m }}

      {{ ctx.output_format }}
    "#
}

function NestedAlias(c: Combination) -> Combination {
    client "openai/gpt-4o"
    prompt r#"
      Return the given value back:

      {{ c }}

      {{ ctx.output_format }}
    "#
}

// Test attribute merging.
type Currency = int @check(gt_ten, {{ this > 10 }})
type Amount = Currency @assert ({{ this > 0 }})

class MergeAttrs {
    amount Amount @description("In USD")
}

function MergeAliasAttributes(money: int) -> MergeAttrs {
    client "openai/gpt-4o"
    prompt r#"
      Return the given integer in the specified format:

      {{ money }}

      {{ ctx.output_format }}
    "#
}

function ReturnAliasWithMergedAttributes(money: Amount) -> Amount {
    client "openai/gpt-4o"
    prompt r#"
      Return the given integer without additional context:

      {{ money }}

      {{ ctx.output_format }}
    "#
}