use std::collections::HashMap;

pub const KW_LET: &str = "let";
pub const KW_FUN: &str = "fun";

pub enum Keyword {
    Let,
    Fun,
}

pub fn keywords() -> HashMap<String, Keyword> {
    let mut kws: HashMap<String, Keyword> = HashMap::with_capacity(2);
    kws.insert(KW_LET.into(), Keyword::Let);
    kws.insert(KW_FUN.into(), Keyword::Fun);
    kws
}
