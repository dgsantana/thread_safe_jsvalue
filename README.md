# Thread Safe JS Value

## Description

This crate provides a thread safe wrapper around `JsValue` from `wasm-bindgen`. It allows you to send `JsValue` between threads and share it between threads.

It also works with other types that aren't `Send` or `Sync`.

This crates was inspired by [`SendWrapper`](https://github.com/thk1/send_wrapper), but is a little more turned to work with `JsValue` based values,
that are required to be `Send` and `Sync` in (`Leptos`)[https://leptos.dev] 0.7+ signals.

If the base javascript Rust wrapper includes Clone, PartialEq, PartialOrd, Hash, Debug, Display this wrapper will also include them. For wasm32 targets the wrapper will avoid the thread check, 
since it is not necessary, since there aren't any threads.

## Features

- macro to help create `From` implementations for your types
- `JsValue` wrapper that is `Send` and `Sync`
- Also works with other types that aren't `Send` or `Sync`

## Installation

Add the following line to your `Cargo.toml` file:

```toml
[dependencies]
thread_safe_jsvalue = "0.1.0"
```

## Usage

```rust
use thread_safe_jsvalue::ThreadSafeJsValue;

/// Example usage
fn good() {
    let value = ThreadSafeJsValue::new(42);
    let value2 = value.clone();
    let value 3 = if let Some(value) = value2.try_value() {
        let value = value * 2;
        value.into()
    } else {
        ThreadSafeJsValue::new(0)
    };
}


/// Bad Example usage
fn bad() {
    let value = ThreadSafeJsValue::new(42);
    let value2 = value.clone();
    let value3 = value.clone();

    let thread = std::thread::spawn(move || {
        let value = value2;
        let value = value.value();
        let value = value * 2;
        value.into()
    });
}

```

## Contributing

Contributions are welcome! For feature requests and bug reports please [submit an issue](github.com/dgsantana/thread_safe_jsvalue/issues).

## License

This project is licensed under the [MIT License](LICENSE).
