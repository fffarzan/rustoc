mod greet {
    pub fn hello() {
        println!("hello");
    }

    pub fn goodbye() {
        println!("goodbye");
    }
}

pub fn sample() {
    use greet::*;
    greet::hello();
}