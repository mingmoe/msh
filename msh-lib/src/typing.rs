
use std::vec::Vec;
use ahash::AHashMap;

pub enum Type{
    None,
    NoReturn,
    String(String),
    Number(i64),
    Array(Vec<Type>),
    Map(AHashMap<Type,Type>),
}
