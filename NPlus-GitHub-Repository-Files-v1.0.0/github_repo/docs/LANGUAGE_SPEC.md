# N+ Language Specification — v0.1 draft

## Source files
`.npl`

## Statement terminators
Both `*` and `!` may terminate a statement in this starter.

```npl
let x = 10*
print(x)!
```

The lexer disambiguates a trailing terminator from multiplication using expression-start lookahead.

## Variables
```npl
let name = "N+"*
var score = 0*
const VERSION = "0.1"*
score = score + 1*
```

## Functions
```npl
fn greet(name) {
    print("Hello {name}!")*
}

greet("developer")*
```

## Conditions
```npl
if score >= 10 {
    print("great")*
} else {
    print("keep going")*
}
```

## Operators
Arithmetic: `+ - * / %`
Comparison: `== != > >= < <=`
Logical: `and or not`

## Comments
```npl
// line comment
/* block comment */
```

## Planned core keywords
`for`, `while`, `loop`, `type`, `import`, `from`, `try`, `catch`, `async`, `await`, `spawn`, `match`, `test`, `expect`.


## v1.0 implementation status

The language specification may list future keywords for ecosystem planning. The v1.0 interpreter currently prioritizes variables, assignment, expressions, functions, conditionals, and the REPL. Loops, arrays, objects, async and package APIs are staged for subsequent milestones.

## v0.2 control flow

### While
```npl
let score = 0*
while score < 3 {
    print(score)*
    score = score + 1*
}
```

### For ranges
```npl
for i in 1..6 {
    print(i)*
}
```

Ranges are end-exclusive: `1..6` produces `1, 2, 3, 4, 5`.

### Else-if
```npl
if score > 10 {
    print("high")*
} else if score > 0 {
    print("low")*
} else {
    print("zero")*
}
```

### Functions
Functions can have optional parameter and return-type annotations.
```npl
fn add(a: int, b: int) -> int {
    return a + b*
}

print(add(2, 3))!
```

## N+ Web

The first N+ Web layer is intentionally dependency-free and built into the interpreter.

```npl
web.listen(3000)*
web.get("/", "<h1>Hello from N+ Web</h1>")*
web.start()!
```

To serve an existing static website directory:
```npl
web.serve("website", 3000)!
```

`web.get` creates static GET routes. `web.start` runs the local HTTP server. `web.serve` provides a simple static-file server with path traversal protection.

## N+ Web v0.3

Built-in development web functions:
- `web.host(string)`
- `web.listen(number)`
- `web.get(path, response)`
- `web.post(path, response)`
- `web.static(directory)`
- `web.html(string)`
- `web.json(string)`
- `web.start()`

The development server supports basic GET/POST routing, static files, MIME types, JSON-style responses, and CORS for local development.
