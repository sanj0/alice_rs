# Alice language specification
## Introduction
Alice is an interpreted statically typed, concatenating, stack based language with a variable table.

## Examples
For complete example scripts, see [example folder](./examples/).

```forth
"hello, world!" println
```

```forth
fun generate_greeting: string -> string {
    "hello, " let prefix: string
    prefix swap +
}
generate_greeting()
```
