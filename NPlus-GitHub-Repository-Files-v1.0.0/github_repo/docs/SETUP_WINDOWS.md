# N+ Windows setup

## 1. Install prerequisites

Install:

- VS Code
- Rust toolchain (`rustup`, `cargo`, `rustc`)
- Node.js LTS and npm
- Git

## 2. Build N+

Open PowerShell in the N+ folder:

```powershell
.\setup-windows.ps1
```

Or manually:

```powershell
cd compiler
cargo build --release
cargo run -- run ..\examples\hello.npl
```

## 3. Use the `.npl` files

Open `examples\hello.npl` in VS Code.

## 4. Install the VS Code extension locally

Open a second terminal:

```powershell
cd vscode-extension
npm install
npm run compile
```

Press `F5` in VS Code to open an Extension Development Host.

The extension registers `.npl`, syntax highlighting, snippets, and N+ Run/Check commands.

## 5. Put the compiler on PATH (optional)

The built compiler is:

```text
compiler\target\release\nplus.exe
```

A simple approach is to copy it into a folder already on PATH, or add its directory to your Windows user PATH.
