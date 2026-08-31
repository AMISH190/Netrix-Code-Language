# N+ Web v0.3 Guide

N+ Web aims to make a complete website project feel like one small N+ application.

## Recommended project structure

```text
my-site/
├── src/
│   └── main.npl
├── public/
│   ├── index.html
│   ├── style.css
│   └── app.js
├── views/
├── assets/
└── nplus.toml
```

## Example

```npl
web.host("127.0.0.1")*
web.listen(3000)*

web.get("/", "<h1>Home</h1>")*
web.get("/api/status", "{\"ok\":true}")*
web.static("./public")*

web.start()!
```

## What v0.3 includes

- HTML pages from routes
- GET and POST route registration
- JSON-style API responses
- Static HTML/CSS/JS/image/SVG/WASM assets
- Local development server
- MIME type detection
- Basic CORS header
- Friendly terminal startup output

## What comes next

The next major Web milestone is a real request/response model: `req.method`, `req.path`, query parameters, headers, cookies, parsed JSON bodies, status codes, redirects and typed responses. After that, add templates, middleware, WebSockets, HTTPS and async runtime integration.
