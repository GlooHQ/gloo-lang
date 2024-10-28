class Foo {
  foo int
  bar string
  @@check(foo_and_bar, {{ this.string|length < this.foo }})
}

class Foo2 {
  foo int
  bar string
  @@assert({{ this.string|length < this.foo }})
}

class Foo3 {
  foo int @check(foo_check, {{ this }})
  bar string @assert(hi, {{ this }}) @check(hi, {{ this }})
  @@assert({{ this.string|length < this.foo }})
}
