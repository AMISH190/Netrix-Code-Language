<div align="center">

# N+

### Build Anything. Simply.

**A modern programming language by Netrix Development.**

[![Version](https://img.shields.io/badge/version-1.0.0-7C4DFF?style=for-the-badge)](#)
[![Language](https://img.shields.io/badge/language-N%2B-8B5CFF?style=for-the-badge)](#)
[![Extension](https://img.shields.io/badge/VS%20Code-.npl-6C63FF?style=for-the-badge&logo=visualstudiocode&logoColor=white)](#)
[![Built By](https://img.shields.io/badge/built%20by-Netrix%20Development-15131F?style=for-the-badge)](#)

**Simple syntax • Fast tooling • Web ready • AI ready • Developer focused**

</div>

---

## What is N+?

**N+** is a programming language and developer ecosystem designed around one idea:

> **Write less. Build more.**

N+ is being built to make it easy to create modern software without forcing developers to jump between different languages and disconnected tools for every project.

The goal is a clean language that can grow from a beginner's first program into real projects such as:

- Web applications and APIs
- Automation and CLI tools
- Discord bots
- Minecraft tooling and server utilities
- AI-powered applications
- Database-backed services
- Developer tools and utilities

N+ source files use the **`.npl`** extension.

```npl
let name = "N+"*
print("Hello from {name}")!
```

---

## Why N+?

### Clean syntax

N+ keeps the syntax approachable and uses `*` or `!` as statement terminators.

```npl
let score = 50*

if score >= 50 {
    print("Passed!")*
} else {
    print("Try again!")*
}
```

### Control flow that reads naturally

```npl
let i = 0*

while i < 3 {
    print("i = {i}")*
    i = i + 1*
}

for number in 1..6 {
    print(number)*
}
```

### Functions

```npl
fn add(a: int, b: int) -> int {
    return a + b*
}

let result = add(20, 22)*
print(result)!
```

### Web development

N+ is being designed with web development as a first-class part of the ecosystem.

```npl
import web*

web.host("127.0.0.1")*
web.listen(3000)*

web.get("/", "<h1>Hello from N+ Web</h1>")*

web.start()!
```

The long-term N+ Web roadmap includes routing, JSON APIs, static assets, request/response handling, templates, WebSockets, databases, authentication and production deployment tooling.

---

# ✨ Highlights

| Area | N+ Direction |
| --- | --- |
| Language | Clean, readable syntax |
| Files | `.npl` |
| Variables | `let`, `var`, `const` |
| Functions | `fn`, parameters, returns |
| Conditions | `if`, `else`, `else if` |
| Loops | `while`, `for`, ranges |
| Strings | Interpolation with `{value}` |
| Web | HTTP server + static web foundation |
| Tooling | CLI, formatter, checker, REPL roadmap |
| Editor | VS Code extension + N+ file icon |
| AI | N+ AI planned as a first-class developer tool |
| Ecosystem | Web, Discord, Minecraft, databases and more |

---

# 🚀 Quick Start

## 1. Download N+
## N+ v1.0.0

## [ Download N+ for Windows ]

## The latest Windows executable is available in the
## v1.0.0 GitHub Release.

## Download:
## nplus.exe

# 🧪 Current Language Examples

## Variables

```npl
let username = "Developer"*
let age = 13*
print("Welcome {username}")*
```

## Conditions

```npl
let online = true*

if online {
    print("Server online")*
} else {
    print("Server offline")*
}
```

## Functions

```npl
fn greet(name: string) {
    print("Hello {name}!")*
}

greet("N+ Developer")!
```

## Arithmetic

```npl
let total = 10 * 5*
print(total)*
```

## Loops

```npl
for i in 1..5 {
    print("Number: {i}")*
}
```

---

# 🌐 N+ Web

N+ Web is the web-development layer of the N+ ecosystem.

The vision is to make this kind of project possible with one language and one toolchain:

```text
my-app/
├── src/
│   └── main.npl
├── public/
│   ├── index.html
│   ├── style.css
│   └── app.js
├── tests/
└── nplus.toml
```

Planned web capabilities include:

- HTTP routing
- REST APIs
- JSON
- Static file serving
- HTML templates
- Request and response objects
- Middleware
- Cookies and sessions
- WebSockets
- Database integrations
- Authentication
- Development server
- Production server
- Frontend interoperability

---

# 🧩 VS Code Support

N+ uses the `.npl` extension and includes a VS Code extension project.

Create a file such as:

```text
website.npl
```

The N+ extension is designed to provide:

- `.npl` language recognition
- Syntax highlighting
- N+ snippets
- Run/check commands
- N+ file icon using the official N+ branding
- Language-server integration as the tooling matures
- N+ AI integration as the AI subsystem matures

### Extension development

```bash
cd vscode-extension
npm install
npm run compile
```

Then open the extension project in VS Code and press **F5** to launch an Extension Development Host.

---

# 🛠️ Project Structure

```text
NPlus-Starter/
│
├── compiler/             # N+ compiler / interpreter implementation
├── runtime/              # Runtime components
├── standard-library/     # Core libraries
├── package-manager/      # Package tooling
├── language-server/      # Language Server / editor intelligence
├── debugger/             # Debugger tooling
├── ai/                   # N+ AI subsystem
├── packages/             # Ecosystem packages
├── vscode-extension/     # VS Code support
├── website/               # N+ website
├── company-site/          # Netrix Development website
├── playground/            # Playground infrastructure
├── examples/              # Example N+ programs
├── tests/                 # Language and compiler tests
├── docs/                  # Documentation and specifications
└── .github/               # CI / release automation
```

---

# 🧠 N+ AI

N+ is designed to have AI-assisted development as part of its ecosystem, not as an afterthought.

The planned N+ AI workflow includes:

```text
Explain code
Fix compiler errors
Generate code
Generate tests
Refactor code
Optimize code
Generate documentation
Understand project context
```

The language itself remains usable without AI. AI features are an optional productivity layer.

---

# 🗺️ Roadmap

## N+ 1.x

- [x] `.npl` language format
- [x] Variables
- [x] Expressions
- [x] String interpolation
- [x] `*` and `!` statement terminators
- [x] Functions foundation
- [x] `if / else`
- [x] `while`
- [x] `for` / ranges
- [x] Initial N+ Web foundation
- [x] VS Code extension foundation
- [ ] Full static type checker
- [ ] Production-ready package manager
- [ ] Full language server
- [ ] Debugger
- [ ] Expanded Web framework
- [ ] Database layer
- [ ] Discord SDK
- [ ] Minecraft SDK
- [ ] N+ AI assistant

## N+ 2.x and beyond

- Native compilation pipeline
- NIR intermediate representation
- Cross compilation
- WebAssembly
- Advanced optimization
- Full-stack application tooling
- Rich package ecosystem
- Cloud and deployment tooling

---

# 🏗️ Development Philosophy

N+ is being developed around a few simple rules:

### Easy to learn

Beginners should be able to understand their first program in minutes.

### Powerful when needed

The language should grow with the developer instead of forcing them to switch languages for every new problem.

### Great tooling matters

A language is more than its syntax. Compiler errors, formatting, debugging, documentation and editor integration are part of the experience.

### Performance without unnecessary complexity

N+ aims for fast tooling and efficient execution while keeping everyday code approachable.

### One ecosystem

The long-term goal is a single N+ developer experience for language, packages, web, AI, servers and tooling.

---

# 🤝 Contributing

N+ is an evolving project, and contributions are welcome once the repository contribution workflow is established.

For now, useful contributions include:

- Bug reports
- Reproduction projects
- Documentation improvements
- Example programs
- Compiler tests
- VS Code tooling ideas
- Web framework ideas

Before opening a pull request, make sure the project builds and tests locally.

```bash
cargo build --release
```

---

# 🔐 Security

Please do not commit:

```text
.env
API keys
Discord tokens
Database passwords
Private certificates
Hosting credentials
```

For security issues, use the repository's private security reporting mechanism rather than publicly posting sensitive information.

---

# 📦 Release Information

**Current public target:** `N+ v1.0.0`

N+ releases are intended to provide:

```text
Source code
Windows build
CI validation
Documentation
VS Code tooling
Examples
```

As the compiler matures, additional platform builds will be added.

---

# 🌌 Netrix Development

N+ is developed by **Netrix Development** — a software and developer-tools initiative focused on creating practical technology with strong design and developer experience.

**N+ is the flagship language project.**

The broader Netrix ecosystem is intended to grow around software, web platforms, automation, AI systems, developer tools and infrastructure.

---

# 📚 Documentation

Project documentation lives in the `docs/` directory.

Recommended starting points:

```text
docs/
├── language/
├── NPLUS_WEB_GUIDE.md
├── SETUP_AND_DEPLOYMENT.md
└── ...
```

---

# ⭐ Support the Project

If N+ is interesting to you:

**Star the repository, try the language, build something with it, and share your feedback.**

Every example project and bug report helps shape the language.

---

<div align="center">

### N+

**Build Anything. Simply.**

Built with ambition by **Netrix Development**.

</div>
