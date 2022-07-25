use std::collections::HashMap;

pub const KW_LET: &str = "let";
pub const KW_FUN: &str = "fun";
pub const KW_TRUE: &str = "true";
pub const KW_FALSE: &str = "false";
pub const KW_IF: &str = "if";
pub const KW_ELSE: &str = "else";

pub enum Keyword {
    Let,
    Fun,
    True,
    False,
    If,
    Else,
}

pub fn keywords() -> HashMap<String, Keyword> {
    let mut kws: HashMap<String, Keyword> = HashMap::with_capacity(2);
    kws.insert(KW_LET.into(), Keyword::Let);
    kws.insert(KW_FUN.into(), Keyword::Fun);
    kws.insert(KW_TRUE.into(), Keyword::True);
    kws.insert(KW_FALSE.into(), Keyword::False);
    kws.insert(KW_IF.into(), Keyword::If);
    kws.insert(KW_ELSE.into(), Keyword::Else);
    kws
}
