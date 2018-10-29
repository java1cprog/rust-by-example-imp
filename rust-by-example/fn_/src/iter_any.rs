// cargo run -p fn_ --bin iter_any

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` для векторов даёт `&i32`. Приводим к `i32`.
    println!("2 в vec1: {}", vec1.iter()     .any(|&_x| 1 / 3 == 0));
    println!("2 в vec1: {}", vec1.iter()     .any(|&x| x == 2));
    println!("2 в vec1: {}", vec1.iter()     .any(|&x| x == 2));

    // `into_iter()` для векторов даёт `i32`. Приведения не требуется.
    println!("2 в vec2: {}", vec2.into_iter().any(| x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` для массивов даёт `&i32`.
    println!("2 в array1: {}", array1.iter()     .any(|&x| x == 2));
    // `into_iter()` для массивов неожиданно даёт `&i32`.
    println!("2 в array2: {}", array2.into_iter().any(|&x| x == 2));

    let a = [1, 2, 3];

    let mut iter = a.iter();

    assert!(!iter.all(|&x| x != 2));

// we can still use `iter`, as there are more elements.
    assert_eq!(iter.next(), Some(&3));
}