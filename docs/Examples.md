# Examples

## Hello World

```bl
print("Hello, World!")
```

```bash
bl hello.bl
```

---

## FizzBuzz

```bl
for i in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] {
    if i % 15 == 0 {
        print("FizzBuzz")
    } else if i % 3 == 0 {
        print("Fizz")
    } else if i % 5 == 0 {
        print("Buzz")
    } else {
        print(i)
    }
}
```

---

## Factorial

```bl
function factorial(n: Num) -> Num {
    if n <= 1 {
        return 1
    }
    n * factorial(n - 1)
}

print(factorial(5))  // 120
print(factorial(10)) // 3628800
```

---

## Fibonacci

```bl
function fib(n: Num) -> Num {
    if n <= 1 { return n }
    fib(n - 1) + fib(n - 2)
}

for i in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10] {
    print("fib(" + i + ") = " + fib(i))
}
```

---

## JSON API Server

```bl
// api.bl — Simple JSON API
import "std.db"

let DB = "/tmp/store.db"
db.open(DB)
db.execute(DB, "CREATE TABLE IF NOT EXISTS products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT,
    price REAL
)")

server store { port: 8080; cors: true }

endpoint GET "/api/products" {
    return {status: 200, body: db.query(DB, "SELECT * FROM products")}
}

endpoint POST "/api/products" {
    db.execute(DB, "INSERT INTO products (name, price) VALUES ('" + req.body.name + "', " + req.body.price + ")")
    return {status: 201, body: {ok: true}}
}

endpoint DELETE "/api/products/:id" {
    db.execute(DB, "DELETE FROM products WHERE id = " + req.params.id)
    return {status: 200, body: {ok: true}}
}
```

Test it:

```bash
bl api.bl &
curl http://localhost:8080/api/products
curl -X POST http://localhost:8080/api/products \
    -H "Content-Type: application/json" \
    -d '{"name":"Widget","price":9.99}'
```

---

## File Concatenation

```bl
import "std.fs"

function cat(paths: Arr) {
    for p in paths {
        print("=== " + p + " ===")
        print(std.fs.readFile(p))
    }
}

cat(["file1.txt", "file2.txt"])
```

---

## Environment Variable Check

```bl
import "std.os"

let env = std.os.getenv("ENVIRONMENT")
if env == "production" {
    print("Running in production mode")
} else {
    print("Running in development mode")
}
```

---

## Random Dice Roller

```bl
import "std.random"

function roll(sides: Num) -> Num {
    std.random.randint(1, sides)
}

// Roll 2D6
let total = roll(6) + roll(6)
print("You rolled: " + total)
```

---

## String Utilities Demo

```bl
import "std.string"

let text = "  Hello, World!  "
print("Original: '" + text + "'")
print("Trimmed:  '" + std.string.trim(text) + "'")
print("Upper:    '" + std.string.toUpper(text) + "'")
print("Lower:    '" + std.string.toLower(text) + "'")

let parts = std.string.split("apple,banana,cherry", ",")
for p in parts {
    print(" - " + p)
}
```
