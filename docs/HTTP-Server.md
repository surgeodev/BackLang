# HTTP Server

BackLang has a built-in HTTP server powered by **Axum** (Rust async HTTP framework). The server declares routes inline as part of the language.

## Server Declaration

```bl
server app {
    port: 8080;
    cors: true;
}
```

| Field | Default | Description |
|-------|---------|-------------|
| `port` | `3000` | TCP port to listen on |
| `host` | `"0.0.0.0"` | Bind address |
| `cors` | `false` | Enable CORS headers |

The `server` block can appear at most once per file. The server name (`app` above) is for documentation purposes.

### With host

```bl
server api {
    port: 9000;
    host: "127.0.0.1";
}
```

## Startup Initialization

Code placed in the server's outer scope (outside any endpoint) runs **before** the server starts accepting requests. This is where you open databases, create tables, and initialize variables.

```bl
import "std.db"
let dbPath = "/tmp/app.db"
db.open(dbPath)
db.execute(dbPath, "CREATE TABLE IF NOT EXISTS items (id INTEGER PRIMARY KEY, name TEXT)")

server app { port: 8080; cors: true }

endpoint GET "/api/items" {
    return {status: 200, body: db.query(dbPath, "SELECT * FROM items")}
}
```

## Endpoints

### Syntax

```bl
endpoint METHOD "/path" {
    // handler body
}
```

### Methods

| Method | Description |
|--------|-------------|
| `GET` | Retrieve data |
| `POST` | Create data |
| `PUT` | Update/replace data |
| `DELETE` | Delete data |
| `PATCH` | Partial update |

### Path Parameters

Path segments starting with `:` are extracted as parameters and available in the handler via `req.params`.

```bl
endpoint GET "/users/:id" {
    return {status: 200, body: {userId: req.params.id}}
}
```

### Example: CRUD API

```bl
import "std.db"

let db = "/tmp/app.db"
db.open(db)
db.execute(db, "CREATE TABLE IF NOT EXISTS items (id INTEGER PRIMARY KEY, name TEXT)")

server api { port: 8080; cors: true }

// List all items
endpoint GET "/api/items" {
    return {status: 200, body: db.query(db, "SELECT * FROM items")}
}

// Get single item
endpoint GET "/api/items/:id" {
    let rows = db.query(db, "SELECT * FROM items WHERE id = " + req.params.id)
    if len(rows) > 0 {
        return {status: 200, body: rows[0]}
    }
    return {status: 404, body: {error: "Not found"}}
}

// Create item
endpoint POST "/api/items" {
    db.execute(db, "INSERT INTO items (name) VALUES ('" + req.body.name + "')")
    return {status: 201, body: {ok: true}}
}

// Delete item
endpoint DELETE "/api/items/:id" {
    db.execute(db, "DELETE FROM items WHERE id = " + req.params.id)
    return {status: 200, body: {ok: true}}
}
```

## Request and Response

### The `req` Object

Inside an endpoint handler, `req` is automatically available:

| Field | Type | Description |
|-------|------|-------------|
| `req.method` | `Str` | HTTP method (GET, POST, etc.) |
| `req.path` | `Str` | Request path |
| `req.params` | `Obj` | Path parameters |
| `req.body` | `Obj` | Parsed JSON body (POST/PUT) |
| `req.query` | `Obj` | Query string parameters |

### Response Format

Use the `return` statement with an object:

```bl
return {status: 200, body: {message: "OK"}}
```

| Field | Required | Description |
|-------|----------|-------------|
| `status` | Yes | HTTP status code (number) |
| `body` | Yes | Response body (any type, serialized as JSON) |

### Status Code Examples

```bl
return {status: 200, body: data}        // OK
return {status: 201, body: {id: 1}}     // Created
return {status: 204, body: null}        // No Content
return {status: 400, body: {error: "Bad request"}}    // Bad Request
return {status: 404, body: {error: "Not found"}}      // Not Found
return {status: 500, body: {error: "Internal error"}} // Server Error
```

## Middlewares

Middlewares run before endpoint handlers. They can modify the request or short-circuit the response.

```bl
middleware logger {
    print("Request: " + req.method + " " + req.path)
}

endpoint logger GET "/api/data" {
    return {status: 200, body: {data: "protected"}}
}
```

Middlewares are declared as comma-separated names before the HTTP method keyword.

## CORS

CORS headers are added automatically to every response when `cors: true` is set in the server declaration.

```bl
server app { port: 8080; cors: true }
```

Headers set on every response:

```
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: GET, POST, PUT, DELETE, PATCH, OPTIONS
Access-Control-Allow-Headers: Content-Type, Authorization
```

Preflight `OPTIONS` requests return `200` with no body.

## SQLite Integration

The most common pattern is to combine the server with SQLite for a full REST API.

```bl
import "std.db"

let dbPath = "/tmp/api.db"
db.open(dbPath)

// Auto-create schema on startup
db.execute(dbPath, "CREATE TABLE IF NOT EXISTS products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    price REAL NOT NULL,
    stock INTEGER DEFAULT 0
)")

// Seed data
let count = db.query(dbPath, "SELECT COUNT(*) as c FROM products")
if count[0].c == 0 {
    db.execute(dbPath, "INSERT INTO products VALUES (1, 'Widget', 9.99, 100)")
    db.execute(dbPath, "INSERT INTO products VALUES (2, 'Gadget', 24.99, 50)")
}

server store { port: 8080; cors: true }

endpoint GET "/api/products" {
    return {status: 200, body: db.query(dbPath, "SELECT * FROM products")}
}

endpoint GET "/api/products/:id" {
    let rows = db.query(dbPath, "SELECT * FROM products WHERE id = " + req.params.id)
    if len(rows) > 0 {
        return {status: 200, body: rows[0]}
    }
    return {status: 404, body: {error: "Product not found"}}
}

endpoint POST "/api/products" {
    let name = req.body.name
    let price = req.body.price
    db.execute(dbPath, "INSERT INTO products (name, price) VALUES ('" + name + "', " + price + ")")
    return {status: 201, body: {ok: true}}
}
```

## Full Example

```bl
// server.bl — Complete REST API with SQLite
import "std.db"

// Initialization (runs before server start)
let DB = "/tmp/app.db"
db.open(DB)
db.execute(DB, "CREATE TABLE IF NOT EXISTS messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    text TEXT NOT NULL,
    created_at TEXT DEFAULT (datetime('now'))
)")

// Server configuration
server api {
    port: 8080;
    cors: true;
}

// Routes
endpoint GET "/api/health" {
    return {status: 200, body: {ok: true, time: "now"}}
}

endpoint GET "/api/messages" {
    let rows = db.query(DB, "SELECT * FROM messages ORDER BY id DESC")
    return {status: 200, body: rows}
}

endpoint POST "/api/messages" {
    let text = req.body.text
    if text == null || len(text) == 0 {
        return {status: 400, body: {error: "text is required"}}
    }
    db.execute(DB, "INSERT INTO messages (text) VALUES ('" + text + "')")
    return {status: 201, body: {ok: true}}
}

endpoint DELETE "/api/messages/:id" {
    db.execute(DB, "DELETE FROM messages WHERE id = " + req.params.id)
    return {status: 200, body: {ok: true}}
}
```

Run it with `bl server.bl` and test with `curl http://localhost:8080/api/health`.
