use std::io;

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    
    Ok(buffer.trim().to_owned())
}

pub fn run() {
    let mut inputs = vec![];
    let mut times_input = 0;

    while times_input < 2 {
        match get_input() {
            Ok(input) => {
                inputs.push(input);
                times_input += 1;
            },
            Err(e) => println!("error: {:?}", e),
        }
    }

    for input in inputs {
        println!("Original: {:?}, Capitalized: {:?}", input, input.to_uppercase());
    }
}