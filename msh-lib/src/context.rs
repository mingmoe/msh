
use ahash::AHashMap;
use std::collections::VecDeque;
use std::iter::Rev;
use std::rc::Rc;
use crate::typing::Type;

pub struct Context{
    /// variables table
    scopes : VecDeque<Rc<Scope>>,
}

pub struct Scope{
    pub name:String,
    pub variables_table: Rc<AHashMap<String,Type>>
}

impl Scope{
    pub fn new(name:&str)->Scope{
        Scope{
            name: name.to_string(),
            variables_table: Rc::from(AHashMap::new())
        }
    }
}

impl Default for Context{
    fn default() -> Self {
        Self::new()
    }
}

impl Context{

    pub fn new() -> Context{
        let mut scopes = VecDeque::new();
        scopes.push_back(Rc::from(Scope::new("global")));
        Context {
            scopes
        }
    }

    pub fn global(&self) -> Rc<Scope>{
        self.scopes.front().unwrap().clone()
    }

    /// get the variables table.
    /// the argument `back` is mean how many steps back to get variables table.
    /// for instance, 0 stands for get current scope variables.
    /// It may returns the global variables table.
    pub fn get_scope(&self,back:usize) -> Rc<Scope>{
        let index = self.scopes.len() - back - 1;
        return self.scopes.get(index).unwrap().clone()
    }

    pub fn new_scope(&mut self,scope:Rc<Scope>){
        self.scopes.push_back(scope);
    }

    /// this would end the global scope!!!
    /// If end the global scope,means the context done its work,should not reuse it.
    pub fn end_scope(&mut self) -> Option<Rc<Scope>> {
        self.scopes.pop_back()
    }

    /// get all the variables table.
    /// the order is from current(local) to global.
    pub fn get_all_scopes(&self) -> Rev<std::collections::vec_deque::Iter<Rc<Scope>>>{
        self.scopes.iter().rev()
    }
}

