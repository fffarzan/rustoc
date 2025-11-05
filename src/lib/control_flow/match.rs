enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

pub fn sample() {
    let name = "Bob";
    match name {
        "Jayson" => println!("not my name"),
        "Bob" => println!("also not my name"),
        "Alice" => println!("hello Alice!"),
        _ => println!("nice to meet you!")
    }

    let n = 3;
    match n {
        3 => println!("three"),
        other => println!("number: {:?}", other),
    }
}

pub fn match_with_enum() {
    let flat = Discount::Flat(2);

    match flat {
        Discount::Flat(2) => println!("flat 2"),
        Discount::Flat(amount) => println!("flat discount of {:?}", amount),
        _ => (),
    }
}

pub fn match_with_struct() {
    let concert = Ticket {
        event: String::from("concert"),
        price: 50,
    };
    
    match concert {
        Ticket { price: 50, event } => println!("event @ 50 = {:?}", event),
        Ticket { price, ..} => println!("price = {:?}", price),
    }
}

pub fn ex1() {
    let is_true = false;

    match is_true {
        true => println!("it's true"),
        false => println!("it's false"),
    }
}

pub fn ex2() {
    let n = 6;

    match n {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other")
    }
}
