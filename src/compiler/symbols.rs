use std::collections::HashMap;



#[derive(Clone)]
pub struct Symbol {
    pub symb_type: String,
    pub offset: u64
}

#[derive(Clone)]
pub struct SymbolTable {
    outer: Option<Box<SymbolTable>>,
    storage: HashMap<String, Box<Symbol>>,
    pub cur_offset: u64
}


impl SymbolTable {
    pub fn new() -> Self {
        return SymbolTable{
            outer: None,
            storage: HashMap::new(),
            cur_offset: 0

        }
    }
    pub fn move_out(&mut self) -> Self {
        if self.outer.is_none() {
            panic!("Somehow you did move into NONE");
        }
        let table = self.outer.clone().unwrap();
        return *table;
    }
    pub fn add(&mut self, name: String, s: Symbol) {
        if self.cur_offset < s.offset {
            self.cur_offset = s.offset;
        }
        self.storage.insert(name, Box::new(s));
    }
    pub fn get(&self, name: String) -> Option<&Box<Symbol>> {
        self.storage.get(&name)
    }

    pub fn new_from_outer(parent: SymbolTable) -> Self {
        return SymbolTable{
            outer: Some(Box::from(parent)),
            storage: HashMap::new(),
            cur_offset: 0,
        }
    }
}
