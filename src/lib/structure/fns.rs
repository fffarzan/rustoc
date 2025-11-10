pub fn print_color(color: super::enums::Color) {
    match color {
        super::enums::Color::Red => println!("red"),
        super::enums::Color::Yellow => println!("yellow"),
        super::enums::Color::Pink => println!("pink"),
        super::enums::Color::Blue => println!("blue"),
    }
}

pub fn print_drink(drink: super::structs::Drink) {
    match drink.flavor {
        super::enums::Flavor::Fruity => println!("the flavor is fruity"),
        super::enums::Flavor::Sweet => println!("the flavor is sweet"),
    }
    
    println!("the ounce is {:?}", drink.oz)
}

pub fn coordinate() -> (i32, i32) {
    (5, 7)
}

pub fn print_message(gt_100: bool) {
    match gt_100 {
        true => println!("it's big"),
        false => println!("it's small")
    }
}