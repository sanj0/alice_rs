use std::collections::HashMap;

pub const TYPE_STRING: &str = "string";

#[derive(Debug)]
pub struct AliceStack {
    pub stack: Vec<AliceVal>,
}

#[derive(Debug)]
pub struct AliceTable {
    pub vars: HashMap<String, AliceVal>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AliceVal {
    Bool(Option<bool>),
    String(Option<String>),
    Byte(Option<i8>),
    Short(Option<i16>),
    Int(Option<i32>),
    Long(Option<i64>),
    Float32(Option<f32>),
    Float64(Option<f64>),
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
            _ => Err(format!("unknown type {s}")),
        }
    }

    pub fn bool() -> Self {
        Self::Bool(None)
    }

    pub fn string() -> Self {
        Self::String(None)
    }

    pub fn byte() -> Self {
        Self::Byte(None)
    }

    pub fn short() -> Self {
        Self::Short(None)
    }

    pub fn int() -> Self {
        Self::Int(None)
    }

    pub fn long() -> Self {
        Self::Long(None)
    }

    pub fn float32() -> Self {
        Self::Float32(None)
    }

    pub fn float64() -> Self {
        Self::Float64(None)
    }
}

impl std::fmt::Display for AliceVal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Bool(val) => write!(f, "{}", val.as_ref().expect("cannot print null binding")),
            Self::String(val) => write!(f, "{}", val.as_ref().expect("cannot print null binding")),
            Self::Byte(val) => write!(f, "{}", val.as_ref().expect("cannot print null binding")),
            Self::Short(val) => write!(f, "{}", val.as_ref().expect("cannot print null binding")),
            Self::Int(val) => write!(f, "{}", val.as_ref().expect("cannot print null binding")),
            Self::Long(val) => write!(f, "{}", val.as_ref().expect("cannot print null binding")),
            Self::Float32(val) => write!(f, "{}", val.as_ref().expect("cannot print null binding")),
            Self::Float64(val) => write!(f, "{}", val.as_ref().expect("cannot print null binding")),
        }
    }
}
