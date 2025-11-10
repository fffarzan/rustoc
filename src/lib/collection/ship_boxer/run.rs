pub fn run() {
    let small_dimensions = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 0.5,
    };
    let small_box = ShippingBox::new(2.5, Color::Red, small_dimensions);
    
    small_box.print();
}