class Tree {
  data int
  children Forest
}

class Forest {
  trees Tree[]
}

class BinaryNode {
  data int
  left BinaryNode?
  right BinaryNode?
}

function BuildTree(input: BinaryNode) -> Tree {
  client GPT35
  prompt #"
    Given the input binary tree, transform it into a generic tree using the given schema.

    INPUT:
    {{ input }}

    {{ ctx.output_format }}    
  "#
}

test TestTree {
  functions [BuildTree]
  args {
    input {
      data 2
      left {
        data 1
        left null
        right null
      }
      right {
        data 3
        left null
        right null
      }
    }
  }
}