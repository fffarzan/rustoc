struct Tempraure {
    degrees_f: f64,
}

impl Tempraure {
    fn freezing() -> Self {
        Self {
            degrees_f: 32.0
        }
    }

    fn boiling() -> Self {
        Self {
            degrees_f: 212.0
        }
    }

    fn show_temp(&self) {
        println!("{:?} degrees F", self.degrees_f);
    }
}