use miette::Diagnostic;
use unicode_ident::{is_xid_continue,is_xid_start};

use super::{error::{ParseDiagnostic, UnexpectCharacterError, UnexpectEndOfSource}, span::SourceSpan, text::SourceText};
use std::{rc::Rc, sync::Arc};

pub enum TokenType{
    /// String with quotes like "str"
    String(String),
    /// the name of variable and so on.
    /// follow the Unicode® Standard Annex #31 standard
    Idenfification(String),
    Number(i64),
    Float(f64),
    /// -
    Sub, 
    /// +
    Add,
    /// *
    Mul,
    /// /
    Div,
    /// >
    GreatThan,
    /// <
    LessThan,
    /// >=
    GreatThanOrEqual,
    /// <=
    LessThanOrEqual,
    /// =
    Assign,
    /// ==
    Equal,
}

pub struct Token{
    pub typing:TokenType,
    pub span:SourceSpan
}

pub struct TokenStream{
    pub source: Arc<SourceText>,
    index:u64
}

impl Token{
    pub fn from(typing:TokenType,source:Arc<SourceText>,from:u64,length:u64) -> Token{
        Token{
            typing,
            span: SourceSpan::from(source.clone(),(from,length))
        }
    }
}

impl TokenStream{
    pub fn from(source:Arc<SourceText>)->TokenStream{
        TokenStream{
            source,
            index:0
        }
    }

    pub fn get_line_begin(&self)->u64{
        let mut rindex = self.index;

        if rindex == 0{
            return 0
        }
        rindex -= 1; // if we are on the end of line,back

        while rindex != 0{
            if let Some(c) = self.source.text_chars.get(rindex as usize) && c.1 == '\n'{
                break;
            }
            rindex -= 1;
        }

        rindex + 1
    }

    pub fn get_line_end(&self) -> u64{
        let mut current = self.index;

        while let Some(c) = self.source.text_chars.get(current as usize) && c.1 != '\n'{
            current += 1;
        }

        current
    }

    pub fn get_current_line_span(&self) -> super::span::SourceSpan{
        SourceSpan::from(
            self.source.clone(),
            (self.get_line_begin(),self.get_line_end())
        )
    }

    fn get_diagnotic(&self,length:u64)->ParseDiagnostic{
        let index = self.source.text_chars.get(self.index as usize).unwrap().0;
        ParseDiagnostic{
            src: self.source.text.clone(),
            err_span: SourceSpan::from(self.source.clone(),(index,index+length))
        }
    }

    fn get_token(&self,typing:TokenType,from:u64,length:u64)->Token{
        Token::from(typing, self.source.clone(), from, length)
    }

    pub fn is_front(&self)->bool{
        self.index == 0
    }

    pub fn is_end(&self) -> bool{
        self.index >= self.source.text_chars.len() as u64
    }

    pub fn move_next(&mut self) -> bool{
        if self.is_end() {
            return false;
        }
        self.index+=1;
        self.is_end()
    }

    pub fn move_back(&mut self){
        if self.index == 0{
            return;
        }

        self.index -= 1;
    }

    pub fn get_current(&self)->Option<char>{
        match self.is_end(){
            true => None,
            false => Some((self.source.text_chars.get(self.index as usize).unwrap()).1)
        }
    }

    pub fn get_some(&self,length:u64)->Option<String>{
        let mut got = String::with_capacity(length as usize);

        while let Some(c) = self.get_current() && got.len() != length as usize{
            got.push(c);
        }

        match got.len() as u64 == length{
            true => Some(got),
            false=> None
        }
    }

    pub fn skip_blank(&mut self){
        while let Some(c) = self.get_current() && c.is_whitespace(){
            self.move_next();
        }
    }

    pub fn skip_line(&mut self){
        while let Some(c) = self.get_current() && c != '\n'{
            self.move_next();
        }
        self.move_next();
    }

    pub fn read_identification(&mut self) -> Result<String, UnexpectCharacterError>{
        let mut identification = String::with_capacity(32);

        if let Some(c) = self.get_current() && is_xid_start(c){
            identification.push(c);
        }
        else{
            return Err(UnexpectCharacterError{
                diagnostic: self.get_diagnotic(1),
                expect: "a unicode xid start code point".to_string(),
                actual: "others".to_string()
            });
        }

        while let Some(c) = self.get_current() && is_xid_continue(c){
            identification.push(c);
            self.move_next();
        }

        Ok(identification)
    }

    pub fn read_string(&mut self)->Result<String,Box<dyn Diagnostic>>{
        if let Some(c) = self.get_current() && c == '"'{
            self.move_next();
        }
        else{
            return Err(Box::new(UnexpectCharacterError{
                diagnostic: self.get_diagnotic(1),
                expect: "a string begin with \"".to_string(),
                actual: "others".to_string()
            }));
        }

        let mut string = String::with_capacity(32);

        let mut end = false;
        while let Some(c) = self.get_current(){
            if c == '"'{
                end = true;
                self.move_next();
                break;
            }

            if c == '\\'{
                // try move
                self.move_next();
                if let Some(next_c) = self.get_current(){
                    match next_c{
                        '0' =>{
                            string.push('\0');
                        },
                        '\\' =>{
                            string.push('\\');
                        },
                        'r'=>{
                            string.push('\r');
                        },
                        'n' => {
                            string.push('\n');
                        },
                        't' => {
                            string.push('\t');
                        },
                        '\''=>{
                            string.push('\'');
                        },
                        '"'=>{
                            string.push('"');
                        },
                        // as the two characters version of \u
                        'x'=>{
                            // get next 2 character
                            match self.get_some(2){
                                Some(str)=>{
                                    string.push(char::from_u32(u32::from_str_radix(&str,16).unwrap()).unwrap());
                                },
                                None=>{
                                    return Err(Box::new(UnexpectEndOfSource{
                                        diagnostic: self.get_diagnotic(1),
                                        expect: "a two-characters hexadecimal unicode code point after \\x".to_string(),
                                    }));
                                }
                            }
                        },
                        'u' => {
                            // get next 4 character
                            match self.get_some(4){
                                Some(str)=>{
                                    string.push(char::from_u32(u32::from_str_radix(&str,16).unwrap()).unwrap());
                                },
                                None=>{
                                    return Err(Box::new(UnexpectEndOfSource{
                                        diagnostic: self.get_diagnotic(1),
                                        expect: "a four-characters hexadecimal unicode code point after \\u".to_string(),
                                    }));
                                }
                            }
                        },
                        _ => {
                         return Err(Box::new(UnexpectCharacterError{
                            diagnostic: self.get_diagnotic(1),
                            expect: "a escape sequence after \\".to_string(),
                            actual: "unknown escape sequence".to_string()
                        }));               
                        }
                    }
                }
                else{
                    self.move_back();
                    return Err(Box::new(UnexpectEndOfSource{
                        expect: "a escape sequence after \\".to_string(),
                        diagnostic: self.get_diagnotic(1)
                    }));

                }
            }

            else{
                string.push(c);
            }
            self.move_next();
        }

        if !end{
            return Err(Box::new(UnexpectEndOfSource{
                expect: "a \" to end string".to_string(),
                diagnostic: self.get_diagnotic(1)
            }));
        }

        Ok(string)
    }
}

#[cfg(test)]
pub mod test{
    use super::*;

    #[test]
    fn token_stream_parse() {
        let source=  Arc::from(SourceText::from_memory("a line\n123yes\nbelow"));
        let mut stream = TokenStream::from(source.clone());

        stream.read_identification().unwrap();
        stream.skip_blank();
        stream.read_identification().unwrap();
        stream.skip_blank();
        let r = stream.read_identification();
        assert!(r.is_err());
    }

}
