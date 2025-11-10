pub struct ShippingBox {
    pub dimensions: Dimensions,
    pub weight: f64,
    pub color: Color,
}

pub impl ShippingBox {
    pub fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    pub fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}