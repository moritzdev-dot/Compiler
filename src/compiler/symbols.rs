use std::collections::HashMap;



pub struct Symbol {
    symb_type: String,
    offset: u64
}


pub struct SymbolTable {
    outer: Option<Box<SymbolTable>>,
    storage: HashMap<String, Box<Symbol>>
}


impl SymbolTable {
    pub fn new() -> Self {
        return SymbolTable{
            outer: None,
            storage: HashMap::new()
        }
    }
    pub fn new_from_outer(parent: SymbolTable) -> Self {
        return SymbolTable{
            outer: Some(Box::new(parent)),
            storage: HashMap::new()
        }
    }
}
