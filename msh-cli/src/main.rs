
use msh_lib::scanning::{span::SourceSpan, text::SourceText, token::TokenStream};
use msh_lib::miette::{self, Error, IntoDiagnostic,Report};
use std::rc::Rc;
use std::sync::Arc;

fn main() {
    let source=  Arc::from(SourceText::from_memory("a line\n123yes\nbelow"));
    let mut stream = TokenStream::from(source.clone());

    stream.read_identification().unwrap();
    stream.skip_blank();
    stream.read_identification().unwrap();
    stream.skip_blank();

    match stream.read_identification(){
        Err(err)=>{
            // let report = Report::new(err);
            println!("{:?}",Report::new(err));
        },
        Ok(ok)=>{
        }
    }
}
