const LEGAL_AGE: i32 = 18;

pub fn sample() {
    let current_age = 21;

    if current_age > LEGAL_AGE {
        println!("You are an adult");
    } else {
        println!("You are not an adult");
    }
}

pub fn ex1() {
    let the_bool = false;

    if the_bool == true {
        println!("Hello");
    } else {
        println!("Goodbye");
    }
}

pub fn ex2() {
    let n = 7;

    if n > 5 {
        println!(">5")
    } else if n < 5 {
        println!("<5")
    } else {
        println!("=5")
    }
}