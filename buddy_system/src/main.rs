mod memory_manager;
mod utils;
mod driver;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let state = driver::ProgramState::new(&args);

    let mut state = match state {
        Err(driver::ProgramError::InvalidArgs) => 
            {
                eprintln!("Error parsing arguments: invalid arguments. Given: {}, expected positive integer", args[1]); 
                process::exit(1);
            },
        Err(driver::ProgramError::NotEnoughArgs) => 
            {
                eprintln!("Error parsing arguments: not enough arguments");
                process::exit(1);
            },
        Ok(state) => state,
        _ => {
            eprintln!("Error: Unexpected error");
            process::exit(1);
        }
    };

    println!("¡Bienvenido al simulador de memoria con buddy system!");
    println!("¿Qué puedo hacer por tí?");
    while !state.exit() { state.run() }
}