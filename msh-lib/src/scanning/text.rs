use std::fs;
use std::path::PathBuf;

use super::source::CodeSource;

#[derive(Debug)]
pub struct SourceText{
    pub source: CodeSource,
    pub text: String,
    pub text_chars:Vec<(u64,char)>
}

impl SourceText {

    fn text_to_chars(text:&str)->Vec<(u64,char)>{
        let mut chars:Vec<(u64,char)> = Vec::with_capacity(text.len());

        for char in text.char_indices(){
            chars.push((char.0 as u64,char.1));
        }

        chars
    }

    pub fn from_file(path:&str)->SourceText{
        let abs_path= fs::canonicalize(PathBuf::from(path)).unwrap();
        let text = fs::read_to_string(&abs_path).expect("read modern-shell script from file should work");

        SourceText{
            source: CodeSource::from_file(abs_path.to_str().unwrap()),
            text_chars:SourceText::text_to_chars(&text),
            text
        }
    }

    pub fn from_memory(text:&str)->SourceText{
        let text = text.to_string();
        SourceText{
            source: CodeSource::from_memory(),
            text_chars:SourceText::text_to_chars(&text),
            text
        }
    }
}

