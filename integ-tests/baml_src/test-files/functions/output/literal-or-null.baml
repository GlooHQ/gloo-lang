class ClassForNullLiteral {
  a "hi"
}

function NullLiteralClassHello(s: string) -> ClassForNullLiteral {
    client GPT35
    prompt #"
        Return the empty object: {}.
    "#
}

test NullLiteralClassHello {
    functions [NullLiteralClassHello]
    args { s "unused" }
}