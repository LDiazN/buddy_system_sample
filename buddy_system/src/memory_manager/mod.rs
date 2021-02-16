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

    pub fn allocate(&mut self, name : String, size : usize) -> Result<Address, MemoryError> {
        
        // Check if too much requested memory to even bother
        if  size > self.memory.len()    { return Err(MemoryError::OutOfMemory) }

        // Check if name is defined
        if  self.is_symbol(&name)       { return Err(MemoryError::AlreadyExistingSymbol) }

        // run our algorithm
        let chunk_size = utils::nearest_upper_2_power(size);
        for (block_size, list) in &mut self.blocks {

            if *block_size < chunk_size || list.is_empty() { continue }

            // Create a new symbol
            let pos = list.pop().unwrap();
            let data = SymbolData {
                position : pos,
                size : size,
                chunk_size : chunk_size,
                id : self.next_available_id
            };

            self.names.insert(name, data);

            // update chunks
            let size = *block_size;
            self.create_chunks(pos + chunk_size, pos + size - 1);

            return Ok(pos)
        }


        Err(MemoryError::OutOfMemory)
    }

    pub fn free(&mut self, name : &String) -> Result<(),MemoryError>{

        // get data for this symbol and then delete it 
        let (chunk_size, position) = {
            match self.names.get(name) {
                None => return Err(MemoryError::SymbolNotDefined),
                Some(d) => (d.chunk_size, d.position)
            }            
        };

        self.names.remove(name);

        // return this symbol's memory to the block;
        let entry = self.blocks.entry(chunk_size).or_insert(vec![]);
        entry.push(position);

        Ok(())
    }

    pub fn show(&self) {
        
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

    pub fn is_symbol(&self, name : &String) -> bool{
        self.names.contains_key(name)
    }
}