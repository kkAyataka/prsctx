# prsctx

This library provides the context struct and the context stack for Rust.
We can use the context stack for debugging or logging.

This library has two parts.

* prsctx: Core library. It provides structs, functions, and the macro.
* prsctx-attr: It provides the attribute macro.

## Examples

A context has a context name, a file name, etc. If you create a Mark struct object, data is stacked. You can access the stacked data by using functions.

Stacked data is stacked on thread-local variables. So data is stacked per thread.

```rust
#[prsctx_attr::mark]
fn fn1() {
    prsctx::print_context_stack_chaind_string(); // >main>fn1
                                                 // >main>b1>fn2>fn1
}

#[prsctx_attr::mark]
fn fn2() {
    prsctx::print_context_stack_chaind_string(); // >main>b1>fn2
    fn1();
}

fn main() {
    prsctx::mark!("main");
    prsctx::print_context_stack_chaind_string(); // >main
    fn1();

    {
        prsctx::mark!("b1");
        prsctx::print_context_stack_chaind_string(); // >main>b1

        fn2();
    }

    prsctx::print_context_stack_chaind_string(); // >main
}
```

## License

[Boost Software License](./LICENSE)
