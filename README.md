# A macro to make thread_local static wasm-bindgen closures easier to make.

Without this macro:

```rust
thread_local! {
    static CLOSURE_NAME: Closure<dyn FnMut()> = Closure::wrap(Box::new(|| {
        some_code();
    }) as Box<dyn FnMut()>);

    static CLOSURE_NAME_2: Closure<dyn FnMut(KeyboardEvent)> = Closure::wrap(Box::new(|event: KeyboardEvent| {
        if event.key_code() == 13 {
            some_code();
        }
    }) as Box<dyn FnMut(KeyboardEvent)>);
}
```

With this macro:

```rust
static_closures! {
    CLOSURE_NAME = || {
        some_code();
    };

    CLOSURE_NAME_2 = |event: KeyboardEvent| {
        if event.key_code() == 13 {
            some_code();
        }
    };
}
```
