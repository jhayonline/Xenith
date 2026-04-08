# std::path - Path Manipulation

## Introduction

The `std::path` module provides platform-safe path manipulation functions. All operations handle the differences between Unix (/) and Windows (\) path separators automatically.

## Importing

```xenith
grab { 
    join, basename, dirname, extension, stem,
    is_absolute, is_relative, absolute, normalize, components, parent
} from "std::path"
```

## Functions

### `join(parts: list<string>) -> string`

Joins path components using the platform's native separator.

```xenith
spawn path: string = join(["/home", "user", "docs", "file.txt"])
echo(path)  # /home/user/docs/file.txt (Unix)
            # \home\user\docs\file.txt (Windows)
```

### `basename(path: string) -> string`

Returns the last component of a path (filename or final directory).

```xenith
spawn name: string = basename("/home/user/file.txt")
echo(name)  # file.txt

spawn dir_name: string = basename("/home/user/docs/")
echo(dir_name)  # docs
```

### `dirname(path: string) -> string`

Returns the parent directory portion of a path.

```xenith
spawn parent: string = dirname("/home/user/file.txt")
echo(parent)  # /home/user

spawn root: string = dirname("/home")
echo(root)  # /
```

### `extension(path: string) -> string`

Returns the file extension without the dot. For files with multiple dots, returns the last extension.

```xenith
spawn ext: string = extension("archive.tar.gz")
echo(ext)  # gz

spawn no_ext: string = extension("README")
echo(no_ext)  # (empty string)
```

### `stem(path: string) -> string`

Returns the filename without its extension.

```xenith
spawn name: string = stem("document.pdf")
echo(name)  # document

spawn name: string = stem("archive.tar.gz")
echo(name)  # archive.tar
```

### `is_absolute(path: string) -> bool`

Returns `true` if the path is absolute (starts from root).

```xenith
echo(is_absolute("/home/user"))     # true (Unix)
echo(is_absolute("C:\\Windows"))    # true (Windows)
echo(is_absolute("relative/path"))  # false
```

### `is_relative(path: string) -> bool`

Returns `true` if the path is relative (not absolute).

```xenith
echo(is_relative("/home/user"))     # false
echo(is_relative("docs/file.txt"))  # true
```

### `absolute(path: string) -> string`

Converts a relative path to an absolute path. Resolves symbolic links.

```xenith
spawn abs: string = absolute(".")
echo(abs)  # /home/user/project (full path)

spawn abs_file: string = absolute("docs/readme.txt")
echo(abs_file)  # /home/user/project/docs/readme.txt
```

**Note:** The path must exist, otherwise an error is thrown.

### `normalize(path: string) -> string`

Removes redundant separators, `.` (current directory), and resolves `..` (parent directory).

```xenith
spawn norm: string = normalize("/home/./user/../user/docs/./file.txt")
echo(norm)  # /home/user/docs/file.txt

spawn norm: string = normalize("docs/../files/./notes.txt")
echo(norm)  # files/notes.txt
```

### `components(path: string) -> list<string>`

Splits a path into its individual components.

```xenith
spawn parts: list<string> = components("/home/user/docs/file.txt")
for part in parts {
    echo(part)
}
# Output:
# /
# home
# user
# docs
# file.txt
```

### `parent(path: string) -> string`

Returns the parent directory. Same as `dirname()` but returns `.` for paths with no parent.

```xenith
spawn parent_dir: string = parent("/home/user/docs/file.txt")
echo(parent_dir)  # /home/user/docs

spawn current: string = parent("file.txt")
echo(current)  # .
```

## Complete Example

```xenith
grab {
    join, basename, dirname, extension, stem,
    is_absolute, normalize, components, parent
} from "std::path"

# Build a path
spawn file_path: string = join(["/var", "log", "app", "output.log"])
echo("Full path: {file_path}")

# Extract components
echo("Filename: {basename(file_path)}")
echo("Directory: {dirname(file_path)}")
echo("Extension: {extension(file_path)}")
echo("Name: {stem(file_path)}")

# Check path type
echo("Is absolute? {is_absolute(file_path)}")

# Normalize messy path
spawn messy: string = join(["/home", ".", "user", "..", "user", "docs"])
spawn clean: string = normalize(messy)
echo("Normalized: {clean}")

# Iterate through components
echo("Path components:")
for comp in components(file_path) {
    echo("  {comp}")
}

# Navigate to parent
spawn current: string = file_path
while parent(current) != current {
    echo("Parent: {current}")
    current = parent(current)
}
```

## Error Handling

`absolute()` can fail if the path doesn't exist:

```xenith
try {
    spawn abs: string = absolute("nonexistent/file.txt")
} catch err {
    echo("Failed to get absolute path: {err}")
}
```

## Platform Behavior

| Operation | Unix | Windows |
|-----------|------|---------|
| Separator | `/` | `\` |
| Root | `/` | `C:\` |
| Absolute | Starts with `/` | Starts with drive letter + `:\` |

## See Also

- `std::fs` - File system operations
- `std::process` - Working with current directory
```
