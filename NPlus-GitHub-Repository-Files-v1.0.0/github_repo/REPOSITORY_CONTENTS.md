# N+ Repository Contents

This repository contains the N+ source project and supporting tools. Compiled release binaries such as `nplus.exe` belong in GitHub Releases, not in the source repository.

## Main directories

- `compiler/` — N+ compiler/interpreter source
- `vscode-extension/` — VS Code language support, snippets, syntax highlighting and `.npl` file icon
- `standard-library/` — standard library documentation/scaffolding
- `packages/` — ecosystem package area
- `ai/` — N+ AI scaffolding/documentation
- `language-server/` — LSP scaffolding/documentation
- `debugger/` — debugger scaffolding/documentation
- `runtime/` — runtime scaffolding/documentation
- `package-manager/` — package-manager scaffolding/documentation
- `examples/` — `.npl` examples and N+ Web demo
- `tests/` — language tests
- `website/` — N+ website source
- `company-site/` — Netrix Development website source
- `docs/` — language and project documentation
- `.github/workflows/` — CI and release automation

## Release binaries

Do not commit `nplus.exe` to the repository. Build it on Windows with `cargo build --release` and attach the executable to the GitHub `v1.0.0` Release.
