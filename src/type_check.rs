use crate::runtime::AliceVal;
use crate::statement::Statement;

use std::collections::HashMap;
use std::rc::Rc;

pub const STRING: u32 = 1;
pub const BOOL: u32 = 2;
pub const INT: u32 = 4;
pub const FLOAT: u32 = 8;
// an object ist represented by
pub const OBJECT: u32 = 16;
pub const OBJECT_SIG_MASK: u32 = 0b11111111111111111111111111100000;
pub const ANY: u32 = 0b11111;

#[derive(Debug, Clone)]
pub struct TypeStack {
    pub vals: Vec<u32>,
    pub vars: HashMap<String, u32>,
    pub funs: HashMap<String, (StackPattern, u32)>,
}

#[derive(Debug, Clone)]
pub struct StackPattern(pub Vec<u32>);

pub fn is_object(bits: &u32) -> bool {
    bits > &15
}

pub fn check(statements: &Vec<Box<dyn Statement>>) -> Result<(), TypeCheckError> {
    let mut stack = TypeStack::new();
    for s in statements {
        //println!("type stack before: {:?}", stack);
        s.in_pattern().type_check(&mut stack)?;
        s.custom_type_check(&mut stack)?;
        s.out_pattern().push(&mut stack);
        //println!("type stack after: {:?}\n", stack);
    }
    if stack.vals.is_empty() {
        Ok(())
    } else {
        Err(TypeCheckError(format!(
            "{} excess values on the stack!",
            stack.vals.len()
        )))
    }
}

pub fn check_interactive(
    stack: &mut TypeStack,
    statements: &Vec<Box<dyn Statement>>,
) -> Result<(), TypeCheckError> {
    for s in statements {
        s.in_pattern().type_check(stack)?;
        s.custom_type_check(stack)?;
        s.out_pattern().push(stack);
    }
    Ok(())
}

pub fn check_rc(
    stack: &mut TypeStack,
    statements: &Vec<Rc<dyn Statement>>,
) -> Result<(), TypeCheckError> {
    for s in statements {
        s.in_pattern().type_check(stack)?;
        s.custom_type_check(stack)?;
        s.out_pattern().push(stack);
    }
    Ok(())
}

impl StackPattern {
    pub fn single(ty: u32) -> Self {
        Self(vec![ty])
    }

    pub fn any(n: usize) -> Self {
        let mut vec = Vec::with_capacity(n);
        for _i in 0..n {
            vec.push(ANY)
        }
        Self(vec)
    }

    pub fn type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
        for t in &self.0 {
            if let Some(actual) = stack.pop() {
                if actual & t != actual {
                    return Err(TypeCheckError(format!(
                        "wrong type on stack when this executes"
                    ))); // todo descriptive error msg
                }
            } else {
                return Err(TypeCheckError(
                    "too few values on stack when this executes".into(),
                ));
            }
        }
        Ok(())
    }

    pub fn push(&self, stack: &mut TypeStack) {
        for t in &self.0 {
            stack.vals.push(*t);
        }
    }
}

impl TypeStack {
    pub fn new() -> Self {
        Self {
            vals: Vec::new(),
            vars: HashMap::new(),
            funs: HashMap::new(),
        }
    }
    pub fn pop(&mut self) -> Option<u32> {
        self.vals.pop()
    }

    pub fn required_size(&self, size: usize) -> Result<(), TypeCheckError> {
        if self.vals.len() < size {
            Err(TypeCheckError(
                "too few elements on stack when this executes".into(),
            ))
        } else {
            Ok(())
        }
    }
}

impl PartialEq<&Self> for TypeStack {
    fn eq(&self, other: &&Self) -> bool {
        self.vals == other.vals
    }
}

pub struct TypeCheckError(pub String);

impl From<TypeCheckError> for String {
    fn from(err: TypeCheckError) -> String {
        err.0.clone()
    }
}

impl TypeCheckError {
    pub fn prefix(&self, mut prefix: String) -> Self {
        prefix.push_str(&self.0);
        Self(prefix)
    }
}

pub fn type_bit(val: &AliceVal) -> u32 {
    match val {
        AliceVal::String(_) => STRING,
        AliceVal::Bool(_) => BOOL,
        AliceVal::Int(_) => INT,
        AliceVal::Float(_) => FLOAT,
        AliceVal::Object(Some(o)) => o.type_hash,
        AliceVal::Object(None) => OBJECT,
        AliceVal::Function(_) => panic!("function should not be allowed on stack"),
    }
}

pub fn type_bit_any_allowed(name: &String) -> Result<u32, String> {
    if name == "any" {
        Ok(ANY)
    } else {
        Ok(type_bit(&AliceVal::for_type_name(name)?))
    }
}
