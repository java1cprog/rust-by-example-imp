// cargo run -p scope --bin lifetime_methods

struct Owner(i32);

impl Owner {
    // Annotate lifetimes as in a standalone function.
    fn add_one<'a>(&'a mut self) { self.0 += 1; }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
    fn add_one1(&mut self) { self.0 += 1; }
    fn print1(&self) {
        println!("`print`: {}", self.0);
    }
}

fn main() {
    let mut owner  = Owner(18);

    owner.add_one();
    owner.print();
    owner.add_one1();
    owner.print1();
}
