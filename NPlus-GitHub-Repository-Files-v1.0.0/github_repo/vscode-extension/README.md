# N+ VS Code Extension v1.0

Provides `.npl` language recognition, syntax highlighting, snippets, N+ file icons, and Run/Check commands.

## Development

```powershell
cd vscode-extension
npm install
npm run compile
```

To test it, open this folder in VS Code and press `F5` to start an Extension Development Host.

The Run command first looks for a compiler built inside the project workspace:

```text
compiler/target/debug/nplus.exe
compiler/target/release/nplus.exe
```

and falls back to `nplus.exe` on PATH.


## N+ file icon

The official N+ logo is included as the `.npl` file icon. After installing the extension, open **Command Palette → Preferences: File Icon Theme → N+ File Icons**. Any file ending in `.npl` will then show the N+ logo in the Explorer.
