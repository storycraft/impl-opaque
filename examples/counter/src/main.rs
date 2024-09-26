use impl_opaque::opaque;

#[opaque(pub, initial: i32)]
impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        #[field]
        let ref mut counter: i32 = initial;
        let cur = *counter;
        *counter += 1;

        Some(cur)
    }
}

fn main() {
    for i in Counter::new(0).take(5) {
        println!("{i}");
    }
}
