#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

pub fn run() {
    let user_name = "Sam";
    let found_user = find_user(user_name)
                        .map(|user_id| {
                                User {
                                    name: user_name.to_owned(),
                                    user_id,
                                }
                            }
                        );
    
    match found_user {
        Some(found_user) => println!("{:?}", found_user),
        None => println!("user not found"),
    }

    let a: Option<i32> = Some(1);
    let b: Option<i32> = None;

    let a_is_some = a.is_some();
    let b_is_some = b.is_some();
    dbg!(a_is_some);
    dbg!(b_is_some);

    let a_is_none = a.is_none();
    let b_is_none = b.is_none();
    dbg!(a_is_none);
    dbg!(b_is_none);

    let a_mapped = a.map(|num| num + 1);
    let b_mapped = b.map(|num| num + 1);
    dbg!(a_mapped);
    dbg!(b_mapped);

    let a_filtered = a.filter(|num| num == &1);
    let b_filtered = b.filter(|num| num == &1);
    dbg!(a_filtered);
    dbg!(b_filtered);

    let a_or_else = a.or_else(|| Some(5));
    let b_or_else = b.or_else(|| Some(5));
    dbg!(a_or_else);
    dbg!(b_or_else);

    let a_unwrapped = a.unwrap_or_else(|| 0);
    let b_unwrapped = b.unwrap_or_else(|| 0);
    dbg!(a_unwrapped);
    dbg!(b_unwrapped);
}
