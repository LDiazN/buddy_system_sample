use std::collections::{HashMap, BTreeMap};

/// Position in array
pub type Address = usize; 
/// Memory segment
pub type Memory  = Vec<u8>;
/// Map from name to data
pub type NameMap = HashMap<String, SymbolData>;
/// Map from block type to positions with such size
pub type BlockLists = BTreeMap<usize, Vec<Address>>;


/// Data defining a symbol
pub struct SymbolData {
    pub id : usize,
    pub position : Address,
    pub size : usize,
    pub chunk_size : usize
}

/// Memory manager object
pub struct MemoryManager {
    memory : Memory,
    names  : NameMap,
    blocks : BlockLists,
    next_available_id : usize
}

