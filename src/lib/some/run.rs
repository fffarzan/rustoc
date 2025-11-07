enum Color {
    Red,
    Green,
    Blue,
}

pub fn run() {
    let maybe_user = Some("Tom");

    match maybe_user {
        Some(user) => println!("user={:?}", user),
        None => println!("no user"),
    };

    super::if_let::sample();

    // // todo: should read doc
    // let red = Color::Red;
    // if let Color::Red = red {
    //     println!("red");
    // } else {
    //     println!("not red");
    // }

    // // todo: should read doc
    // let mut data = Some(3);
    // while let Some(i) = data {
    //     println!("loop");
    //     data = None
    // }
    // println!("done");

    // // todo: should read doc
    // let x_numbers = vec![1, 2, 3];
    // let mut nums_iter = x_numbers.iter();
    // while let Some(num) = nums_iter.next() {
    //     println!("num={:?}", num);
    // }
    // println!("done");
}