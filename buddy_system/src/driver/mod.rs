use crate::memory_manager::{MemoryManager, MemoryError};
use std::io;
use std::io::Write;

/// Program state at any moment
pub struct ProgramState {
    manager : MemoryManager,
    _allocated_memory : usize,
    exit : bool
}

/// Possible Errors
pub enum ProgramError {
    InvalidArgs,
    NotEnoughArgs,
    InvalidAction(String),
    IvalidActionArgs(Vec<String>),
    NotEnoughActionArgs(usize, usize) // give, expected
}

/// Possible actions
enum ProgramAction {
    Show,
    Exit,
    Allocate(String, usize),
    Free(String)
}

impl ProgramState {

    /// Creates a new Program object by parsing its data from command line arguments
    /// ## Parameters
    /// * `args` - a string vector containing the command line arguments for this program
    /// ---
    /// ## Return
    /// A program, or an error depending on how was the argument parsing
    pub fn new(args : &Vec<String>) -> Result<ProgramState, ProgramError> {
        // not enough arguments
        if args.len() < 2 { return Err(ProgramError::NotEnoughArgs) }

        // Parse memory
        let memory_ammount = args[1].parse::<usize>();
        
        // if it's a valid argument, create a program
        match memory_ammount {
            Err(_) => Err(ProgramError::InvalidArgs),
            Ok(n)  => Ok( ProgramState {
                manager : MemoryManager::new(n),
                _allocated_memory : n,
                exit : false
            })
        }
    }


    /// Run one iteration of this program. If you want a main loop,
    /// call this function as much as you want.
    pub fn run(&mut self) {
        let mut line = String::new();

        print!(">> "); // print prompt
        // so the print! doesn't mess up the execution order with read_line
        io::stdout().flush().expect("Couldn't flush stdout"); 

        // Read a single line
        match io::stdin().read_line(&mut line) {
            Err(_) => panic!("Error leyendo input D:"),
            Ok(_)  => {}
        }

        // Do something depending on the action type
        match ProgramState::parse(line) {
            Err(e)   => ProgramState::print_errors(e), // if there was an error, just print it

            Ok(ProgramAction::Show) => self.manager.show(), // just show

            Ok(ProgramAction::Exit) => self.exit = true, // Next iteration will stop the program

            Ok(ProgramAction::Allocate(name, size)) => { // allocate memory
                match self.manager.allocate(name, size) {
                    // If there was an error, just print it
                    Err(MemoryError::AlreadyExistingSymbol) => {
                        eprintln!("🚨 Esa variable ya existe, intenta con una nueva.");
                    },
                    Err(MemoryError::OutOfMemory) => {
                        eprintln!("🔋 No tengo suficiente memoria para asignarte {} bytes.", size);
                    },
                    Ok(addr) => { // Tell the user where is her variable
                        println!("Tu variable tiene la dirección de memoria {}", addr);
                    },
                    _ => eprintln!("🐞 Unexpected Error") // something went wrong, programming error
                }
            },

            Ok(ProgramAction::Free(name)) => { // free memory
                match self.manager.free(&name) {
                    Ok(_)                              => println!("¡Memoria liberada!"),
                    Err(MemoryError::SymbolNotDefined) => eprintln!("🚨 Ese símbolo no está definido."),
                    Err(_)                             => eprintln!("🐞 Unexpected Error")
                }
            }
        }
    }

    /// Parse a string into an actual ProgramAction, or an error if 
    /// something happen
    fn parse(input : String) -> Result<ProgramAction, ProgramError>{
        let mut args = input.split_whitespace().map(|s| s.to_string());
        let action = match args.next() {
            None => { return Err(ProgramError::NotEnoughArgs) },
            Some(s) => s
        }.to_lowercase();

        let allocate = String::from("reservar");
        let free     = String::from("liberar");
        let show     = String::from("mostrar");
        let exit     = String::from("salir");

        if action == allocate { 
            ProgramState::parse_allocation(args.collect::<Vec<String>>())
        }
        else if action == show {
            Ok(ProgramAction::Show)
        }
        else if action == exit {
            Ok(ProgramAction::Exit)
        }
        else if action == free {
            ProgramState::parse_free(args.collect::<Vec<String>>())
        }
        else {
            Err(ProgramError::InvalidAction(action))
        }
    }

    /// parse an allocation action
    fn parse_allocation(args : Vec<String> ) -> Result<ProgramAction, ProgramError> {

        // Check how many args
        if args.len() < 2 { return Err(ProgramError::NotEnoughActionArgs(args.len(), 2)); }

        // Check valid number
        let size = match args[1].parse::<usize>() {
            Err(_) => return Err(ProgramError::IvalidActionArgs(args)),
            Ok(n)  => n
        };

        Ok(ProgramAction::Allocate(args[0].clone(), size))
    }

    /// parse a free action
    fn parse_free(args : Vec<String>) -> Result<ProgramAction, ProgramError> {
        if args.len() < 1 
        {
            Err(ProgramError::NotEnoughActionArgs(0,1))
        }
        else {
            Ok(ProgramAction::Free(args[0].clone()))
        }
    }

    /// Print error messages
    fn print_errors(error : ProgramError) {
        
        match error {
            ProgramError::InvalidAction(s) => {
                eprintln!("🚨 Accion no valida: {}", s);
                ProgramState::usage();
            },
            ProgramError::NotEnoughActionArgs(given, expected) => {
                eprintln!("🚨 No hay suficientes argumentos para esta accion. Necesito: {}, recibí: {}", expected, given);
            },
            ProgramError::IvalidActionArgs(args) => {
                eprintln!("🚨 Argumentos incorrectos para esta accion: ");
                eprint!("  ");
                for arg in args {
                    eprint!("{} ", arg);
                }
                eprintln!("");
            },
            _ => eprintln!("🐞 Unexpected Error")
        }
    }

    /// If the program should stop
    pub fn exit(&self) -> bool {
        self.exit
    }

    /// Print how to use our commands
    fn usage() {
        println!("Uso:\n  <accion> [args]");
        println!("Posibles acciones: ");
        println!("  * reservar <nombre_variable> <cantidad_bytes>");
        println!("  * liberar <nombre_variable>");
        println!("  * mostrar");
        println!("  * salir");
    }
}