pub struct Dimensions {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
}

pub impl Dimensions {
    pub fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}