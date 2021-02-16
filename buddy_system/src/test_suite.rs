use crate::memory_manager::*;
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