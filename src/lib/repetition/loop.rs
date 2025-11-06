pub fn sample() {
    let mut i = 3;

    loop {
        println!("{:?}", i);
        i = i - 1;

        if i == 0 {
            break;
        }
    }

    println!("done");
}

pub fn ex() {
    let mut n = 1;

    loop {
        println!("{:?}", n);

        if n == 4 {
            break;
        }

        n = n + 1;
    }

    println!("done");
}