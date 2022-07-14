use std::iter::Peekable;
use std::num::{ParseFloatError, ParseIntError};
use std::str::Chars;

use crate::loc::Loc;

#[derive(Debug)]
pub struct AliceLexer {
    src: String,
    loc: Loc,
}

// when adding a new item, must modify all places comment-marked:
// on_add_token
#[derive(Debug)]
pub enum AliceToken {
    IdentOrKeyw(String),
    String(String),
    Number(f64),
    Sep(AliceSeparator),
    Op(AliceOp),
}

// when adding a new item, must modify all places comment-marked:
// on_add_sep
#[derive(Debug)]
pub enum AliceSeparator {
    OpenP,  // (
    CloseP, // )
    OpenB,  // {
    CloseB, // }
    OpenS,  // [
    CloseS, // ]
    Comma,  // ,
    Period, // .
    Colon,  // :
    Semi,   // ;
    At,     // @
}

// when adding a new item, must modify all places comment-marked:
// on_add_op
#[derive(Debug)]
pub enum AliceOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow, // **
    Mod,
    Eqs,
}

#[derive(Debug)]
pub enum AliceLexerErr {
    MissingDelimeter(String, Loc),
    HitEOFWhileParsing(String, Loc),
    IllegalEscapeSequence(String, Loc),
    NumberFormatErr(String, Loc),
    UnexpectedSymbol(String, Loc),
}

impl AliceLexer {
    pub fn new(src: String, file: String) -> Self {
        Self {
            src,
            loc: Loc::new(file, 1, 1),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<AliceToken>, AliceLexerErr> {
        let mut tokens = Vec::<AliceToken>::new();
        let mut char_iter = self.src.chars().peekable();
        while let Some(c) = char_iter.next() {
            if c.is_whitespace() {
                if c == '\n' {
                    self.loc.line += 1;
                    self.loc.column = 1;
                } else {
                    self.loc.column += 1;
                }
                continue;
            }
            let token = self.gobble_token(c, &mut char_iter)?;
            tokens.push(token);
        }
        Ok(tokens)
    }

    fn gobble_token(
        &self,
        start: char,
        iter: &mut Peekable<Chars>,
    ) -> Result<AliceToken, AliceLexerErr> {
        match start {
            '"' | '\'' => self.gobble_string(start, iter),
            n if n.is_digit(10) => self.gobble_number(start, iter),
            o if AliceOp::contains(&o) => self.gobble_operator(start, iter),
            s if AliceSeparator::contains(&s) => self.gobble_separator(s, iter),
            _ => self.gobble_ident_or_keyw(start, iter),
        }
    }

    fn gobble_string(
        &self,
        end: char,
        iter: &mut Peekable<Chars>,
    ) -> Result<AliceToken, AliceLexerErr> {
        let mut s = String::new();
        let mut escaped = false;
        while let Some(c) = iter.peek() {
            let c = *c;
            if escaped {
                match c {
                    '\\' | '"' | '\'' => s.push(c),
                    'n' => s.push('\n'),
                    'r' => s.push('\r'),
                    't' => s.push('\t'),
                    _ => {
                        return Err(AliceLexerErr::IllegalEscapeSequence(
                            format!("unknown escape sequence \\{c}"),
                            self.loc.clone(),
                        ))
                    }
                }
                escaped = false;
            } else if c == end {
                iter.next();
                return Ok(AliceToken::String(s));
            } else if c == '\\' {
                escaped = true;
            } else {
                s.push(c);
            }
            iter.next();
        }
        Err(AliceLexerErr::MissingDelimeter(
            format!("missing string delimiter '{end}'"),
            self.loc.clone(),
        ))
    }

    fn gobble_number(
        &self,
        start: char,
        iter: &mut Peekable<Chars>,
    ) -> Result<AliceToken, AliceLexerErr> {
        let mut s = String::new();
        s.push(start);
        let base = if start == '0' {
            match iter.next() {
                Some(b) if b == 'x' => 16,
                Some(b) if b == 'b' => 2,
                Some(b) if is_token_separator(&b) || b.is_whitespace() => {
                    return Ok(AliceToken::Number(0.0))
                }
                Some(b) if b == '_' || b == '.' || !b.is_digit(10) => {
                    s.push(b);
                    10
                }
                Some(b) => {
                    return Err(AliceLexerErr::NumberFormatErr(
                        format!("illegal base hint {b}"),
                        self.loc.clone(),
                    ))
                }
                None => return Ok(AliceToken::Number(0.0)),
            }
        } else {
            10
        };
        let mut had_period = start == '.';
        while let Some(c) = iter.peek() {
            match *c {
                d if d.is_digit(base) => s.push(d),
                '_' => (),
                '.' => {
                    if had_period {
                        return Err(AliceLexerErr::NumberFormatErr(
                            "multiple period in number literal!".into(),
                            self.loc.clone(),
                        ));
                    } else {
                        s.push('.');
                        had_period = true;
                    }
                }
                c if is_token_separator(&c) || c.is_whitespace() => {
                    return Ok(AliceToken::Number(self.parse_number(s, base as u32)?));
                }
                _ => {
                    return Err(AliceLexerErr::NumberFormatErr(
                        format!("unexpected symbol in number literal :'{c}'"),
                        self.loc.clone(),
                    ))
                }
            }
            iter.next();
        }
        Ok(AliceToken::Number(self.parse_number(s, base as u32)?))
    }

    fn parse_number(&self, s: String, base: u32) -> Result<f64, AliceLexerErr> {
        if base == 10 {
            s.parse()
                .map_err(|e: ParseFloatError| to_number_format_error(e, self.loc.clone()))
        } else {
            i64::from_str_radix(&s, base)
                .map(|n| n as f64)
                .map_err(|e: ParseIntError| to_number_format_error(e, self.loc.clone()))
        }
    }

    // on_add_op
    fn gobble_operator(
        &self,
        start: char,
        iter: &mut Peekable<Chars>,
    ) -> Result<AliceToken, AliceLexerErr> {
        if let Some(next) = iter.peek() {
            if start == '*' && *next == '*' {
                iter.next();
                return Ok(AliceToken::Op(AliceOp::Pow));
            }
        }
        Ok(AliceToken::Op(start.into()))
    }

    // on_add_sep
    fn gobble_separator(
        &self,
        sep: char,
        _iter: &mut Peekable<Chars>,
    ) -> Result<AliceToken, AliceLexerErr> {
        match sep {
            '(' => Ok(AliceToken::Sep(AliceSeparator::OpenP)),
            ')' => Ok(AliceToken::Sep(AliceSeparator::CloseP)),
            '{' => Ok(AliceToken::Sep(AliceSeparator::OpenB)),
            '}' => Ok(AliceToken::Sep(AliceSeparator::CloseB)),
            '[' => Ok(AliceToken::Sep(AliceSeparator::OpenS)),
            ']' => Ok(AliceToken::Sep(AliceSeparator::CloseS)),
            ',' => Ok(AliceToken::Sep(AliceSeparator::Comma)),
            '.' => Ok(AliceToken::Sep(AliceSeparator::Period)),
            ':' => Ok(AliceToken::Sep(AliceSeparator::Colon)),
            ';' => Ok(AliceToken::Sep(AliceSeparator::Semi)),
            '@' => Ok(AliceToken::Sep(AliceSeparator::At)),
            _ => Err(AliceLexerErr::UnexpectedSymbol(
                format!("unexpected separator '{sep}'"),
                self.loc.clone(),
            )),
        }
    }

    fn gobble_ident_or_keyw(
        &self,
        start: char,
        iter: &mut Peekable<Chars>,
    ) -> Result<AliceToken, AliceLexerErr> {
        let mut s: String = start.into();
        while let Some(c) = iter.peek() {
            if is_token_separator(c) || c.is_whitespace() {
                break;
            } else {
                s.push(*c);
                iter.next();
            }
        }
        Ok(AliceToken::IdentOrKeyw(s))
    }
}

fn is_token_separator(c: &char) -> bool {
    AliceSeparator::contains(c) || AliceOp::contains(c) || c == &'\'' || c == &'"'
}

trait CharCollection {
    fn all_chars() -> &'static [char];
    /// convenience method with possible future perf improvements
    fn contains(c: &char) -> bool {
        Self::all_chars().contains(c)
    }
}

impl CharCollection for AliceSeparator {
    // on_add_sep
    fn all_chars() -> &'static [char] {
        &['(', ')', '{', '}', '[', ']', ',', '.', ':', ';', '@']
    }
}

impl CharCollection for AliceOp {
    // on_add_op
    fn all_chars() -> &'static [char] {
        &['+', '-', '*', '/', '%', '=']
    }
}

impl From<char> for AliceOp {
    // on_add_op
    fn from(c: char) -> Self {
        match c {
            '+' => AliceOp::Add,
            '-' => AliceOp::Sub,
            '*' => AliceOp::Mul,
            '/' => AliceOp::Div,
            '%' => AliceOp::Mod,
            '=' => AliceOp::Eqs,
            _ => panic!("cannot convert {c} into an AliceOp"),
        }
    }
}

fn to_number_format_error<T: ToString>(e: T, loc: Loc) -> AliceLexerErr {
    AliceLexerErr::NumberFormatErr(e.to_string(), loc)
}
