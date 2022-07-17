use std::collections::HashMap;
use crate::statement::Statement;
use crate::runtime::*;
use crate::type_check::*;

use std::rc::Rc;
use std::fmt;

#[derive(Debug, Clone)]
pub struct AliceObj {
    pub type_name: String,
    /// hash of the type name and type signature, truncated to the 27 most significant bits
    /// using type_check::OBJECT_SIG_MASK
    pub type_hash: u32,
    pub members: HashMap<String, AliceVal>,
    pub functions: HashMap<String, Vec<AliceFun>>,
}

impl PartialEq for AliceObj {
    fn eq(&self, other: &Self) -> bool {
        self.type_hash == other.type_hash
    }
}

#[derive(Clone)]
pub struct AliceFun {
    pub args: StackPattern,
    /// possible values defined in type_check.rs
    pub return_type: u32,
    pub body: Vec<Rc<dyn Statement>>,
}

impl AliceFun {
    pub fn new(args: StackPattern, return_type: u32, body: Vec<Rc<dyn Statement>>) -> Self {
        Self {
            args,
            return_type,
            body
        }
    }
}

impl Statement for AliceFun {
    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        // todo! create new stack frame on table
        for s in &self.body {
            s.execute(stack, _table)?;
        }
        Ok(())
    }
}

impl PartialEq for AliceFun {
    fn eq(&self, other: &Self) -> bool {
        // functions cannot be placed on the stack, i. e. this is never needed
        panic!("this should never happen... wtf?")
    }
}

impl fmt::Debug for AliceFun {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AliceFun")
            .field("args", &self.args)
            .field("return_type", &self.return_type)
            .finish()
    }
}
