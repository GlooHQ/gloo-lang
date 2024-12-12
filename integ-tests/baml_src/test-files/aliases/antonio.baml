class Foo2 {
    bar int
    baz string
    sub Subthing @assert( {{ this.bar == 10}} ) | null
}

class Foo3 {
    bar int
    baz string
    sub Foo3 | null
}

type Subthing = Foo2 @assert( {{ this.bar == 10 }})

function RunFoo2(input: Foo3) -> Foo2 {
    client Claude
    prompt #"Generate a Foo2 wrapping 30. Use {{ input }}.
       {{ ctx.output_format }}
    "#
}