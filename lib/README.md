# `impl-opaque`
[Documentation](https://docs.rs/impl-opaque/latest)

Declare struct fields and initializers in implementation area.

This macro tries to solve separation of field declarations, initializations and usages.
The `opaque` attribute generates struct declaration, struct constructor (`new` method) by collecting field declarations and initializers inside impl block.

This crate is no_std on runtime and requires alloc to build macro.

## Features
1. Declare fields inside method
```rust no_run
#[opaque]
impl Struct {
    fn run(&mut self) {
        #[field]
        let ref mut count: i32 = 0;
        *count += 1;

        println!("{}", count);
    }
}
```

2. Declare fields inside impl block
```rust no_run
#[opaque]
impl Struct {
    field!(count: i32 = 0);

    fn run(&mut self) {
        self.count += 1;
        println!("{}", self.count);
    }
}
```

3. Convert constructor arguments into fields
```rust no_run
#[opaque(pub(self) count: i32)]
impl Struct {
    fn run(&mut self) {
        self.count += 1;
        println!("{}", self.count);
    }
}
```

4. Declare struct and implement trait at once
```rust no_run
#[opaque]
impl Iterator for Struct {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        #[field]
        let ref mut count: i32 = 0;
        *count += 1;

        Some(*count)
    }
}
```

5. Pattern matching and early return for fields inside a method
```rust no_run
#[opaque]
impl Struct {
    pub fn run(&mut self) {
        #[field]
        let ref mut running @ true: bool = true else {
            return
        };

        println!("run");
        *running = false;
    }
}
```

Attribute reference
```rust no_run
#[opaque($(as $vis $(const)? ,)? $($($vis)? $ident: $ty),*)]
```

## Examples
See `examples` for simple example

## License
This crate is licensed under MIT OR Apache-2.0
