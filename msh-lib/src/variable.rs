use crate::typing::Type;

pub struct Variable{
    pub name : String,
    pub value : Type,
}

impl Variable{
    pub fn none(name:&str) -> Variable{
        Variable {
            name: name.to_string(),
            value : Type::None
        }
    }
    pub fn no_return(name:&str) -> Variable{
        Variable {
            name : name.to_string(),
            value : Type::NoReturn
        }
    }
}

