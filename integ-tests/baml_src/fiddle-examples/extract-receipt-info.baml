class ReceiptItem {
  name string
  description string?
  quantity int
  price float
}

class ReceiptInfo {
    items ReceiptItem[]
    total_cost float?
    venue "barisa" | "ox_burger"
}

function ExtractReceiptInfo(email: string, idx: int) -> ReceiptInfo {
  client GPT4o
  prompt #"
    {{idx}}: Given the receipt below:

    ```
    {{email}}
    ```

    {{ ctx.output_format }}
  "#
}

test TestName {
  functions [ExtractReceiptInfo]
  args {
    email #"
      04/14/2024 1:05 pm
      Venue: Ox Burger
      Ticket: 220000082489
      Register: Shop Counter
      Employee: Connor
      Customer: Sam
      Item  #  Price
      Guide leash (1 Pair) uni UNI
      1 $34.95
      The Index Town Walls
      1 $35.00
      Boot Punch
      3 $60.00
      Subtotal $129.95
      Tax ($129.95 @ 9%) $11.70
      Total Tax $11.70
      Total $141.65
    "#
    reason "curiosity"
  }
}
