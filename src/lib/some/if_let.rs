pub fn sample() {
    let maybe_user = Some("Jerry");

    if let Some(user) = maybe_user {
        println!("user={:?}", user);
    } else {
        println!("no user");
    }
}