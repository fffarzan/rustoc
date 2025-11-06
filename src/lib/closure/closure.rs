pub fn sample() {
    let add = |a: i32, b: i32| -> i32 {
        a + b
    };
    let add = |a, b| a + b;

    let sum = add(1, 1);

    println!("sum: {:?}", sum);
}