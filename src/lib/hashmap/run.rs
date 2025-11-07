use std::collections::HashMap;

#[derive(Debug )]
struct Contents {
    content: String,
}

fn sample() {
    // inserting
    let mut lockers = HashMap::new();
    lockers.insert(1, Contents { content: "stuff".to_owned() });
    lockers.insert(2, Contents { content: "shirt".to_owned() });
    lockers.insert(3, Contents { content: "shorts".to_owned() }); 

    // iterating
    for (num, content) in lockers.iter() {
        println!("locker number: {:?}, content: {:?}", num, content);
    }
}

fn ex() {
    let mut stock = HashMap::new();
    stock.insert("chair", 5);
    stock.insert("bed", 3);
    stock.insert("table", 2);
    stock.insert("couch", 0);

    let mut total_stock = 0;
    for (name, quantity) in stock.iter() {
        total_stock = total_stock + quantity;

        let count = if quantity == &0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", quantity)
        };

        println!("name: {:?}, count: {:?}", name, count);
    }
    
    println!("total stock {:?}", total_stock);
}

pub fn run() {
    sample();
    ex();
}