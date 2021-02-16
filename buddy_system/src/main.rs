mod memory_manager;
mod utils;

fn main() {
    let mut manager = memory_manager::MemoryManager::new(20);
    manager.show();

    manager.allocate(String::from("x"), 9);
    
    println!("----");
    manager.show();
    
}