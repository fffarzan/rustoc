pub fn sample() {
    let range = 1..=3;
    for num in range {
        println!("{:?}", num);
    }

    let another_range = 1..4;
    for num in another_range {
        println!("{:?}", num);
    }

    for ch in 'a'..='f' {
        println!("{:?}", ch);
    }
}