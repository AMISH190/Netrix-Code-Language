# N+ Build Map

`compiler/` — Rust compiler/interpreter.
`runtime/` — future runtime.
`standard-library/` — core libraries.
`package-manager/` — future `npack` tool.
`language-server/` — future NPLS service.
`debugger/` — future debugger.
`ai/` — future N+ AI engine.
`packages/` — ecosystem packages such as web, Discord, Minecraft and databases.
`vscode-extension/` — VS Code support.
`docs/` — language specification and design docs.
`examples/` — N+ programs.
`tests/` — compiler/runtime tests.
`playground/` — future playground.
`website/` — future official website.

## N+ Web v0.2

The interpreter now includes a dependency-free local HTTP development server.

```powershell
nplus web examples/web.npl
```

N+ Web APIs:

```npl
web.listen(3000)*
web.get("/", "<h1>Hello from N+ Web</h1>")*
web.start()!
```

To serve a static folder:

```npl
web.serve("../website", 3000)!
```


## v0.3 Web milestone
- dependency-free local HTTP server
- GET/POST routes
- static asset serving
- JSON-style API responses
- MIME type detection
