// todo: read more about this
#[derive(Debug, Clone, Copy)]
enum Position {
    Manager, 
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i64,
}

fn print_employee(emp: Employee) {
    println!("{:?}", emp);
}

pub fn run() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 60,
    };
    
    print_employee(me);
}