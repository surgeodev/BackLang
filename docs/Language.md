# Language Reference

## Syntax & Basics

BackLang uses a C-style syntax with curly braces, semicolons, and familiar keywords.

### Whitespace & Semicolons

Semicolons are optional. Statements are separated by newlines or semicolons.

```bl
let a = 1
let b = 2; let c = 3
```

### Keywords

```
let, const, if, else, while, for, in, break, continue,
function, return, true, false, null,
server, endpoint, import, middleware,
GET, POST, PUT, DELETE, PATCH
```

### Identifiers

Identifiers start with a letter or underscore, followed by letters, digits, or underscores.

```bl
myVar          // valid
_count         // valid
data2          // valid
2data          // invalid (starts with digit)
my-var         // invalid (hyphen not allowed)
```

## Types and Values

BackLang is **dynamically typed**. Variables can hold values of any type.

### `null`

The null value represents the absence of a value.

```bl
let x = null
```

### `bool`

Boolean values.

```bl
let a = true
let b = false
```

### `num`

All numbers are 64-bit floating point. Integer literals with no fractional part are displayed without decimals.

```bl
let a = 42           // integer
let b = 3.14         // float
let c = -10          // negative
let d = 1.5e10       // scientific notation
```

### `str`

Strings are enclosed in double or single quotes. Supports escape sequences.

```bl
let a = "hello"
let b = 'world'
let c = "line1\nline2"    // newline
let d = "tab\there"       // tab
let e = "quote: \"hi\""   // escaped quote
```

### `array`

Ordered, mutable lists. Indexed from 0.

```bl
let arr = [1, 2, 3]
print(arr[0])      // 1
print(len(arr))    // 3
push(arr, 4)
pop(arr)
```

### `object`

Key-value maps. Keys are strings.

```bl
let obj = {name: "Alice", age: 30}
print(obj.name)         // Alice
obj.age = 31            // mutate
print(obj["name"])      // Alice (bracket access)
```

### `function` (as a value)

Functions are first-class values.

```bl
function add(a, b) { a + b }
let fn = add
print(fn(3, 4))    // 7
```

## Variables and Constants

### `let` — Mutable

```bl
let x = 10
x = 20    // OK
```

### `const` — Immutable

```bl
const PI = 3.14159
PI = 3    // Error: cannot assign to immutable variable
```

### Scoping

Variables are lexically scoped. Inner scopes can access variables from enclosing scopes.

```bl
let x = 10
if true {
    let y = 20
    print(x + y)    // 30
}
// print(y)         // Error: y is not defined
```

## Operators

### Arithmetic

| Operator | Description |
|----------|-------------|
| `+` | Addition / concatenation |
| `-` | Subtraction |
| `*` | Multiplication |
| `/` | Division |
| `%` | Modulo |
| `+=` | Add and assign |
| `-=` | Subtract and assign |
| `*=` | Multiply and assign |
| `/=` | Divide and assign |

```bl
print(2 + 3)           // 5
print("a" + "b")       // "ab"
print([1] + [2, 3])    // [1, 2, 3]
print(10 % 3)          // 1
```

### Comparison

| Operator | Description |
|----------|-------------|
| `==` | Equal |
| `!=` | Not equal |
| `<` | Less than |
| `>` | Greater than |
| `<=` | Less than or equal |
| `>=` | Greater than or equal |

```bl
print(5 == 5)    // true
print(5 != 3)    // true
print(5 < 10)    // true
```

### Logical

| Operator | Description |
|----------|-------------|
| `&&` | And |
| `\|\|` | Or |
| `!` | Not |

```bl
if a > 0 && a < 10 { print("single digit") }
if !done { process() }
```

### Assignment

```bl
let x = 5
x = 10           // direct assignment
x += 3           // x = x + 3
x -= 2           // x = x - 2
x *= 4           // x = x * 4
x /= 2           // x = x / 2
```

### Member Access

```bl
let user = {name: "Alice", age: 30}
print(user.name)
print(user["name"])

user.age = 31
user["name"] = "Bob"
```

### Indexing

```bl
let arr = [10, 20, 30]
print(arr[0])      // 10
print(arr[2])      // 30
arr[1] = 25        // mutates array

let s = "hello"
print(s[0])        // "h"
print(s[4])        // "o"
```

## Control Flow

### `if` / `else`

```bl
if x > 0 {
    print("positive")
} else if x < 0 {
    print("negative")
} else {
    print("zero")
}
```

### `while`

```bl
let i = 0
while i < 5 {
    print(i)
    i = i + 1
}
```

### `for` — iteration

Iterates over arrays and strings.

```bl
// Over array
let items = ["a", "b", "c"]
for item in items {
    print(item)
}

// Over string
for ch in "hello" {
    print(ch)
}
```

### `break` / `continue`

```bl
for i in [1, 2, 3, 4, 5] {
    if i == 3 { break }       // exit loop
    if i == 2 { continue }    // skip to next
    print(i)
}
// prints: 1
```

## Functions

### Definition

```bl
function add(a: Num, b: Num) -> Num {
    a + b
}
```

- `a: Num` — parameter name with type annotation (annotations are optional and informational)
- `-> Num` — return type annotation (optional, informational)
- The last expression is the return value (implicit return)
- Explicit `return` also works

### With explicit return

```bl
function max(a: Num, b: Num) -> Num {
    if a > b {
        return a
    }
    b
}
```

### Calling

```bl
let result = add(3, 4)
print(result)    // 7
```

### No parameters

```bl
function greet() {
    print("Hello!")
}

greet()
```

### No return value

```bl
function log(msg: Str) {
    print("[log] " + msg)
}

log("server started")
// returns null
```

## Comments

```bl
// Single-line comment

/*
Multi-line
comment
*/
```

## Imports and Modules

### Standard Library Modules

```bl
import "std.os"
import "std.fs"
import "std.math"
import "std.string"
import "std.random"
import "std.db"
```

See the **[Standard Library](/#/Standard-Library)** for details.

### Local File Imports

```bl
import "myutils"          // loads myutils.bl
import "lib/helpers"      // loads lib/helpers.bl
```

### Installed Packages

```bl
import "package_name"     // loads ~/.backlang/packages/package_name/index.bl
```

## Truthiness

Values are considered `false` in boolean contexts if they are:

- `null`
- `false`
- `0` (zero)
- `""` (empty string)

Everything else is `true`:

- Non-zero numbers
- Non-empty strings
- Arrays (even empty)
- Objects (even empty)
- Functions

```bl
if null { print("no") }          // not printed
if 0 { print("no") }             // not printed
if 1 { print("yes") }            // printed
if [] { print("yes") }           // printed (non-null)
if {} { print("yes") }           // printed (non-null)
```

## Error Handling

Runtime errors produce an error message and exit with a non-zero code.

```bl
print(undefined_var)       // Error: undefined: undefined_var
print(1 / 0)               // Error: division by zero
db.query("bad.sql")        // Error: database not opened
```

There is currently no `try`/`catch` mechanism. (Planned for a future release.)
