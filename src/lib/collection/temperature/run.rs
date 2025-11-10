pub fn run() {
    let hot = Tempraure {
        degrees_f: 99.6,
    };
    hot.show_temp();

    let cold = Tempraure::freezing();
    cold.show_temp();
    cold.show_temp();
    cold.show_temp();

    let boiling = Tempraure::boiling();
    boiling.show_temp();
}