// cargo run -p enums --bin conditional-enum-variants

pub trait Bool {
    const VALUE: bool;
}

pub struct True;

impl Bool for True {
    const VALUE: bool = true;
}

pub enum False {}

impl Bool for False {
    const VALUE: bool = false;
}

pub enum MyEnum<AllowB: Bool> {
    A,
    B(AllowB), // TODO: disallow on MyEnum<False>
}

impl<AllowB: Bool> MyEnum<AllowB> {
    pub fn is_a(&self) -> bool {
        match *self {
            MyEnum::A => true,
            _ => false,
        }
    }
}

impl MyEnum<True> {
    // this won't be allowed on MyEnum<False>, giving a helpful compile-time error
    pub fn is_b(&self) -> bool {
        match *self {
            MyEnum::B(_) => true,
            _ => false,
        }
    }
}

fn main() {
    let _e: MyEnum<True> = MyEnum::B(True);
}