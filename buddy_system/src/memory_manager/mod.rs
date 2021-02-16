use std::collections::{HashMap, BTreeMap};
use crate::utils;

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

/// Possible memory errors
pub enum MemoryError {
    OutOfMemory,
    SymbolNotDefined,
    AlreadyExistingSymbol
}

impl MemoryManager {

    pub fn new(capacity : usize) -> MemoryManager {
        let mut manager = MemoryManager{
            memory : vec![0; capacity],
            names : HashMap::new(),
            blocks : BTreeMap::new(),
            next_available_id : 1
        };

        manager.create_chunks(0, capacity - 1);

        manager
    }

    pub fn allocate() {

    }

    fn create_chunks(&mut self, start : usize, end : usize) {

        // Check if we actually have something to do
        if end > start || start >= self.memory.len()  { return; }

        let mut capacity = end - start;
        let mut i = start;

        while capacity > 0 && i <= end {
            let current_size = utils::nearest_lower_2_power(capacity);
            self.insert_chunk(current_size, i);

            capacity -= current_size;
            i += current_size;
        }
    }

    fn insert_chunk(&mut self, size : usize, position : usize) {
        let entry  = self.blocks.entry(size).or_insert(vec![]);
        entry.push(position);
    }

    fn insert_symbol(&mut self, name : String, symbol_data : SymbolData) -> Result<(), MemoryError>{

        self.names.insert(name, symbol_data);
        return Ok(());
    }

}