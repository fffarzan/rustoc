use super::enums::{Direction, Color, Flavor, Access};
use super::structs::{GroceryItem, Drink};
use super::fns::{print_color, print_drink, coordinate, print_message};

pub fn run() {
    let go = Direction::Left;

    match go {
        Direction::Left => println!("go left"),
        Direction::Right => println!("go right"),
        Direction::Up => println!("go up"),
    }

    print_color(Color::Red);

    let cereal = GroceryItem {
        stock: 10,
        price: 2.99,
    };

    println!("stock: {:?}", cereal.stock);
    println!("price: {:?}", cereal.price);

    let drink = Drink {
        flavor: Flavor::Fruity,
        oz: 6.99,
    };

    print_drink(drink);

    let coord = (2, 3);
    println!("{:?}, {:?}", coord.0, coord.1);

    let (x, y) = (2, 3);
    println!("{:?}, {:?}", x, y);

    let (name, age) = ("Emma", 20);

    let (x, y) = coordinate();
    if y > 5 {
        println!(">5")
    } else if y < 5 {
        println!("<5")
    } else {
        println!("=5")
    }

    let access_level = Access::Guest;
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false
    };
    println!("{:?}", can_access_file);

    let val = 111;
    let is_gt_100 = val > 100;
    print_message(is_gt_100);
}