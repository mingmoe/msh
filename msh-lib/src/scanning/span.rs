
use super::text::SourceText;
use std::sync::Arc;

/// a part of source
#[derive(Debug)]
pub struct SourceSpan{
    pub source: Arc<SourceText>,
    pub range: (u64,u64)
}

impl SourceSpan{
    pub fn from(source:Arc<SourceText>,range:(u64,u64))->SourceSpan{
        SourceSpan{
            source,
            range
        }
    }
}

impl From<&SourceSpan> for miette::SourceSpan{
    fn from(val: &SourceSpan) -> Self {
       let offset = val.range.0 as usize;
       offset.into()
   }
}

