pub enum Color {
    Red,
    Green,
    Blue,
}

pub impl Color {
    pub fn print(&self) {
        match self {
            Color::Red => println!("red"),
            Color::Green => println!("green"),
            Color::Blue => println!("blue")
        }
    }
}