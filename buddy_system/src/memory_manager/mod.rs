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
#[derive(Debug, PartialEq)]
pub struct SymbolData {
    pub id : usize,
    pub position : Address,
    pub size : usize,
    pub chunk_size : usize
}

#[derive(Debug, PartialEq)]
/// Memory manager object
pub struct MemoryManager {
    memory : Memory,
    names  : NameMap,
    blocks : BlockLists,
    next_available_id : usize
}

/// Possible memory errors
#[derive(Debug)]
pub enum MemoryError {
    OutOfMemory,
    SymbolNotDefined,
    AlreadyExistingSymbol
}

impl MemoryManager {

    /// Create a new memory manager that manages a chunk of memory with the provided capacity
    /// ## Parameters
    /// * `capacity` - how much memory will be managed by this manager
    /// ---
    /// ## Return
    /// A memory manager a maximum memory of 'capacity'
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

    /// Allocate Memory for 
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

            self.next_available_id = (self.next_available_id + 1) % (usize::MAX - 1);
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
        self.show_memory();
        self.show_variables();
        self.show_blocks();
    }

    pub fn get_blocks(&self) -> &BlockLists {
        &self.blocks
    }

    pub fn get_names(&self) -> &NameMap {
        &self.names
    }

    pub fn get_memory(&self) -> &Memory {
        &self.memory
    }

    fn show_memory(&self) {
        let mut temp_mem = vec![0; self.memory.len()];

        // create temporal simulated memory
        for (_, data) in &self.names {
            for i in data.position .. (data.position + data.chunk_size) {
                temp_mem[i] = if i < data.position + data.size {data.id as isize} else {-1};
            }
        }

        // print title
        println!("[Memory]");
        // print items. If it's unused memory, print a dot instead of its id
        print!("    ");
        for i in temp_mem {
            print!("{} ", if i < 0 {String::from(".")} else {i.to_string()})
        }

        println!("");
    }

    fn show_variables(&self) {

        // Print title
        println!("[Variables]");

        // Print actual variables
        for (name, data) in &self.names{
            println!("  {}", name);
            println!("    id: {}, size: {}, chunk size: {}, position: {}", data.id, data.size, data.chunk_size, data.position);
        }
    }

    fn show_blocks(&self) {
        // Print title
        println!("[Blocks]");

        // print blocks
        for (size, list) in &self.blocks {
            println!("  Blocks of size: {}", size);
            print!("   positions: ");
            for i in list {
                print!("{} ", i);
            }
            println!();
        }
    }

    fn create_chunks(&mut self, start : usize, end : usize) {

        // Check if we actually have something to do
        if end < start || start >= self.memory.len()  { return; }

        let mut capacity = end - start + 1;
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