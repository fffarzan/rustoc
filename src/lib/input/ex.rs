use std::io;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    fn new(state: &str) -> Option<PowerState> {
        let state = state.trim().to_lowercase();

        match state.as_str() {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None,
        }
    }
    
}

fn print_power_status(state: PowerState) {
    use PowerState::*;
    match state {
        Off => println!("turning off.."),
        Sleep => println!("sleeping.."),
        Reboot => println!("rebooting.."),
        Shutdown => println!("shtting down.."),
        Hibernate => println!("hibernating.."),
    }
}

pub fn run() {
    let mut buffer = String::new();
    let input = io::stdin().read_line(&mut buffer);

    if input.is_ok() {
        let status = PowerState::new(&buffer);

        match status {
            Some(status) => print_power_status(status),
            None => println!("invalid power state!"),
        }
    } else {
        println!("error reading input")
    } 
}