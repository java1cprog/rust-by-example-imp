// cargo run -p scope --bin drop

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    let _x = ToDrop;
    println!("Made a ToDrop!");
}