use crate::memory_manager::*;
#[allow(unused)]
use std::collections::{HashMap, BTreeMap};

#[test]
fn test_manager_creation1 (){
    let manager = MemoryManager::new(10);
    let mut expected_block_list : BlockLists = BTreeMap::new();
    let expected_name_list : NameMap = HashMap::new();
    let expected_memory = vec![0; 10];

    expected_block_list.insert(8,vec![0]);
    expected_block_list.insert(2,vec![8]);

    assert_eq!(*manager.get_blocks(), expected_block_list);
    assert_eq!(*manager.get_memory(), expected_memory);
    assert_eq!(*manager.get_names(), expected_name_list);
}

#[test]
fn test_manager_creation2 (){
    let manager = MemoryManager::new(1);
    let mut expected_block_list : BlockLists = BTreeMap::new();
    let expected_name_list : NameMap = HashMap::new();
    let expected_memory = vec![0; 1];

    expected_block_list.insert(1,vec![0]);

    assert_eq!(*manager.get_blocks(), expected_block_list);
    assert_eq!(*manager.get_memory(), expected_memory);
    assert_eq!(*manager.get_names(), expected_name_list);
}

#[test]
fn test_manager_creation3 (){
    let manager = MemoryManager::new(8);
    let mut expected_block_list : BlockLists = BTreeMap::new();
    let expected_name_list : NameMap = HashMap::new();
    let expected_memory = vec![0; 8];

    expected_block_list.insert(8,vec![0]);

    assert_eq!(*manager.get_blocks(), expected_block_list);
    assert_eq!(*manager.get_memory(), expected_memory);
    assert_eq!(*manager.get_names(), expected_name_list);
}

#[test]
fn test_should_allocate_mem() {
    let mut manager = MemoryManager::new(20);
    // allocating with enough space should work
    
    assert!(manager.allocate(String::from("x"), 8).is_ok());
    assert!(manager.allocate(String::from("z"), 8).is_ok());
}

#[test]
fn test_should_scream_too_much_memory() {
    let mut manager = MemoryManager::new(20);

    // Requesting too much memory should trigger an obvios out of memory
    assert_eq!(manager.allocate(String::from("ohno"), 999999),Err(MemoryError::OutOfMemory))
}

#[test]
fn test_should_scream_too_much_memory_even_with_enough() {
    let mut manager = MemoryManager::new(20);

    // Due to the buddy system nature, it shouldn't be possible to 
    // allocate memory too near to the max available memory
    assert_eq!(manager.allocate(String::from("ohno"), 18),Err(MemoryError::OutOfMemory))
}

#[test]
fn test_should_scream_already_created() {
    let mut manager = MemoryManager::new(20);

    // creating the same variable twice should trigger an error
    let _ = manager.allocate(String::from("x"), 4);

    assert_eq!(manager.allocate(String::from("x"), 2),Err(MemoryError::AlreadyExistingSymbol))
}

#[test]
fn test_delete_ok() {
    let mut manager = MemoryManager::new(20);

    
    let _ = manager.allocate(String::from("x"), 4);
    assert!(manager.free(&String::from("x")).is_ok());
}


#[test]
fn test_delete_not_available() {
    let mut manager = MemoryManager::new(20);

    // Symbol should not be available after free
    let _ = manager.allocate(String::from("x"), 4);
    let _ = manager.free(&String::from("x"));

    let names = manager.get_names();

    assert_eq!(names.get(&String::from("x")), None);
}