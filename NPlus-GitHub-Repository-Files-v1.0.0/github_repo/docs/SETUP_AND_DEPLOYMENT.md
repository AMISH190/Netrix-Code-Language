# N+ v1.0 — Setup, Development & Publishing Guide

## 1. Install prerequisites (Windows)
Install:
- VS Code: https://code.visualstudio.com/
- Rust: https://www.rust-lang.org/tools/install
- Node.js LTS: https://nodejs.org/
- Git: https://git-scm.com/download/win

Verify in a **new** PowerShell window:
```powershell
rustc --version
cargo --version
node --version
npm --version
git --version
```

## 2. Extract the project
Recommended location:
`C:\NPlus`

Then open that folder in VS Code.

## 3. Build the N+ compiler
```powershell
cd C:\NPlus\compiler
cargo build --release
```
If successful, the compiler is at:
`compiler\target\release\nplus.exe`

## 4. Run an N+ program
```powershell
cd C:\NPlus\compiler
cargo run -- run ..\examples\hello.npl
```

## 5. Check syntax without running
```powershell
cargo run -- check ..\examples\hello.npl
```

## 6. Run the REPL
```powershell
cargo run -- repl
```

## 7. Use the setup script
From the project root:
```powershell
powershell -ExecutionPolicy Bypass -File .\setup-windows.ps1
```
If PowerShell blocks the script in the future, use the same command above.

## 8. VS Code extension
```powershell
cd C:\NPlus\vscode-extension
npm install
npm run compile
```
In VS Code, press `F5` to launch an Extension Development Host. Open a `.npl` file there to test syntax highlighting, snippets, the N+ icon and extension commands.

## 9. N+ website
The language website is:
`C:\NPlus\website`
Open `index.html` for a quick local preview, or deploy the `website` folder as a static site.

Target domain: `nplus.dedyn.io`

## 10. Netrix Development website
The company site is:
`C:\NPlus\company-site`
Deploy the **contents of this folder** as a static site.

Target domain: `netrix.ice.fo`

## 11. GitHub repository
Recommended repository name:
`nplus`

From the root:
```powershell
git init
git add .
git commit -m "N+ v1.0.0"
git branch -M main
git remote add origin https://github.com/YOUR-USERNAME/nplus.git
git push -u origin main
```

Before pushing, confirm that no secrets are committed. Never commit `.env`, API keys, Discord bot tokens, database passwords, or private certificates.

## 12. Release
Create a GitHub Release named `v1.0.0`. For a source release, attach the project ZIP. For a binary release, build on each target machine/CI runner and attach the resulting binaries.

## 13. Cloudflare Pages deployment
1. Create a Cloudflare account.
2. Workers & Pages → Create application → Pages → Connect to Git.
3. Select the GitHub repository.
4. For the N+ site, set the output/root directory to `website`.
5. For the company site, either use a separate repository or a separate deployment pointing to `company-site`.
6. Deploy.
7. Add the desired custom domain in Pages → Custom domains.

For `nplus.dedyn.io` and `netrix.ice.fo`, the DNS provider must point the chosen hostname to the hosting service using the records/target shown by that host. Do not invent an IP; use the exact DNS target provided by Cloudflare Pages or your chosen host.

## 14. What belongs in GitHub
Source code and documentation belong in GitHub. Built release binaries belong in GitHub Releases. Website source belongs in the repo; the deployed website is the hosted output.
