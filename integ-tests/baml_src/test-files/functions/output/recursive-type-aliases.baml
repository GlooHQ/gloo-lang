// Simple alias that points to recursive type.
class LinkedListAliasNode {
    value int
    next LinkedListAliasNode?
}

type LinkedListAlias = LinkedListAliasNode

function AliasThatPointsToRecursiveType(list: LinkedListAlias) -> LinkedListAlias {
    client "openai/gpt-4o"
    prompt r#"
      Return the given linked list back:
      
      {{ list }}
      
      {{ ctx.output_format }}
    "#
}
