# rust DotEnv - A Lightweight Rust Library for Loading Environment Variables

`rust_dotenv` is a Rust library designed to load environment variables from `.env` files, supporting multiple environments and local overrides. This library is perfect for managing configurations dynamically across different environments like development, testing, and production.

> **Note**: I'm French dev begginer in Rust, and this is my first Rust project! Feel free to open issues or submit pull requests to suggest improvements and new features. Contributions are more than welcome!

## Features

- **Environment-Specific Files**: Loads variables from the default `.env` file or an environment-specific file like `.env.development`.
- **Automatic Local Overrides**: By default, `rust_dotenv` checks for a `.local` file, such as `.env.development.local` or `.env.local`. If this file exists, it will be prioritized over the main `.env` file, allowing for easy local configuration without modifying environment files directly.
- **Convenient API**: Provides methods to retrieve (`get_var`), check existence (`has_var`), and set new environment variables (`set_var`) without overwriting existing ones.

## Installation

Add `rust_dotenv` to your `Cargo.toml` dependencies:

```toml
[dependencies]
rust_dotenv = "0.1.0"
```

## Usage Example

### 1. Initializing `rust_dotenv` with an Environment

Use `DotEnv::new()` to load variables from a specified environment. Pass the environment name as a string (like `"development"`), or an empty string `""` to load the default `.env` file.

```rust
use rust_dotenv::DotEnv;

fn main() {
    // Load the default `.env` file
    let dotenv = DotEnv::new("");
    println!("Loaded variables (default): {:?}", dotenv.all_vars());

    // Load environment-specific file, e.g., `.env.development`
    let dotenv_dev = DotEnv::new("development");
    println!("Loaded variables (development): {:?}", dotenv_dev.all_vars());
}
```

### 2. Retrieving an Environment Variable with `get_var`

Retrieve a specific environment variable using `get_var`.

```rust
fn main() {
    let dotenv = DotEnv::new("development");

    if let Some(value) = dotenv.get_var("DATABASE_URL".to_string()) {
        println!("DATABASE_URL = {}", value);
    } else {
        println!("DATABASE_URL is not set.");
    }
}
```

### 3. Checking for a Variable with `has_var`

Check if a variable exists with `has_var`.

```rust
fn main() {
    let dotenv = DotEnv::new("development");

    if dotenv.has_var("SECRET_KEY".to_string()) {
        println!("SECRET_KEY is set.");
    } else {
        println!("SECRET_KEY is not set.");
    }
}
```

### 4. Setting a New Variable with `set_var`

Add a new variable dynamically without overwriting existing ones. `set_var` returns `false` if the variable already exists.

```rust
fn main() {
    let mut dotenv = DotEnv::new("development");

    let added = dotenv.set_var("NEW_VARIABLE".to_string(), "value".to_string());
    if added {
        println!("NEW_VARIABLE added successfully.");
    } else {
        println!("NEW_VARIABLE already exists.");
    }
}
```

## Core API Overview

- **`DotEnv::new(env: &str) -> DotEnv`**: Loads variables from `.env`, `.env.{env}`, or `.env.{env}.local`, depending on the environment. If a `.local` file exists, it will take priority.
- **`get_var(&self, key: String) -> Option<String>`**: Retrieves the value of a variable.
- **`has_var(&self, key: String) -> bool`**: Checks if a variable is defined in the loaded configuration.
- **`all_vars(&self) -> HashMap<String, String>`**: Returns all environment variables as a `HashMap`.
- **`set_var(&mut self, key: String, value: String) -> bool`**: Adds a new variable only if it doesn’t already exist.

## Contributing

This project is open to contributions and suggestions! Since it’s my first Rust project, any feedback or feature requests are appreciated. Don’t hesitate to open an issue or pull request to help improve `rust_dotenv`.

1. Fork the repo
2. Create a new branch (`git checkout -b feature-branch`)
3. Commit your changes (`git commit -am 'Add new feature'`)
4. Push to the branch (`git push origin feature-branch`)
5. Create a pull request

---

This README introduces `rust_dotenv`'s capabilities, clarifies its default behavior with `.local` files, and invites the community to contribute. Let me know if any further adjustments are needed!
