use crate::keyword::{keywords, Keyword};
use crate::lexer::*;
use crate::runtime::AliceVal;
use crate::statement::*;
use crate::type_check::*;
use crate::object::*;
use crate::utils::*;

use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;
use std::rc::Rc;

pub const ST_PRINTLN: &str = "println";
pub const ST_PRINT: &str = "print";
pub const ST_PRINT_STACK: &str = "pstack";
pub const ST_EXIT: &str = "exit";
pub const ST_OK_EXIT: &str = "okexit";
pub const ST_DROP: &str = "drop";
pub const ST_SWAP: &str = "swap";
pub const ST_DUP: &str = "dup";
pub const ST_OVER: &str = "over";
pub const ST_ROT: &str = "rot";
pub const ST_CLEAR: &str = "clear";

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

    /// prev = Some(_) assumed interactive mode
    pub fn parse(&self, prev: Option<&mut TypeStack>) -> Result<Vec<Box<dyn Statement>>, String> {
        let mut statements = Vec::new();

        let mut iter = self.tokens.iter().peekable();
        while let Some(token) = iter.next() {
            statements.push(self.gobble_token(token, &mut iter)?);
        }
        if let Some(stack) = prev {
            check_interactive(stack, &statements)?;
            Ok(statements)
        } else {
            check(&statements)?;
            Ok(statements)
        }
    }

    fn gobble_token(&self, token: &AliceToken, iter: &mut TokenIter)
        -> Result<Box<dyn Statement>, String> {
        match token {
            AliceToken::IdentOrKeyw(iok) => {
                self.gobble_ident_or_kw(iok, iter)
            }
            AliceToken::String(s) => self.gobble_string_literal(s, iter),
            AliceToken::Number(f, dec) => {
                self.gobble_number_literal(*f, *dec, iter)
            }
            AliceToken::Op(op) => self.gobble_operator(op, iter),
            _ => todo!(),
        }
    }

    fn gobble_ident_or_kw(
        &self,
        iok: &String,
        iter: &mut TokenIter,
    ) -> Result<Box<dyn Statement>, String> {
        if let Some(kw) = self.keywords.get(iok) {
            self.gobble_kw(kw, iter)
        } else {
            if let Some(statement) = self.maybe_gobble_statement(iok) {
                Ok(statement)
            } else {
                self.gobble_ident(iok, iter)
            }
        }
    }

    fn gobble_kw(&self, kw: &Keyword, iter: &mut TokenIter) -> Result<Box<dyn Statement>, String> {
        Ok(Box::new(match kw {
            Keyword::True => PushStatement(AliceVal::Bool(Some(true))),
            Keyword::False => PushStatement(AliceVal::Bool(Some(false))),
            Keyword::Let => return self.gobble_let(iter),
            Keyword::Fun => return self.gobble_fun(iter),
            _ => todo!(),
        }))
    }

    /// syntax:
    /// let = "let", ident, ":", type, ["=", literal]
    /// where literal can also be sbuject to an @-conversion
    fn gobble_let(&self, iter: &mut TokenIter) -> Result<Box<dyn Statement>, String> {
        if let (Some(AliceToken::IdentOrKeyw(ident)),
            Some(AliceToken::Sep(AliceSeparator::Colon)),
            Some(AliceToken::IdentOrKeyw(ty)))
            = (iter.next(), iter.next(), iter.next()) {
                if self.keywords.contains_key(ident) {
                    return Err(format!("{ident} is a reserved keyword, can't bind a variable to it"))
                }
                Ok(Box::new(LetStatement {
                    ident: ident.into(),
                    ty: type_bit(&AliceVal::for_type_name(ty)?),
                    literal: None
                }))
        } else {
            Err("let syntax: 'let' ident ':' type ['=' literal]".into())
        }
    }

    // syntax:
    // fun = "fun", ident, [":", { type [","] }], ["->", type], block
    fn gobble_fun(&self, iter: &mut TokenIter) -> Result<Box<dyn Statement>, String> {
        if let Some(AliceToken::IdentOrKeyw(ident)) = iter.next() {
            // case 1: no type signature at all
            if let Some(AliceToken::Sep(AliceSeparator::OpenB)) = iter.peek() {
                iter.next();
                let fun = AliceFun {
                    args: StackPattern(Vec::new()),
                    return_type: 0,
                    body: self.gobble_block(iter)?.into_iter().map(|b| box_to_rc(b)).collect()
                };
                fun.type_check()?;
                Ok(Box::new(FunStatement {
                    ident: ident.clone(),
                    fun
                }))
            // case 2: no args but return type
            } else if let Some(AliceToken::Op(AliceOp::Sub)) = iter.peek() {
                iter.next();
                let return_type = self.parse_fun_return_after_dash(iter)?;
                let fun = AliceFun {
                    args: StackPattern(Vec::new()),
                    return_type,
                    body: self.gobble_block(iter)?.into_iter().map(|b| box_to_rc(b)).collect()
                };
                fun.type_check()?;
                Ok(Box::new(FunStatement {
                    ident: ident.clone(),
                    fun
                }))
            // case 3: args + maybe return type
            } else if let Some(AliceToken::Sep(AliceSeparator::Colon)) = iter.next() {
                let mut args = Vec::new();
                let mut return_type = 0u32;
                let mut comma_ok = false;
                while let Some(ty) = iter.next() {
                    match ty {
                        AliceToken::IdentOrKeyw(ty) => {
                            args.push(type_bit_any_allowed(ty)?);
                            comma_ok = true;
                        }
                        AliceToken::Sep(AliceSeparator::Comma) => {
                            if comma_ok {
                                comma_ok = false;
                            } else {
                                return Err("Unexpected double comma in function signature".into())
                            }
                        }
                        AliceToken::Op(AliceOp::Sub) => {
                            return_type = self.parse_fun_return_after_dash(iter)?;
                            break;
                        }
                        AliceToken::Sep(AliceSeparator::OpenB) => break,
                        _ => return Err("Unexpected token in function signature".into()),
                    }
                }
                if args.is_empty() {
                    return Err("expected argument type(s) after `'fun' ident ':'`".into());
                }
                let fun = AliceFun {
                    args: StackPattern(args),
                    return_type,
                    body: self.gobble_block(iter)?.into_iter().map(|b| box_to_rc(b)).collect()
                };
                fun.type_check()?;
                Ok(Box::new(FunStatement {
                    ident: ident.clone(),
                    fun
                }))
            } else {
                Err("after `'fun' ident`, expected one of: `'->'` `':'` `'{'`".into())
            }
        } else {
            Err("fun syntax: 'fun' ident '{' statement* '}'".into())
        }
    }

    fn parse_fun_return_after_dash(&self, iter: &mut TokenIter) -> Result<u32, String> {
        if !matches!(iter.next(), Some(AliceToken::Op(AliceOp::Gt))) {
            return Err("unexpected token after `'fun' ident ... -`, you probably meant to put `->`".into())
        }
        if let (Some(AliceToken::IdentOrKeyw(ty)), Some(AliceToken::Sep(AliceSeparator::OpenB)))
            = (iter.next(), iter.next()) {
            Ok(type_bit(&AliceVal::for_type_name(ty)?))
        } else {
            Err("return type and function body expected".into())
        }
    }

    /// parses tokens into a vec until the closing "}" is found
    fn gobble_block(&self, iter: &mut TokenIter) -> Result<Vec<Box<dyn Statement>>, String> {
        let mut vec: Vec<Box<dyn Statement>> = Vec::new();
        while let Some(tok) = iter.next() {
            if matches!(tok, AliceToken::Sep(AliceSeparator::CloseB)) {
                return Ok(vec);
            }
            vec.push(self.gobble_token(tok, iter)?);
        }
        Err("missing delimiter: hit EOF while searching for '}'".into())
    }

    fn maybe_gobble_statement(&self, ident: &String) -> Option<Box<dyn Statement>> {
        match ident.as_str() {
            ST_PRINTLN => Some(Box::new(PrintlnStatement)),
            ST_PRINT => Some(Box::new(PrintStatement)),
            ST_PRINT_STACK => Some(Box::new(PrintStackStatement)),
            ST_EXIT => Some(Box::new(ExitStatement)),
            ST_OK_EXIT => Some(Box::new(OkExitStatement)),
            ST_DROP => Some(Box::new(DropStatement)),
            ST_SWAP => Some(Box::new(SwapStatement)),
            ST_DUP => Some(Box::new(DupStatement)),
            ST_OVER => Some(Box::new(OverStatement)),
            ST_ROT => Some(Box::new(RotStatement)),
            ST_CLEAR => Some(Box::new(ClearStatement)),
            _ => None,
        }
    }

    fn gobble_ident(&self, ident: &String, iter: &mut TokenIter) -> Result<Box<dyn Statement>, String> {
        // todo!: at conversion
        if let Some(AliceToken::Sep(AliceSeparator::OpenP)) = iter.peek() {
            iter.next();
            if !matches!(iter.next(), Some(AliceToken::Sep(AliceSeparator::CloseP))) {
                Err("closing parentheses in function call missing!".into())
            } else {
                Ok(Box::new(ExecuteFunStatement(ident.clone())))
            }
        } else {
            Ok(Box::new(PushFromTableStatement(ident.clone())))
        }
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
                Ok(None) => {
                    if dec {
                        AliceVal::Float(Some(f))
                    } else {
                        AliceVal::Int(Some(f as i64))
                    }
                }
                Ok(Some(val)) => {
                    return Err(format!(
                        "cannot convert number literal to {}",
                        val.type_name()
                    ))
                }
                Err(e) => return Err(e),
            },
        )))
    }

    fn gobble_operator(
        &self,
        op: &AliceOp,
        iter: &mut TokenIter,
    ) -> Result<Box<dyn Statement>, String> {
        Ok(match op {
            AliceOp::Add => Box::new(AddStatement),
            AliceOp::Sub => Box::new(SubStatement),
            AliceOp::Mul => Box::new(MulStatement),
            AliceOp::Div => Box::new(DivStatement),
            AliceOp::Pow => Box::new(PowStatement),
            AliceOp::Mod => Box::new(ModStatement),
            AliceOp::Gt => todo!(),
            AliceOp::Lt => todo!(),
            AliceOp::Eqs => return Err("unexpected equal sign".into()),
        })
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

