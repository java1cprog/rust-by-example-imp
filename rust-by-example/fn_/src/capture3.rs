// cargo run -p fn_ --bin capture3

fn do_twice<F>(mut func: F)
    where F: FnMut()
{
    for _i in 0..=100 {
        func();
    }
//    func();
//    func();
}
fn main() {


    let mut x: usize = 1;
    {
        let add_two_to_x = || x += 2;
        do_twice(add_two_to_x);
       // do_twice(add_two_to_x);
    }
let mut x1=1;
    for _i in 0..=100 {
        x1 += 2;
    }
    assert_eq!(x1, x);


    let mut x = 5;
    {
        let mut square_x = || x *= x;
        square_x();
        square_x();
    }
  //  assert_eq!(x, 25);

}
