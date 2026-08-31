# N+

**N+ is a clean, friendly, high-performance programming language project by Netrix Development.**

> Build anything. Simply.

## Features in v1.0
- `.npl` source files
- `*` or `!` statement terminators
- variables, functions and conditionals
- string interpolation: `"Hello {name}"`
- REPL
- friendly terminal UI
- readable `NPLxxxx` diagnostics
- VS Code extension starter
- website starter

## Quick start on Windows

Requirements:
- Rust (rustc + cargo)
- Node.js + npm

Run the setup script from PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -File .\setup-windows.ps1
```

Then:

```powershell
cd compiler
cargo run -- run ..\examples\hello.npl
```

Or check a file:

```powershell
cargo run -- check ..\examples\hello.npl
```

## Example

```npl
let name = "N+"*
print("Hello from {name}")!
```

## Repository layout

```text
compiler/            Rust compiler + interpreter
vscode-extension/    VS Code support for .npl
standard-library/    N+ standard library roadmap
packages/            Ecosystem packages
ai/                  N+ AI roadmap
language-server/     NPLS roadmap
debugger/            Debugger roadmap
docs/                Language and project documentation
website/             Official project website
examples/            Example N+ programs
```

## Development

N+ v1 is intentionally small. The goal is to stabilize the language core before adding advanced packages and native compilation.

## License

MIT

Developed by **Netrix Development**.

## Included company site
`company-site/` contains the Netrix Development website for `netrix.ice.fo`.

## Deployment guide
See `docs/SETUP_AND_DEPLOYMENT.md` for Windows setup, compiler builds, VS Code extension testing, GitHub publishing, and website deployment.

## N+ v0.2 additions

- `if / else if / else`
- `while` loops
- `for item in start..end` range loops
- typed function parameter/return annotations
- dotted built-ins such as `web.listen()`
- built-in N+ Web server: `web.listen`, `web.get`, `web.start`, `web.serve`

Example:

```npl
web.listen(3000)*
web.get("/", "<h1>Hello from N+ Web</h1>")*
web.start()!
```
