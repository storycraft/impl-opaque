use impl_opaque::opaque;

trait Trait {
    fn run(&self);
}

#[opaque(pub, msg: &str)]
impl Trait for State {
    fn run(&self) {
        #[field]
        let ref msg: String = msg.to_string();

        println!("{}", msg);
    }
}

fn main() {
    let state = State::new("hello world");

    state.run();
}
