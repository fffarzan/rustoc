struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>
}

fn sample() {
    let response = Survey {
        q1: Some(12),
        q2: None,
        q3: Some("A".to_owned()),
    };

    match response.q1 {
        Some(ans) => println!("q1: {:?}", ans),
        None => println!("q1: no response"),
    }
    match response.q2 {
        Some(ans) => println!("q2: {:?}", ans),
        None => println!("q2: no response"),
    }
}

struct Student {
    name: String,
    locker_id: Option<i32>
}

fn ex() {
    let mark = Student {
        name: "Mark".to_owned(),
        locker_id: Some(5),
    };
    let john = Student {
        name: "John".to_owned(),
        locker_id: None
    };

    println!("student: {:?}", john.name);
    match john.locker_id {
        Some(id) => println!("the locker id is: {:?}", id),
        None => println!("No locker is assigned!")
    }
}

pub fn run() {
    sample();
    ex();
}