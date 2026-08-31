# N+ Web v0.3

N+ Web is the built-in development web stack for N+. The starter server is dependency-free and intended for local development and learning.

## Server

```npl
web.host("127.0.0.1")*
web.listen(3000)*
web.start()!
```

## Routes

```npl
web.get("/", "<h1>Hello</h1>")*
web.get("/api/status", "{\"ok\":true}")*
web.post("/api/echo", "{\"ok\":true}")*
```

## Static files

```npl
web.static("./public")*
```

`/` serves `index.html`. CSS, JavaScript, images, SVG, JSON, WASM and common text assets get useful MIME types.

## Helpers

`web.html(value)` and `web.json(value)` mark the intent of a response in source code.

## Next production milestones

Request objects, body parsing, templating, middleware, cookies, sessions, WebSockets, HTTPS, async workers, secure headers and a real production HTTP stack should be added before N+ Web is marketed as production-ready.
