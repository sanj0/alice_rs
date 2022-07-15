use crate::keyword::{keywords, Keyword};
use crate::lexer::{AliceOp, AliceSeparator, AliceToken};
use crate::runtime::AliceVal;
use crate::statement::*;

use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

pub const ST_PRINTLN: &str = "println";
pub const ST_PRINT: &str = "print";
pub const ST_PRINT_STACK: &str = "pstack";
pub const ST_EXIT: &str = "exit";
pub const ST_OK_EXIT: &str = "okexit";

type TokenIter<'a> = Peekable<Iter<'a, AliceToken>>;

pub struct AliceParser {
    tokens: Vec<AliceToken>,
    keywords: HashMap<String, Keyword>,
}

impl AliceParser {
    pub fn new(tokens: Vec<AliceToken>) -> Self {
        Self {
            tokens,
            keywords: keywords(),
        }
    }

    pub fn parse(&self) -> Result<Vec<Box<dyn Statement>>, String> {
        let mut statements = Vec::new();

        let mut iter = self.tokens.iter().peekable();
        while let Some(token) = iter.next() {
            match token {
                AliceToken::IdentOrKeyw(iok) => {
                    statements.push(self.gobble_ident_or_kw(iok, &mut iter))
                }
                AliceToken::String(s) => statements.push(self.gobble_string_literal(s, &mut iter)?),
                AliceToken::Number(f, dec) => statements.push(self.gobble_number_literal(*f, *dec, &mut iter)?),
                _ => todo!(),
            }
        }
        Ok(statements)
    }

    fn gobble_ident_or_kw(&self, iok: &String, iter: &mut TokenIter) -> Box<dyn Statement> {
        let keyword = self.keywords.get(iok);
        if let Some(kw) = keyword {
            self.gobble_kw(kw, iter)
        } else {
            if let Some(statement) = self.maybe_gobble_statement(iok) {
                statement
            } else {
                self.gobble_ident(iok, iter)
            }
        }
    }

    fn gobble_kw(&self, kw: &Keyword, iter: &mut TokenIter) -> Box<dyn Statement> {
        todo!()
    }

    fn maybe_gobble_statement(&self, ident: &String) -> Option<Box<dyn Statement>> {
        match ident.as_str() {
            ST_PRINTLN => Some(Box::new(PrintlnStatement)),
            ST_PRINT => Some(Box::new(PrintStatement)),
            ST_PRINT_STACK => Some(Box::new(PrintStackStatement)),
            ST_EXIT => Some(Box::new(ExitStatement)),
            ST_OK_EXIT => Some(Box::new(OkExitStatement)),
            _ => None,
        }
    }

    fn gobble_ident(&self, ident: &String, iter: &mut TokenIter) -> Box<dyn Statement> {
        todo!()
    }

    fn gobble_string_literal(
        &self,
        s: &String,
        iter: &mut TokenIter,
    ) -> Result<Box<dyn Statement>, String> {
        Ok(Box::new(PushStatement(
            match self.maybe_at_conversion(iter) {
                Ok(Some(AliceVal::String(_))) => AliceVal::String(Some(s.to_string())),
                Ok(Some(_)) => todo!(),
                Ok(None) => AliceVal::String(Some(s.to_string())),
                Err(e) => return Err(e),
            },
        )))
    }

    fn gobble_number_literal(
        &self,
        f: f64,
        dec: bool,
        iter: &mut TokenIter,
    ) -> Result<Box<dyn Statement>, String> {
        Ok(Box::new(PushStatement(
            match self.maybe_at_conversion(iter) {
                Ok(Some(AliceVal::Float(_))) => AliceVal::Float(Some(f)),
                Ok(Some(AliceVal::Int(_))) => AliceVal::Int(Some(f as i64)),
                Ok(Some(AliceVal::String(_))) => AliceVal::String(Some(f.to_string())),
                Ok(None) => if dec { AliceVal::Float(Some(f)) } else { AliceVal::Int(Some(f as i64)) },
                Ok(Some(val)) => return Err(format!("cannot convert number literal to {}", val.type_name())),
                Err(e) => return Err(e),
            },
        )))
    }

    /// returns
    /// - `Ok(Some(_))` if the next two tokens parsed to a valid @ conversion
    /// - `Ok(None)` if the next tokens was not an @-separator in the first place
    /// - `Err(_)` if the next token(s) parsed to an invalid @ conversion (e. g. missing type)
    fn maybe_at_conversion(&self, iter: &mut TokenIter) -> Result<Option<AliceVal>, String> {
        match iter.peek() {
            Some(AliceToken::Sep(AliceSeparator::At)) => {
                iter.next();
                match iter.next() {
                    None => Err("missing target type for @ conversion".into()),
                    Some(AliceToken::IdentOrKeyw(iok)) => {
                        if let Ok(ty) = AliceVal::for_type_name(iok) {
                            Ok(Some(ty))
                        } else {
                            Err(format!("unexpected token '{iok}' that is not a type; @ conversion expects target type"))
                        }
                    }
                    Some(tok) => Err(format!(
                        "unexpected token {tok:?}; @ conversion expects target type"
                    )),
                }
            }

            None | Some(_) => Ok(None),
        }
    }
}
