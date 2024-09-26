# `impl-opaque`
[Documentation](https://docs.rs/retained/latest)

Declare complex opaque struct using impl block

This crate is no_std.

## Usage
```rust no_run
trait Trait {
    fn run(&self);
}

// Using trait impl block
#[opaque(pub /* optional struct vis */ , msg: &str /* constructor */ )]
impl Trait for TraitImpl {
    fn run(&self) {
        #[field]
        let ref msg: String = msg.to_string();

        println!("{}", msg);
    }
}

// Using struct impl block
#[opaque(msg: &str)]
impl Impl {
    fn run(&self) {
        #[field]
        let ref msg: String = msg.to_string();

        println!("{}", msg);
    }
}

fn main() {
    let trait_impl = TraitImpl::new("hello world");
    let imp = Impl::new("hello world");
}
```

## Examples
See `examples` for simple example

## License
This crate is licensed under MIT OR Apache-2.0
