# Quick Start

## Hello World

Create a file `hello.bl`:

```bl
print("Hello, World!")
```

Run it:

```bash
bl hello.bl
```

## Variables & Types

```bl
let name = "Alice"         // mutable string
const PI = 3.14159         // immutable number
let is_active = true        // boolean
let items = [1, 2, 3]       // array
let user = {name: "Bob", age: 30}  // object

print(name)    // Alice
print(PI)      // 3.14159
```

## Control Flow

```bl
let x = 10

if x > 0 {
    print("positive")
} else {
    print("negative")
}

let i = 0
while i < 5 {
    print(i)
    i = i + 1
}

let fruits = ["apple", "banana", "cherry"]
for fruit in fruits {
    print(fruit)
}
```

## Functions

```bl
function add(a: Num, b: Num) -> Num {
    a + b
}

let result = add(3, 4)
print(result)  // 7
```

## HTTP Server

```bl
server app { port: 8080; cors: true }

endpoint GET "/api/hello" {
    return {status: 200, body: {message: "Hello, World!"}}
}
```

Run and visit `http://localhost:8080/api/hello`.

## SQLite Database

```bl
import "std.db"

let dbPath = "/tmp/data.db"
db.open(dbPath)

db.execute(dbPath, "CREATE TABLE IF NOT EXISTS users (id INTEGER, name TEXT)")
db.execute(dbPath, "INSERT INTO users VALUES (1, 'Alice')")

let users = db.query(dbPath, "SELECT * FROM users")
print(users)
```

## What's Next?

- Read the full **[Language Reference](Language)**
- Explore the **[Standard Library](Standard-Library)**
- Build an **[HTTP API with SQLite](HTTP-Server?id=sqlite-integration)**
- Set up the **[VS Code Debugger](VS-Code?id=debugger)**
