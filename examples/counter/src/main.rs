use impl_opaque::opaque;

// use constructor arg as field
#[opaque(pub, pub(self) counter: i32)]
impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;

        Some(self.counter)
    }
}

fn main() {
    for i in Counter::new(0).take(5) {
        println!("{i}");
    }
}
