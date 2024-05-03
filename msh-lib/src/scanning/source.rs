
#[derive(Debug)]
pub struct CodeSource{
    pub from:String
}

impl CodeSource{

    pub fn unknown_source() -> CodeSource{
        CodeSource::new("unknown code source")
    }
    
    pub fn new(from:&str) -> CodeSource{
        CodeSource{
            from:from.to_string()
        }
    }

    pub fn from_file(path:&str) -> CodeSource{
        CodeSource::new(&format!("file:{}",path))
    }

    pub fn from_memory()->CodeSource{
        CodeSource::new("memory")
    }
}
