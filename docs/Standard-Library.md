# Standard Library

The standard library provides built-in modules for common tasks. Import them with the `import` keyword.

## `std.db` — SQLite

Provides SQLite3 database operations. Each database is identified by its file path.

```bl
import "std.db"
```

### `db.open(path: Str) -> Str`

Opens (or creates) a SQLite database at the given path. Returns the path.

```bl
let db = db.open("/tmp/mydb.db")
// db = "/tmp/mydb.db"
```

### `db.query(path: Str, sql: Str) -> Array`

Executes a SELECT query and returns an array of row objects. Each object's keys are column names.

```bl
let rows = db.query("/tmp/mydb.db", "SELECT * FROM users")
for row in rows {
    print(row.name)
}
```

### `db.execute(path: Str, sql: Str) -> Num`

Executes an INSERT, UPDATE, or DELETE statement. Returns the number of affected rows.

```bl
let affected = db.execute("/tmp/mydb.db",
    "INSERT INTO users VALUES (1, 'Alice')")
print(affected)  // 1
```

### Alias Forms

All three functions are available with multiple naming conventions:

| Standard | Alias 1 | Alias 2 |
|----------|---------|---------|
| `db.open()` | `db_open()` | `std.db.open()` |
| `db.query()` | `db_query()` | `std.db.query()` |
| `db.execute()` | `db_execute()` | `std.db.execute()` |

### Complete Example

```bl
import "std.db"

let dbPath = "/tmp/app.db"
db.open(dbPath)

db.execute(dbPath, "CREATE TABLE IF NOT EXISTS items (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    price REAL
)")

db.execute(dbPath, "INSERT INTO items VALUES (1, 'Widget', 9.99)")
db.execute(dbPath, "INSERT INTO items VALUES (2, 'Gadget', 24.99)")

let items = db.query(dbPath, "SELECT * FROM items")
print(items)
// [{id: 1, name: "Widget", price: 9.99}, {id: 2, name: "Gadget", price: 24.99}]
```

## `std.os` — Operating System

```bl
import "std.os"
```

### `std.os.getenv(var: Str) -> Str | Null`

Returns the value of an environment variable, or `null` if not set.

```bl
let home = std.os.getenv("HOME")
print(home)   // "/home/user"
```

### `std.os.exit(code: Num)`

Terminates the process with the given exit code.

```bl
if error { std.os.exit(1) }
```

### `std.os.args() -> Array`

Returns the command-line arguments as an array of strings.

```bl
let args = std.os.args()
print(args[0])   // program name
```

## `std.fs` — File System

```bl
import "std.fs"
```

### `std.fs.readFile(path: Str) -> Str`

Reads an entire file into a string.

```bl
let content = std.fs.readFile("data.txt")
print(content)
```

### `std.fs.writeFile(path: Str, content: Str) -> Null`

Writes a string to a file, overwriting if it exists.

```bl
std.fs.writeFile("output.txt", "Hello, World!")
```

## `std.math` — Mathematics

```bl
import "std.math"
```

All functions take a single `Num` argument and return a `Num`.

```bl
let n = 25.0

print(std.math.sqrt(n))      // 5
print(std.math.abs(-10))     // 10
print(std.math.floor(3.7))   // 3
print(std.math.ceil(3.2))    // 4
```

| Function | Description |
|----------|-------------|
| `std.math.sqrt(x)` | Square root |
| `std.math.abs(x)` | Absolute value |
| `std.math.floor(x)` | Round down |
| `std.math.ceil(x)` | Round up |

## `std.string` — String Utilities

```bl
import "std.string"
```

### `std.string.split(s: Str, sep: Str) -> Array`

Splits a string by a separator into an array of substrings.

```bl
let parts = std.string.split("a,b,c", ",")
print(parts)   // ["a", "b", "c"]
```

### `std.string.trim(s: Str) -> Str`

Removes leading and trailing whitespace.

```bl
let clean = std.string.trim("  hello  ")
print(clean)   // "hello"
```

### `std.string.toUpper(s: Str) -> Str`

Converts to uppercase.

```bl
print(std.string.toUpper("hello"))   // "HELLO"
```

### `std.string.toLower(s: Str) -> Str`

Converts to lowercase.

```bl
print(std.string.toLower("WORLD"))   // "world"
```

## `std.random` — Random Numbers

```bl
import "std.random"
```

### `std.random.rand() -> Num`

Returns a random floating-point number between 0.0 and 1.0.

```bl
let r = std.random.rand()
print(r)   // e.g. 0.734528
```

### `std.random.randint(min: Num, max: Num) -> Num`

Returns a random integer between `min` and `max` (inclusive).

```bl
let roll = std.random.randint(1, 6)
print(roll)   // e.g. 4
```

## Built-in Functions

These functions are always available without importing anything.

### `print(val)`

Prints a value to stdout, followed by a newline. If called with no arguments, prints an empty line.

```bl
print("Hello")      // Hello
print(42)           // 42
print()             // (empty line)
```

### `len(val) -> Num`

Returns the length of a string or the number of elements in an array.

```bl
print(len("hello"))    // 5
print(len([1, 2, 3]))  // 3
```

### `push(arr, val) -> Array`

Appends a value to an array. The array is modified in place and returned.

```bl
let items = [1, 2]
push(items, 3)
print(items)   // [1, 2, 3]
```

### `pop(arr) -> Value`

Removes and returns the last element of an array.

```bl
let items = [1, 2, 3]
let last = pop(items)
print(last)    // 3
print(items)   // [1, 2]
```

### `keys(obj) -> Array`

Returns an array of the object's property names (as strings).

```bl
let user = {name: "Alice", age: 30}
print(keys(user))   // ["name", "age"]
```

### `str(val) -> Str`

Converts any value to its string representation.

```bl
print(str(42))          // "42"
print(str(true))        // "true"
print(str([1, 2, 3]))   // "[1, 2, 3]"
```

### `num(val) -> Num`

Converts a string to a number. Returns `NaN` for invalid input.

```bl
print(num("42"))      // 42
print(num("3.14"))    // 3.14
// num("abc")         → error
```

### `type(val) -> Str`

Returns the type name of a value as a string.

```bl
print(type(null))      // "null"
print(type(true))      // "bool"
print(type(42))        // "num"
print(type("hi"))      // "str"
print(type([1, 2]))    // "array"
print(type({}))        // "object"
print(type(print))     // "function"
```
