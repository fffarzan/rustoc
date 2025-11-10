struct LineItem {
    name: String,
    count: i32,
}

fn print_name(name: &str) {
    println!("name: {:?}", name);
}

struct Person {
    age: i32,
    name: String,
    fav_color: String,
}

fn just_print(data: &str) {
    println!("{:?}", data);
}

pub fn run() {
    let all_numbers = vec![10, 20, 30, 40];
    for num in &all_numbers {
        match num {
            30 => println!("thirty"),
            _ => println!("{:?}", num)
        }
    }
    println!("length: {:?}", all_numbers.len());
    
    let receipt = vec![
        LineItem {
            name: "cereal".to_owned(),
            count: 1,
        },
         LineItem {
            name: String::from("fruit"),
            count: 5
         },
    ];
    for item in receipt {
        print_name(&item.name);
        println!("count = {:?}", item.count);
    }

    let people = vec![
        Person {
            name: String::from("John"),
            age: 21,
            fav_color: String::from("Red"),
        },
        Person {
            name: String::from("Jane"),
            age: 23,
            fav_color: String::from("Blue"),
        },
        Person {
            name: String::from("Michael"),
            age: 9,
            fav_color: String::from("Yellow"),
        },
    ];
    for person in people {
        if person.age <= 10 {
            just_print(&person.name);
            just_print(&person.fav_color);
        }
    }
}