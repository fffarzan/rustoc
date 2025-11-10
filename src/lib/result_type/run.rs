#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mm" => Ok(MenuChoice::MainMenu),
        "s" => Ok(MenuChoice::Start),
        "q" => Ok(MenuChoice::Quit),
        _ => Err("menu choice not found".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("choice = {:?}", choice);
}

fn pick_choice(input: &str) -> Result<(), String> {
    let choice: MenuChoice = get_choice(input)?;
    print_choice(&choice);

    Ok(())
}

struct Customer {
    age: i32,
}

fn purchase(customer: &Customer) -> Result<(), String> {
    if customer.age < 21 {
        Err("Illegal".to_owned())
    } else {
        Ok(())
    }    
}

enum EmployeePosition {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnician,
}

enum EmployeeStatus {
    Active,
    Terminated,
}

struct Employee {
    position: EmployeePosition,
    status: EmployeeStatus,
}

fn try_access(emp: &Employee) -> Result<(), String> {
    match emp.status {
        EmployeeStatus::Terminated => return Err("terminated".to_owned()),
        _ => (),
    }

    match emp.position {
        EmployeePosition::Maintenance =>  Ok(()),
        EmployeePosition::Marketing => Ok(()),
        EmployeePosition::Manager => Ok(()),
        _ => Err("invalid position".to_owned()),
    }
}

fn print_access(emp: &Employee) -> Result<(), String> {
    try_access(emp)?;
    println!("ok access");
    Ok(())
}

pub fn run() {
    pick_choice("mm");

    let choice = pick_choice("e");
    println!("choice value {:?}", choice);

    let mike = Customer {
        age: 21,
    };
    let purchased = purchase(&mike);
    println!("{:?}", purchased);

    let manager = Employee {
        position: EmployeePosition::Manager,
        status: EmployeeStatus::Terminated,
    };
    match print_access(&manager) {
        Err(e) => println!("access denied: {:?}", e),
        _ => (),
    }
}
