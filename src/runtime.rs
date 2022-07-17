use std::collections::HashMap;
use crate::object::AliceObj;

pub const TYPE_STRING: &str = "string";
pub const TYPE_BOOL: &str = "bool";
pub const TYPE_INT: &str = "int";
pub const TYPE_FLOAT: &str = "float";
pub const TYPE_OBJECT: &str = "object";

#[derive(Debug)]
pub struct AliceStack {
    pub stack: Vec<AliceVal>,
}

#[derive(Debug)]
pub struct AliceTable {
    pub vars: HashMap<String, AliceVal>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AliceVal {
    String(Option<String>),
    Bool(Option<bool>),
    Int(Option<i64>),
    Float(Option<f64>),
    Object(Option<AliceObj>),
}

impl AliceStack {
    pub fn new(capacity: usize) -> Self {
        Self {
            stack: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, val: AliceVal) {
        self.stack.push(val);
    }

    pub fn get(&self, offset: usize) -> Option<&AliceVal> {
        self.stack.get(self.stack.len() - 1 - offset)
    }

    pub fn remove(&mut self, offset: usize) -> AliceVal {
        self.stack.remove(self.stack.len() - 1 - offset)
    }

    pub fn size(&self) -> usize {
        self.stack.len()
    }

    pub fn pop(&mut self) -> Result<AliceVal, String> {
        if let Some(val) = self.stack.pop() {
            Ok(val)
        } else {
            Err("empty stack".into())
        }
    }

    /// returns:
    /// - `Ok(Some(_))` if the stack is non empty and the head was of the same type as the given
    /// - `Ok(None)` if the stack was non emtpy but the head was of wrong type
    /// - `Err(_)` if the stack was empty
    pub fn pop_typed(&mut self, type_: &AliceVal) -> Result<Option<AliceVal>, String> {
        if let Some(val) = self.stack.get(self.stack.len() - 1) {
            if variant_eq(val, type_) {
                Ok(Some(self.stack.pop().unwrap())) // unwrapping safe due to previous check
            } else {
                Ok(None)
            }
        } else {
            Err("empty stack".into())
        }
    }
}

impl AliceTable {
    pub fn new(capacity: usize) -> Self {
        Self {
            vars: HashMap::with_capacity(capacity),
        }
    }

    pub fn put(&mut self, key: String, val: AliceVal) -> Option<AliceVal> {
        self.vars.insert(key, val)
    }

    pub fn get(&mut self, key: &String) -> Option<&AliceVal> {
        self.vars.get(key)
    }

    /// returns:
    /// - `Ok(Some(_))` if a binding with the given key exists and the type is correct
    /// - `Ok(None)` if a binding with the given key exists but the type is incorrect
    /// - `Err(_)` if a binding with the given key doesn't exist
    pub fn get_typed(
        &mut self,
        key: &String,
        type_: &AliceVal,
    ) -> Result<Option<&AliceVal>, String> {
        if let Some(val) = self.vars.get(key) {
            if variant_eq(val, type_) {
                Ok(Some(val))
            } else {
                Ok(None)
            }
        } else {
            Err(format!("unknown variable binding '{key}'"))
        }
    }

    /// removes the given binding, if existing, from the table,
    /// allowing ownership over it to be passed out
    pub fn take(&mut self, key: &String) -> Option<AliceVal> {
        self.vars.remove(key)
    }

    /// returns:
    /// - `Ok(Some(_))` if a binding with the given key existed and the type was correct
    /// - `Ok(None)` if a binding with the given key existed but the type was incorrect
    /// - `Err(_)` if a binding with the given key didn't exist
    pub fn take_type(
        &mut self,
        key: &String,
        type_: &AliceVal,
    ) -> Result<Option<AliceVal>, String> {
        if let Some(val) = self.vars.remove(key) {
            if variant_eq(&val, type_) {
                Ok(Some(val))
            } else {
                Ok(None)
            }
        } else {
            Err(format!("unknown variable binding '{key}'"))
        }
    }
}

/// helper function that checks if the given two `AliceVal`s have the same type
/// (e. g. rust enum variant).
fn variant_eq(a: &AliceVal, b: &AliceVal) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

/// contains convenience functions like
/// ```
/// AliceVal::bool()
/// // short for
/// AliceVal::Bool(None)
/// ```
impl AliceVal {
    pub fn for_type_name(s: &String) -> Result<Self, String> {
        match s.as_str() {
            TYPE_STRING => Ok(Self::string()),
            TYPE_BOOL => Ok(Self::bool()),
            TYPE_INT => Ok(Self::int()),
            TYPE_FLOAT => Ok(Self::float()),
            _ => Err(format!("unknown type name {s}")),
        }
    }

    pub fn type_name(&self) -> String {
        match self {
            AliceVal::String(_) => TYPE_STRING.into(),
            AliceVal::Bool(_) => TYPE_BOOL.into(),
            AliceVal::Int(_) => TYPE_INT.into(),
            AliceVal::Float(_) => TYPE_FLOAT.into(),
            AliceVal::Object(Some(o)) => o.type_name.clone(),
            AliceVal::Object(None) => TYPE_OBJECT.into(),
        }
    }

    pub fn unchecked_string(&self) -> String {
        match self {
            AliceVal::String(s) => s.clone().as_ref().unwrap().clone(),
            _ => panic!("self is not of type String"),
        }
    }

    pub fn unchecked_bool(&self) -> bool {
        match self {
            AliceVal::Bool(b) => *b.as_ref().unwrap(),
            _ => panic!("self is not of type bool"),
        }
    }

    pub fn unchecked_int(&self) -> i64 {
        match self {
            AliceVal::Int(i) => *i.as_ref().unwrap(),
            _ => panic!("self is not of type int"),
        }
    }

    pub fn unchecked_float(&self) -> f64 {
        match self {
            AliceVal::Float(f) => *f.as_ref().unwrap(),
            _ => panic!("self is not of type float"),
        }
    }

    pub fn string() -> Self {
        Self::String(None)
    }

    pub fn bool() -> Self {
        Self::Bool(None)
    }

    pub fn int() -> Self {
        Self::Int(None)
    }

    pub fn float() -> Self {
        Self::Float(None)
    }
}

impl std::fmt::Display for AliceVal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::String(val) => write!(f, "{}", val.as_ref().expect("cannot print null binding")),
            Self::Bool(val) => write!(f, "{}", val.as_ref().expect("cannot print null binding")),
            Self::Int(val) => write!(f, "{}", val.as_ref().expect("cannot print null binding")),
            Self::Float(val) => write!(f, "{}", val.as_ref().expect("cannot print null binding")),
            Self::Object(o) => todo!(),
        }
    }
}
