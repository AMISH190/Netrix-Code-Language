# Publish N+ v1.0.0 on GitHub

The repository is configured so that pushing the `v1.0.0` tag automatically builds a Windows x64 `nplus.exe` and creates a GitHub Release containing both the `.exe` and a ZIP package.

## 1. Create the GitHub repository

Create a repository such as:

```text
nplus
```

Do not upload passwords, API keys, Discord tokens, database passwords, or `.env` files.

## 2. Open PowerShell in the N+ project root

```powershell
cd C:\NPlus\NPlus-Starter
```

## 3. Run the release helper

```powershell
.\release-v1.0.0.ps1 -RepositoryUrl "https://github.com/YOUR-USERNAME/nplus.git"
```

The script builds the compiler locally, commits the source, pushes `main`, and pushes the `v1.0.0` tag.

## 4. GitHub Actions builds the Windows executable

The workflow is:

```text
.github/workflows/release.yml
```

GitHub's Windows runner will:

1. Install stable Rust.
2. Build `compiler/target/release/nplus.exe`.
3. Run a smoke test.
4. Create `NPlus-v1.0.0-windows-x64.zip`.
5. Create the GitHub `v1.0.0` Release.
6. Attach the `.exe` and ZIP to the release.

## 5. Where to find it

Open your repository on GitHub and select:

```text
Releases → v1.0.0
```

You should see:

```text
nplus.exe
NPlus-v1.0.0-windows-x64.zip
Source code (zip)
Source code (tar.gz)
```

The website download button can later point to the GitHub release asset.
