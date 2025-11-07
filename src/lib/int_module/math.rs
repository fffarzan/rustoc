mod math {
    pub fn add(lhs: isize, rhs: isize) -> isize {
        lhs + rhs
    }

    pub fn sub(lhs: isize, rhs: isize) -> isize {
        lhs - rhs
    }

    pub fn mul(lhs: isize, rhs: isize) -> isize {
        lhs * rhs
    }
}

pub fn sample() {
    math::add(1, 2);

    let result = {
        let two_plus_two = math::add(2, 2);
        let three = math::sub(two_plus_two, 1);
        math::mul(three, three)
    };

    assert_eq!(result, 9);
    println!("(2 + 2 -1) * 3 = {}", result);
}