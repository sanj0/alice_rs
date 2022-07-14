use std::collections::HashMap;

pub const KW_VAR: &str = "var";
pub const KW_FUN: &str = "fun";

pub enum Keyword {
    Var,
    Fun,
}

pub fn keywords() -> HashMap<String, Keyword> {
    let mut kws: HashMap<String, Keyword> = HashMap::with_capacity(2);
    kws.insert(KW_VAR.into(), Keyword::Var);
    kws.insert(KW_FUN.into(), Keyword::Fun);
    kws
}
