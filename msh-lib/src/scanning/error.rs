
use miette::Diagnostic;
use super::span::SourceSpan;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
#[error("get an error when parse text to tokens")]
pub struct ParseDiagnostic{
   #[source_code]
    pub src: String,

   #[label = "errors happen at"]
    pub err_span: SourceSpan,
}

#[derive(Error, Diagnostic, Debug)]
#[error("expect to get {} but get {}",.expect,.actual)]
#[diagnostic()]
pub struct UnexpectCharacterError{
    pub expect:String,
    pub actual:String,
    #[diagnostic_source()]
    pub diagnostic:ParseDiagnostic
}

