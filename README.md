# Thread Safe JS Value

## Description

This crate provides a thread safe wrapper around `JsValue` from `wasm-bindgen`. It allows you to send `JsValue` between threads and share it between threads.

It also works with other types that aren't `Send` or `Sync`.

## Features

- macro to help create `From` implementations for your types
- `JsValue` wrapper that is `Send` and `Sync`
- Also works with other types that aren't `Send` or `Sync`

## Installation

Add the following line to your `Cargo.toml` file:

```toml
[dependencies]
thread_safe_js_value = "0.1.0"
```

## Usage

```rust
use thread_safe_js_value::ThreadSafeJsValue;

// Example usage
let value = ThreadSafeJsValue::from(42);
let value2 = value.clone();
let value 3 = if let Some(value) = value2.try_into_inner() {
    let value = value * 2;
    value.into()
} else {
    ThreadSafeJsValue::from(0)
}

// Bad Example usage
let value = ThreadSafeJsValue::from(42);
let value2 = value.clone();
let value3 = value.clone();

let thread = std::thread::spawn(move || {
    let value = value2;
    let value = value.into_inner();
    let value = value * 2;
    value.into()
});

```

## Contributing

Contributions are welcome! For feature requests and bug reports please [submit an issue](github.com/dgsantana/thread_safe_jsvalue/issues).

## License

This project is licensed under the [MIT License](LICENSE).