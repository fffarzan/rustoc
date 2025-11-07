use chrono::prelude::*;

pub fn run() {
    let local: DateTime<Local> = Local::now();
    let formated_date = local.format("%Y-%m-%d %H:%M:%S").to_string();
    
    println!("{:?}", formated_date);
}