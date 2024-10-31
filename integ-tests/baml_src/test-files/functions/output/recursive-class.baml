class Node {
  data int
  next Node?
}

class LinkedList {
  head Node?
  len int
}

function BuildLinkedList(input: int[]) -> LinkedList {
  client GPT35
  prompt #"
    Build a linked list from the input array of integers.

    INPUT:
    {{ input }}

    {{ ctx.output_format }}    
  "#
}

test TestLinkedList {
  functions [BuildLinkedList]
  args {
    input [1, 2, 3, 4, 5]
  }
}
