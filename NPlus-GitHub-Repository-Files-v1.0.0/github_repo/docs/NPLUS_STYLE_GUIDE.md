# N+ Style Guide v1.0

## Statements

Preferred:

```npl
print("Hello")*
```

Alternative:

```npl
print("Hello")!
```

## Names

Use `snake_case` for variables and functions. Use clear names instead of abbreviations.

## Strings

String interpolation uses braces:

```npl
let players = 10*
print("Players: {players}")*
```

## Terminal

N+ CLI output uses a compact status line, a separator, and a final success/failure line. Program output remains plain and readable between the boundaries.
