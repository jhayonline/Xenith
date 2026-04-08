# std::json - JSON Processing

## Introduction

The `std::json` module provides JSON parsing and serialization. It converts between Xenith maps and JSON format, with support for nested objects and arrays.

## Importing

```xenith
grab { parse, stringify, stringify_pretty } from "std::json"
```

## Functions

### `parse(input: any) -> json`

Converts a Xenith map or JSON string into a JSON value.

**With a map literal (recommended):**

```xenith
spawn user: json = parse({
    "name": "Alice",
    "age": 25,
    "active": true
})
```

**With a JSON string:**

```xenith
spawn user: json = parse("{\"name\":\"Alice\",\"age\":25,\"active\":true}")
```

### `stringify(value: json) -> string`

Converts a JSON value to a compact JSON string.

```xenith
spawn user: json = parse({
    "name": "Alice",
    "age": 25
})
spawn json_str: string = stringify(user)
echo(json_str)  # {"age":25,"name":"Alice"}
```

### `stringify_pretty(value: json) -> string`

Converts a JSON value to a formatted JSON string with indentation.

```xenith
spawn user: json = parse({
    "name": "Alice",
    "age": 25
})
spawn pretty: string = stringify_pretty(user)
echo(pretty)
# Output:
# {
#   "age": 25,
#   "name": "Alice"
# }
```

## Complete Examples

### Basic Object

```xenith
grab { parse, stringify } from "std::json"

spawn person: json = parse({
    "name": "Alice",
    "age": 25,
    "email": "alice@example.com"
})

spawn json_str: string = stringify(person)
echo(json_str)
```

### Nested Object

```xenith
grab { parse, stringify_pretty } from "std::json"

spawn config: json = parse({
    "server": {
        "host": "localhost",
        "port": 8080
    },
    "database": {
        "name": "myapp",
        "user": "admin"
    }
})

echo(stringify_pretty(config))
# Output:
# {
#   "database": {
#     "name": "myapp",
#     "user": "admin"
#   },
#   "server": {
#     "host": "localhost",
#     "port": 8080
#   }
# }
```

### Arrays

```xenith
grab { parse, stringify } from "std::json"

spawn data: json = parse({
    "fruits": ["apple", "banana", "orange"],
    "scores": [95, 87, 92]
})

spawn json_str: string = stringify(data)
echo(json_str)
# {"fruits":["apple","banana","orange"],"scores":[95,87,92]}
```

### API Response Handling

```xenith
grab { parse, stringify_pretty } from "std::json"

# Simulating an API response
spawn api_response: json = parse({
    "status": "success",
    "data": {
        "user": {
            "id": 1001,
            "name": "Alice",
            "email": "alice@example.com"
        },
        "token": "abc123xyz"
    }
})

# Pretty print for debugging
echo(stringify_pretty(api_response))
```

### Configuration File

```xenith
grab { parse, stringify_pretty } from "std::json"
grab { read, write } from "std::fs"

# Load configuration
try {
    spawn config_content: string = read("config.json")
    spawn config: json = parse(config_content)
    echo("Config loaded successfully")
} catch err {
    # Create default config
    spawn config: json = parse({
        "debug": true,
        "port": 3000,
        "database": {
            "host": "localhost",
            "port": 5432
        }
    })
    echo("Created default config")
}

# Save configuration
spawn config_str: string = stringify_pretty(config)
write("config.json", config_str)
```

### Round Trip Example

```xenith
grab { parse, stringify } from "std::json"

# Original data
spawn original: json = parse({
    "name": "Bob",
    "age": 30,
    "hobbies": ["reading", "coding", "gaming"]
})

# Convert to string and back
spawn json_str: string = stringify(original)
spawn restored: json = parse(json_str)

echo("Round trip successful!")
```

### Error Handling

```xenith
grab { parse } from "std::json"

# Invalid JSON will throw an error
try {
    spawn invalid: json = parse("{\"name\": \"Alice\"")
    echo("This won't print")
} catch err {
    echo("Invalid JSON: {err}")
}
```

## Type Conversion

When parsing JSON, values are converted to Xenith types:

| JSON Type | Xenith Type |
|-----------|-------------|
| `null` | `null` |
| `boolean` | `bool` |
| `number` | `float` |
| `string` | `string` |
| `array` | `json` (with list behavior) |
| `object` | `json` (with map behavior) |

## Performance Notes

- `parse()` with map literals is immediate (no parsing overhead)
- `parse()` with strings uses Rust's serde_json (very fast)
- `stringify()` and `stringify_pretty()` are also fast

## See Also

- `std::fs` - For reading/writing JSON files
- `std::http` - For fetching JSON from APIs
```
